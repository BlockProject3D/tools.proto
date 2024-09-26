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

use crate::compiler::message::{Field, Message};
use crate::compiler::util::types::TypeMapper;
use crate::gen::base::map::TypePathMapper;
use crate::gen::base::message::Utilities;
use crate::gen::base::message_common::generate_field_type_inline;
use crate::gen::template::Template;
use itertools::Itertools;

fn gen_field_write_impl<U: Utilities, T: TypeMapper>(
    msg: &Message,
    field: &Field,
    template: &Template,
    type_path_map: &TypePathMapper<T>,
    function: &str,
) -> String {
    let mut scope = template.scope();
    scope.var("name", &field.name);
    let msg_type = generate_field_type_inline::<U, T>(msg, field, template, type_path_map);
    let union = field.ty.as_union();
    if let Some(v) = union {
        scope.var("on_name", &v.on_name);
    }
    scope.var("type", msg_type);
    if union.is_some() {
        scope.render(function, &["field_union"]).unwrap()
    } else if field.ty.is_string() {
        scope.render(function, &["field_string"]).unwrap()
    } else {
        scope.render(function, &["field"]).unwrap()
    }
}

pub fn generate<'variable, U: Utilities, T: TypeMapper>(
    mut template: Template<'_, 'variable>,
    msg: &'variable Message,
    type_path_map: &TypePathMapper<T>,
    function: &str,
) -> String {
    template.var("msg_name", &msg.name);
    let fields = msg
        .fields
        .iter()
        .map(|field| gen_field_write_impl::<U, T>(msg, field, &template, type_path_map, function))
        .join("");
    template.var("fields", fields).render("", &[function]).unwrap()
}
