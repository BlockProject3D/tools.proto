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

use crate::compiler::message::Referenced;
use crate::compiler::union::{DiscriminantField, Union};
use crate::compiler::util::TypeMapper;
use crate::gen::base::map::TypePathMapper;
use crate::gen::template::Template;
use itertools::Itertools;
use crate::gen::hook::TemplateHooks;

pub trait Utilities: crate::gen::base::structure::Utilities {
    fn gen_discriminant_path(discriminant: &DiscriminantField) -> String;
    fn gen_discriminant_path_mut(discriminant: &DiscriminantField) -> String;
    fn get_generics(u: &Union) -> &str;
}

fn gen_union_from_slice_impl<T: TypeMapper>(
    u: &Union,
    template: &Template,
    type_path_map: &TypePathMapper<T>,
    function: &str
) -> String {
    let cases = u
        .cases
        .iter()
        .map(|case| {
            let mut scope = template.scope();
            scope.var("name", &case.name).var_d("case", case.case);
            match &case.item_type {
                None => scope.render(function, &["none.case"]).unwrap(),
                Some(item_type) => scope
                    .var("type_name", type_path_map.get(item_type))
                    .render(function, &["content.case"])
                    .unwrap(),
            }
        })
        .join("");
    let mut scope = template.scope();
    scope.var("cases", cases);
    if u.has_content() {
        scope.render_to_var(function, &["content"], "fragment").unwrap();
    } else {
        scope.render_to_var(function, &["none"], "fragment").unwrap();
    }
    scope.render("", &[function]).unwrap()
}

fn gen_union_write_to_impl<T: TypeMapper>(
    u: &Union,
    template: &Template,
    type_path_map: &TypePathMapper<T>,
    function: &str
) -> String {
    let mut scope = template.scope();
    if u.has_content() {
        let cases = u
            .cases
            .iter()
            .filter_map(|case| {
                let mut scope = template.scope();
                scope.var("name", &case.name).var_d("case", case.case);
                case.item_type.as_ref().map(|item_type| {
                    scope
                        .var("type_name", type_path_map.get(item_type))
                        .render(function, &["content.case"])
                        .unwrap()
                })
            })
            .join("");
        scope.var("cases", cases).render_to_var(function, &["content"], "fragment").unwrap();
    } else {
        scope.render_to_var(function, &["none"], "fragment").unwrap();
    }
    scope.render("", &[function]).unwrap()
}

fn gen_union_set_discriminant(u: &Union, template: &Template) -> String {
    let cases = u
        .cases
        .iter()
        .map(|case| {
            let mut scope = template.scope();
            scope.var("name", &case.name).var_d("case", case.case);
            match &case.item_type {
                Some(_) => scope.render("setter", &["ref"]).unwrap(),
                None => scope.render("setter", &["none"]).unwrap(),
            }
        })
        .join("");
    template.scope().var("cases", cases).render("", &["setter"]).unwrap()
}

fn gen_union_as_getters<T: TypeMapper>(
    u: &Union,
    template: &Template,
    type_path_map: &TypePathMapper<T>,
) -> String {
    let cases = u
        .cases
        .iter()
        .map(|case| {
            let mut scope = template.scope();
            scope.var("name", &case.name);
            match &case.item_type {
                Some(Referenced::Struct(v)) => {
                    scope.var("type_name", type_path_map.get(v)).render("getters", &["struct"]).unwrap()
                }
                Some(Referenced::Message(v)) => {
                    scope.var("type_name", type_path_map.get(v)).render("getters", &["message"]).unwrap()
                }
                None => scope.render("getters", &["none"]).unwrap(),
            }
        })
        .join("");
    template.scope().var("cases", cases).render("", &["getters"]).unwrap()
}

fn gen_decl<T: TypeMapper>(u: &Union, template: &Template, type_path_map: &TypePathMapper<T>, function: &str) -> String {
    let cases = u
        .cases
        .iter()
        .map(|case| match &case.item_type {
            None => template.scope().var("name", &case.name).render(function, &["none"]).unwrap(),
            Some(Referenced::Struct(v)) => template
                .scope()
                .var("name", &case.name)
                .var("type_name", type_path_map.get(v))
                .render(function, &["struct"])
                .unwrap(),
            Some(Referenced::Message(v)) => template
                .scope()
                .var("name", &case.name)
                .var("type_name", type_path_map.get(v))
                .render(function, &["message"])
                .unwrap(),
        })
        .join("");
    template.scope().var("cases", cases).render("", &[function]).unwrap()
}

pub fn generate<'variable, U: Utilities, T: TypeMapper>(
    mut template: Template<'_, 'variable>,
    u: &'variable Union,
    type_path_map: &'variable TypePathMapper<T>,
    hooks: &TemplateHooks
) -> String {
    let generics = U::get_generics(u);
    template
        .var("discriminant_raw_type", U::get_field_type(u.discriminant.get_leaf().ty))
        .var("union_name", &u.name)
        .var("generics", generics)
        .var("discriminant_path_mut", U::gen_discriminant_path_mut(&u.discriminant))
        .var("discriminant_path", U::gen_discriminant_path(&u.discriminant))
        .var("discriminant_type", type_path_map.get(&u.discriminant.root));
    let mut code = gen_decl(u, &template, type_path_map, "decl");
    for func in hooks.get_functions("decl") {
        code += &gen_decl(u, &template, type_path_map, func);
    }
    code += &gen_union_from_slice_impl::<T>(u, &template, type_path_map, "from_slice");
    for func in hooks.get_functions("from_slice") {
        code += &gen_union_from_slice_impl::<T>(u, &template, type_path_map, func);
    }
    code += &gen_union_write_to_impl::<T>(u, &template, type_path_map, "write_to");
    for func in hooks.get_functions("write_to") {
        code += &gen_union_write_to_impl::<T>(u, &template, type_path_map, func);
    }
    code += &gen_union_set_discriminant(u, &template);
    code += &gen_union_as_getters(u, &template, type_path_map);
    code
}
