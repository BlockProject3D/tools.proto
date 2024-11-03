// Copyright (c) 2024, BlockProject 3D
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of BlockProject 3D nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use crate::compiler::structure::{Field, FieldRaw, FieldType, FieldView, FixedField, FixedFieldType, Structure};
use crate::compiler::util::types::TypeMapper;
use crate::gen::base::map::TypePathMapper;
use crate::gen::template::hook::{Render, TemplateHooks};
use crate::gen::template::{Scope, Template};
use crate::model::protocol::{Description, Endianness};
use itertools::Itertools;
use std::borrow::Cow;

pub trait Utilities {
    fn get_field_type(field_type: FixedFieldType) -> &'static str;
    fn get_fragment_name(field: &Field) -> &'static str;
    fn get_fragment_name_mut(field: &Field) -> &'static str;
    fn get_bit_codec_inline(endianness: Endianness) -> &'static str;
    fn get_byte_codec_inline(endianness: Endianness) -> &'static str;
    fn get_byte_codec(endianness: Endianness) -> &'static str;
    fn gen_description(desc: &Description) -> Cow<str> {
        match desc {
            Description::Single(v) => Cow::Borrowed(v),
            Description::Multi(v) => Cow::Owned(v.join(" ")),
        }
    }
}

enum Mode {
    Getter,
    Setter,
}

impl Mode {
    pub fn get_path<'a>(&self, getter: &'a str, setter: &'a str) -> &'a str {
        match self {
            Mode::Getter => getter,
            Mode::Setter => setter,
        }
    }
}

fn gen_structure_field_prologue<'a, 'fragment, 'variable: 'fragment, U: Utilities>(
    template: &'variable Template<'fragment, 'variable>,
    field: &'variable Field,
) -> Scope<'a, 'fragment, 'variable> {
    let mut scope = template.scope();
    scope
        .var_d("start", field.loc.byte_offset)
        .var_d("end", field.loc.byte_offset + field.loc.byte_size)
        .var("name", &field.name)
        .var(
            "description",
            field.description.as_ref().map(U::gen_description).unwrap_or("".into()),
        )
        .var_d("info", field);
    scope
}

fn gen_structure_field<U: Utilities, G: FnMut(Mode, &Field, &FixedField, Scope) -> String>(
    mode: Mode,
    field: &Field,
    template: &Template,
    field_generator: &mut G,
) -> Option<String> {
    let mut scope = gen_structure_field_prologue::<U>(template, field);
    match &field.ty {
        FieldType::Fixed(v) => {
            let bits_type = U::get_field_type(v.bits_type);
            let raw_type = U::get_field_type(v.raw_type);
            scope
                .var("bits_type", bits_type)
                .var("raw_type", raw_type)
                .var_d("bit_offset", field.loc.bit_offset)
                .var_d("bit_size", field.loc.bit_size);
            Some(field_generator(mode, field, v, scope))
        }
        _ => None,
    }
}

fn gen_structure<
    'variable,
    U: Utilities,
    T: TypeMapper,
    G: FnMut(Mode, &Field, &FixedField, Scope) -> String,
>(
    s: &'variable Structure,
    mut template: Template<'_, 'variable>,
    mut field_generator: G,
) -> String {
    template.var("struct_name", &s.name).var(
        "struct_description",
        s.description.as_ref().map(U::gen_description).unwrap_or("".into()),
    );
    let getters = s
        .fields
        .iter()
        .filter_map(|v| gen_structure_field::<U, G>(Mode::Getter, v, &template, &mut field_generator))
        .join("");
    let setters = s
        .fields
        .iter()
        .filter_map(|v| gen_structure_field::<U, G>(Mode::Setter, v, &template, &mut field_generator))
        .join("");
    let mut code = template.render("", &["decl"]).unwrap();
    if !getters.is_empty() {
        code += &template.var("fields", getters).render("", &["getters"]).unwrap();
    }
    if !setters.is_empty() {
        code += &template.var("fields", setters).render("", &["setters"]).unwrap();
    }
    code
}

fn gen_field_raw<'variable>(
    mode: Mode,
    _: &'variable Field,
    fixed: &'variable FixedField,
    mut scope: Scope<'_, '_, 'variable>,
) -> String {
    let path = mode.get_path("getters.field", "setters.field");
    match &fixed.raw {
        FieldRaw::Transmute => {
            if fixed.raw_type == FixedFieldType::Bool {
                scope.render_to_var(path, &["transmute_bool"], "fragment").unwrap()
            } else {
                scope.render_to_var(path, &["transmute_other"], "fragment").unwrap()
            }
        }
        FieldRaw::SignedCast(max_positive) => {
            scope.var_d("max_positive", max_positive).render_to_var(path, &["signed"], "fragment").unwrap()
        }
        FieldRaw::None => scope.render_to_var(path, &["none"], "fragment").unwrap(),
    };
    scope.render(mode.get_path("getters", "setters"), &["field"]).unwrap()
}

fn gen_field_bin<'variable, U: Utilities>(
    mode: Mode,
    field: &'variable Field,
    fixed: &'variable FixedField,
    mut scope: Scope<'_, '_, 'variable>,
) -> String {
    let fragment_name = U::get_fragment_name(field);
    if field.loc.bit_size % 8 != 0 {
        let path = mode.get_path("getters.field.bit", "setters.field.bit");
        scope
            .var("codec", U::get_bit_codec_inline(fixed.endianness))
            .render_to_var(path, &[fragment_name], "fragment")
            .unwrap();
    } else {
        let path = mode.get_path("getters.field.byte", "setters.field.byte");
        scope
            .var("codec", U::get_byte_codec_inline(fixed.endianness))
            .render_to_var(path, &[fragment_name], "fragment")
            .unwrap();
    }
    scope.render(mode.get_path("getters", "setters"), &["field"]).unwrap()
}

fn gen_field_getter<U: Utilities, T: TypeMapper>(
    field: &Field,
    template: &Template,
    type_path_map: &TypePathMapper<T>,
) -> String {
    let mut scope = gen_structure_field_prologue::<U>(template, field);
    match &field.ty {
        FieldType::Fixed(v) => gen_field_view_getter::<U, T>(v, &scope, type_path_map),
        FieldType::Array(v) => scope
            .var("raw_type", U::get_field_type(v.ty))
            .var("codec", U::get_byte_codec(v.endianness))
            .var_d("bit_size", v.item_bit_size)
            .render("getters", &["array"])
            .unwrap(),
        FieldType::Struct(v) => scope.var("type_name", type_path_map.get(v)).render("getters", &["struct"]).unwrap(),
    }
}

fn gen_field_setter<U: Utilities, T: TypeMapper>(
    field: &Field,
    template: &Template,
    type_path_map: &TypePathMapper<T>,
) -> String {
    let mut scope = gen_structure_field_prologue::<U>(template, field);
    match &field.ty {
        FieldType::Fixed(v) => gen_field_view_setter::<U, T>(v, &scope, type_path_map),
        FieldType::Array(v) => scope
            .var("raw_type", U::get_field_type(v.ty))
            .var("codec", U::get_byte_codec(v.endianness))
            .var_d("bit_size", v.item_bit_size)
            .render("setters", &["array"])
            .unwrap(),
        FieldType::Struct(v) => scope.var("type_name", type_path_map.get(v)).render("setters", &["struct"]).unwrap(),
    }
}

fn gen_field_view_getter<U: Utilities, T: TypeMapper>(
    field: &FixedField,
    scope: &Scope,
    type_path_map: &TypePathMapper<T>,
) -> String {
    let mut scope = scope.clone();
    scope.var("raw_type", U::get_field_type(field.raw_type));
    match &field.view {
        FieldView::Float { a, b, .. } => scope
            .var("view_type", U::get_field_type(field.view_type))
            .var("a", format!("{:?}", a))
            .var("b", format!("{:?}", b))
            .render("getters", &["view_float"])
            .unwrap(),
        FieldView::Enum(r) => scope
            .var("view_type", type_path_map.get(r))
            .var("repr_type", U::get_field_type(r.repr_type))
            .render("getters", &["view_enum"])
            .unwrap(),
        FieldView::None => scope
            .var("view_type", U::get_field_type(field.view_type))
            .render("getters", &["view_none"])
            .unwrap(),
    }
}

fn gen_field_view_setter<U: Utilities, T: TypeMapper>(
    field: &FixedField,
    scope: &Scope,
    type_path_map: &TypePathMapper<T>,
) -> String {
    let mut scope = scope.clone();
    scope.var("raw_type", U::get_field_type(field.raw_type));
    match &field.view {
        FieldView::Float { a_inv, b_inv, .. } => scope
            .var("view_type", U::get_field_type(field.view_type))
            .var("a_inv", format!("{:?}", a_inv))
            .var("b_inv", format!("{:?}", b_inv))
            .render("setters", &["view_float"])
            .unwrap(),
        FieldView::Enum(r) => scope
            .var("view_type", type_path_map.get(r))
            .var("repr_type", U::get_field_type(r.repr_type))
            .render("setters", &["view_enum"])
            .unwrap(),
        FieldView::None => scope
            .var("view_type", U::get_field_type(field.view_type))
            .render("setters", &["view_none"])
            .unwrap(),
    }
}

fn gen_structure_getters<U: Utilities, T: TypeMapper>(
    s: &Structure,
    template: &Template,
    type_path_map: &TypePathMapper<T>,
) -> String {
    let mut scope = template.scope();
    let fields = s.fields.iter().map(|v| gen_field_getter::<U, T>(v, template, type_path_map)).join("");
    scope.var("fields", fields).render("", &["getters"]).unwrap()
}

fn gen_structure_setters<U: Utilities, T: TypeMapper>(
    s: &Structure,
    template: &Template,
    type_path_map: &TypePathMapper<T>,
) -> String {
    let mut scope = template.scope();
    let fields = s.fields.iter().map(|v| gen_field_setter::<U, T>(v, template, type_path_map)).join("");
    scope.var("fields", fields).render("", &["setters"]).unwrap()
}

pub struct Templates<'fragment, 'variable> {
    pub field_template: Template<'fragment, 'variable>,
    pub template: Template<'fragment, 'variable>,
    pub bits_template: Template<'fragment, 'variable>,
    pub raw_template: Template<'fragment, 'variable>,
}

pub fn generate<'variable, U: Utilities, T: TypeMapper>(
    templates: Templates<'_, 'variable>,
    s: &'variable Structure,
    type_path_map: &TypePathMapper<T>,
    hooks: &TemplateHooks,
) -> String {
    let mut template = templates.template;
    let mut field_template = templates.field_template;
    field_template.var("struct_name", &s.name).var(
        "struct_description",
        s.description.as_ref().map(U::gen_description).unwrap_or("".into()),
    );
    template.var("name", &s.name).var_d("byte_size", s.byte_size).var(
        "struct_description",
        s.description.as_ref().map(U::gen_description).unwrap_or("".into()),
    );
    let mut code = template.render("", &["decl", "new", "fixed_size", "write_to", "from_bytes"]).unwrap();
    for frag in hooks.get_fragments("ext") {
        code += &template.render_frag(frag).unwrap();
    }
    code += &gen_structure_getters::<U, T>(s, &field_template, type_path_map);
    code += &gen_structure_setters::<U, T>(s, &field_template, type_path_map);
    code += &gen_structure::<U, T, _>(s, templates.bits_template, |mode, field, fixed, scope| {
        gen_field_bin::<U>(mode, field, fixed, scope)
    });
    code += &gen_structure::<U, T, _>(s, templates.raw_template, |mode, field, fixed, scope| {
        gen_field_raw(mode, field, fixed, scope)
    });
    code
}
