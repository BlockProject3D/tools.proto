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
use crate::compiler::message::Referenced;
use crate::compiler::union::Union;
use crate::compiler::util::TypePathMap;
use crate::gen::template::Template;

const TEMPLATE: &[u8] = include_bytes!("./union.template");

fn get_generics(u: &Union) -> &str {
    if u.cases.iter().any(|v| v.item_type.is_some()) {
        "<'a>"
    } else {
        ""
    }
}

fn get_discriminant_path(u: &Union) -> String {
    u.discriminant.iter().map(|(f, is_leaf)| if is_leaf {
        format!("get_raw_{}()", f.name())
    } else {
        format!("get_{}()", f.name())
    }).join(".")
}

fn get_discriminant_path_mut(u: &Union) -> String {
    u.discriminant.iter().map(|(f, is_leaf)| if is_leaf {
        format!("set_raw_{}", f.name())
    } else {
        format!("get_{}_mut()", f.name())
    }).join(".")
}

fn gen_union_from_slice_impl(u: &Union, template: &Template, type_path_by_name: &TypePathMap) -> String {
    let cases = u.cases.iter().map(|case| {
        let mut scope = template.scope();
        scope.var("name", &case.name).var_d("case", case.case);
        match &case.item_type {
            None => scope.render("from_slice.none", &["case"]).unwrap(),
            Some(item_type) => scope.var("type_name", type_path_by_name.get(item_type.name()))
                .render("from_slice.generics", &["case"]).unwrap()
        }
    }).join("");
    let mut scope = template.scope();
    scope.var("cases", cases);
    let generics = get_generics(u);
    if generics != "" {
        scope.render_to_var("from_slice", &["generics"], "fragment").unwrap();
    } else {
        scope.render_to_var("from_slice", &["none"], "fragment").unwrap();
    }
    scope.render("", &["from_slice"]).unwrap()
}

fn gen_union_write_to_impl(u: &Union, template: &Template, type_path_by_name: &TypePathMap) -> String {
    let generics = get_generics(u);
    let mut scope = template.scope();
    if generics != "" {
        let cases = u.cases.iter().filter_map(|case| {
            let mut scope = template.scope();
            scope.var("name", &case.name).var_d("case", case.case);
            match &case.item_type {
                None => None,
                Some(item_type) => Some(scope.var("type_name", type_path_by_name.get(item_type.name()))
                    .render("write_to.generics", &["case"]).unwrap())
            }
        }).join("");
        scope.var("cases", cases).render_to_var("write_to", &["generics"], "fragment").unwrap();
    } else {
        scope.render_to_var("write_to", &["none"], "fragment").unwrap();
    }
    scope.render("", &["write_to"]).unwrap()
}

fn gen_union_set_discriminant(u: &Union, template: &Template) -> String {
    let cases = u.cases.iter().map(|case| {
        let mut scope = template.scope();
        scope.var("name", &case.name).var_d("case", case.case);
        match &case.item_type {
            Some(_) => scope.render("setter", &["ref"]).unwrap(),
            None => scope.render("setter", &["none"]).unwrap()
        }
    }).join("");
    template.scope().var("cases", cases).render("", &["setter"]).unwrap()
}

fn gen_union_as_getters(u: &Union, template: &Template, type_path_by_name: &TypePathMap) -> String {
    let cases = u.cases.iter().map(|case| {
        let mut scope = template.scope();
        scope.var("name_lower", case.name.to_ascii_lowercase()).var("name", &case.name);
        match &case.item_type {
            Some(Referenced::Struct(v)) => scope.var("type_name", type_path_by_name.get(&v.name))
                .render("getters", &["struct"]).unwrap(),
            Some(Referenced::Message(v)) => scope.var("type_name", type_path_by_name.get(&v.name))
                .render("getters", &["message"]).unwrap(),
            None => scope.render("getters", &["none"]).unwrap()
        }
    }).join("");
    template.scope().var("cases", cases).render("", &["getters"]).unwrap()
}

pub fn gen_union_decl(u: &Union, type_path_by_name: &TypePathMap) -> String {
    let mut template = Template::compile(TEMPLATE).unwrap();
    let generics = get_generics(u);
    template.var("union_name", &u.name).var("generics", generics)
        .var("discriminant_path_mut", get_discriminant_path_mut(u))
        .var("discriminant_path", get_discriminant_path(u))
        .var("discriminant_type", type_path_by_name.get(&u.discriminant.root.name));
    let cases = u.cases.iter().map(|case| match &case.item_type {
        None => template.scope().var("name", &case.name).render("decl", &["none"]).unwrap(),
        Some(Referenced::Struct(v)) => template.scope()
            .var("name", &case.name).var("type_name", type_path_by_name.get(&v.name))
            .render("decl", &["struct"]).unwrap(),
        Some(Referenced::Message(v)) => template.scope()
            .var("name", &case.name).var("type_name", type_path_by_name.get(&v.name))
            .render("decl", &["message"]).unwrap()
    }).join("");
    let mut code = template.scope().var("cases", cases).render("", &["decl"]).unwrap();
    code += &gen_union_from_slice_impl(u, &template, type_path_by_name);
    code += &gen_union_write_to_impl(u, &template, type_path_by_name);
    code += &gen_union_set_discriminant(u, &template);
    code += &gen_union_as_getters(u, &template, type_path_by_name);
    code
}
