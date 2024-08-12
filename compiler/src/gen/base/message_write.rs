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
use crate::compiler::util::TypePathMap;
use crate::gen::base::message::Utilities;
use crate::gen::base::message_from_slice::generate_field_type_inline;
use crate::gen::template::Template;
use itertools::Itertools;

fn gen_field_write_impl<U: Utilities>(
    msg: &Message,
    field: &Field,
    template: &Template,
    type_path_by_name: &TypePathMap,
) -> String {
    let mut scope = template.scope();
    scope.var("name", &field.name);
    let (msg_type, union) =
        generate_field_type_inline::<U>(msg, field, template, type_path_by_name);
    if let Some(on_name) = union {
        scope.var("on_name", on_name);
    }
    scope.var("type", msg_type);
    if union.is_some() {
        scope.render("impl", &["field_union"]).unwrap()
    } else {
        scope.render("impl", &["field"]).unwrap()
    }
}

pub fn generate<U: Utilities>(
    template: &[u8],
    msg: &Message,
    type_path_by_name: &TypePathMap,
) -> String {
    let mut template = Template::compile(template).unwrap();
    template.var("msg_name", &msg.name).var("generics", U::gen_generics(msg));
    let fields = msg
        .fields
        .iter()
        .map(|field| gen_field_write_impl::<U>(msg, field, &template, type_path_by_name))
        .join("");
    template.var("fields", fields).render("", &["impl"]).unwrap()
}
