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

use itertools::Itertools;
use crate::compiler::structure::{Field, FieldView, FixedField, FixedFieldType, Structure};
use crate::compiler::util::TypePathMap;
use crate::gen::template::{Scope, Template};
use crate::model::protocol::Endianness;

pub trait Utilities {
    fn get_field_type(field_type: FixedFieldType) -> &'static str;
    fn get_function_name(field: &FixedField) -> &'static str;
    fn get_function_name_mut(field: &FixedField) -> &'static str;
    fn get_bit_codec_inline(endianness: Endianness) -> &'static str;
    fn get_byte_codec_inline(endianness: Endianness) -> &'static str;
    fn get_byte_codec(endianness: Endianness) -> &'static str;
}

fn gen_field_getter<U: Utilities>(field: &Field, template: &Template, type_path_by_name: &TypePathMap) -> String {
    let mut scope = template.scope();
    scope.var_d("start", field.loc().byte_offset)
        .var_d("end", field.loc().byte_offset + field.loc().byte_size)
        .var("name", field.name());
    match field {
        Field::Fixed(v) => {
            let raw_field_type = v.loc.get_unsigned_integer_type();
            let raw_field_type = U::get_field_type(raw_field_type);
            let function_name = U::get_function_name(v);
            scope.var("raw_type", raw_field_type).var("function_name", function_name);
            if v.loc.bit_size % 8 != 0 {
                scope.var("codec", U::get_bit_codec_inline(v.endianness))
                    .var_d("bit_offset", v.loc.bit_offset)
                    .var_d("bit_size", v.loc.bit_size)
                    .render_to_var("getters.fixed", &["bit"], "fragment").unwrap();
            } else {
                scope.var("codec", U::get_byte_codec_inline(v.endianness))
                    .render_to_var("getters.fixed", &["byte"], "fragment").unwrap();
            }
            let mut code = scope.render("getters", &["fixed"]).unwrap();
            code += &gen_field_view_getter::<U>(v, scope, type_path_by_name);
            code
        }
        Field::Array(v) => scope.var("raw_type", U::get_field_type(v.ty))
            .var("codec", U::get_byte_codec(v.endianness)).var_d("bit_size", v.item_bit_size())
            .render("getters", &["array"]).unwrap(),
        Field::Struct(v) => scope.var("type_name", type_path_by_name.get(&v.r.name))
            .render("getters", &["struct"]).unwrap()
    }
}

fn gen_field_setter<U: Utilities>(field: &Field, template: &Template, type_path_by_name: &TypePathMap) -> String {
    let mut scope = template.scope();
    scope.var_d("start", field.loc().byte_offset)
        .var_d("end", field.loc().byte_offset + field.loc().byte_size)
        .var("name", field.name());
    match field {
        Field::Fixed(v) => {
            let raw_field_type = v.loc.get_unsigned_integer_type();
            let raw_field_type = U::get_field_type(raw_field_type);
            let function_name = U::get_function_name_mut(v);
            scope.var("raw_type", raw_field_type).var("function_name", function_name);
            if v.loc.bit_size % 8 != 0 {
                scope.var("codec", U::get_bit_codec_inline(v.endianness))
                    .var_d("bit_offset", v.loc.bit_offset)
                    .var_d("bit_size", v.loc.bit_size)
                    .render_to_var("setters.fixed", &["bit"], "fragment").unwrap();
            } else {
                scope.var("codec", U::get_byte_codec_inline(v.endianness))
                    .render_to_var("setters.fixed", &["byte"], "fragment").unwrap();
            }
            let mut code = scope.render("setters", &["fixed"]).unwrap();
            code += &gen_field_view_setter::<U>(v, scope, type_path_by_name);
            code
        }
        Field::Array(v) => scope.var("raw_type", U::get_field_type(v.ty))
            .var("codec", U::get_byte_codec(v.endianness)).var_d("bit_size", v.item_bit_size())
            .render("setters", &["array"]).unwrap(),
        Field::Struct(v) => scope.var("type_name", type_path_by_name.get(&v.r.name))
            .render("setters", &["struct"]).unwrap()
    }
}

fn gen_field_view_getter<U: Utilities>(field: &FixedField, mut scope: Scope, type_path_by_name: &TypePathMap) -> String {
    match &field.view {
        FieldView::Float { a, b, .. } => scope.var("view_type", U::get_field_type(field.ty))
            .var("a", format!("{:?}", a)).var("b", format!("{:?}", b))
            .render("getters", &["view_float"]).unwrap(),
        //Rust absolutely wants to allocate as randomly the lifetime does not match
        FieldView::Enum(e) => scope.var("view_type", String::from(type_path_by_name.get(&e.name)))
            .var_d("enum_largest", e.largest).render("getters", &["view_enum"]).unwrap(),
        FieldView::Transmute => {
            let field_type = U::get_field_type(field.ty);
            scope.var("view_type", field_type);
            if field.ty == FixedFieldType::Bool {
                scope.render_to_var("getters.view_transmute", &["bool"], "fragment").unwrap();
            } else {
                scope.render_to_var("getters.view_transmute", &["other"], "fragment").unwrap();
            }
            scope.render("getters", &["view_transmute"]).unwrap()
        },
        FieldView::SignedCast(max_positive) => scope.var("view_type", U::get_field_type(field.ty))
            .var_d("max_positive", max_positive).render("getters", &["view_signed"]).unwrap(),
        FieldView::None => scope.var("view_type", U::get_field_type(field.ty)).render("getters", &["view_none"]).unwrap()
    }
}

fn gen_field_view_setter<U: Utilities>(field: &FixedField, mut scope: Scope, type_path_by_name: &TypePathMap) -> String {
    match &field.view {
        FieldView::Float { a_inv, b_inv, .. } => scope.var("view_type", U::get_field_type(field.ty))
            .var("a_inv", format!("{:?}", a_inv)).var("b_inv", format!("{:?}", b_inv))
            .render("setters", &["view_float"]).unwrap(),
        //Rust absolutely wants to allocate as randomly the lifetime does not match
        FieldView::Enum(e) => scope.var("view_type", String::from(type_path_by_name.get(&e.name)))
            .var_d("enum_largest", e.largest).render("setters", &["view_enum"]).unwrap(),
        FieldView::Transmute | FieldView::SignedCast { .. } => {
            let field_type = U::get_field_type(field.ty);
            scope.var("view_type", field_type);
            if field.ty == FixedFieldType::Bool {
                scope.render_to_var("setters.view_transmute", &["bool"], "fragment").unwrap();
            } else {
                scope.render_to_var("setters.view_transmute", &["other"], "fragment").unwrap();
            }
            scope.render("setters", &["view_transmute"]).unwrap()
        },
        FieldView::None => scope.var("view_type", U::get_field_type(field.ty)).render("setters", &["view_none"]).unwrap()
    }
}

fn gen_structure_getters<U: Utilities>(s: &Structure, template: &Template, type_path_by_name: &TypePathMap) -> String {
    let mut scope = template.scope();
    let fields = s.fields.iter().map(|v| gen_field_getter::<U>(v, template, type_path_by_name)).join("");
    scope.var("fields", fields).render("", &["getters"]).unwrap()
}

fn gen_structure_setters<U: Utilities>(s: &Structure, template: &Template, type_path_by_name: &TypePathMap) -> String {
    let mut scope = template.scope();
    let fields = s.fields.iter().map(|v| gen_field_setter::<U>(v, template, type_path_by_name)).join("");
    scope.var("fields", fields).render("", &["setters"]).unwrap()
}

pub struct Templates<'a> {
    pub field_template: &'a [u8],
    pub template: &'a [u8]
}

pub fn generate<U: Utilities>(templates: Templates, s: &Structure, type_path_by_name: &TypePathMap) -> String {
    let mut template = Template::compile(templates.template).unwrap();
    let mut field_template = Template::compile(templates.field_template).unwrap();
    field_template.var("struct_name", &s.name);
    template.var("name", &s.name).var_d("byte_size", s.byte_size)
        .var("name_upper", s.name.to_ascii_uppercase());
    let mut code = template.render("", &["decl", "new", "fixed_size", "write_to", "from_slice"]).unwrap();
    code += &gen_structure_getters::<U>(s, &field_template, type_path_by_name);
    code += &gen_structure_setters::<U>(s, &field_template, type_path_by_name);
    code
}
