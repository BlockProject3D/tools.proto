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

use crate::compiler::message::{Field, FieldType, Message, Referenced};
use crate::compiler::util::TypePathMap;
use crate::gen::base::message::Utilities;
use crate::gen::base::message_from_slice::generate_from_slice_impl;
use crate::gen::base::map::{DefaultTypeMapper, TypePathMapper};
use crate::gen::rust::util::RustUtils;
use crate::gen::template::Template;
use itertools::Itertools;

const TEMPLATE: &[u8] = include_bytes!("./message.offsets.template");

fn gen_message_offset_field(
    field: &Field,
    template: &Template,
    type_path_by_name: &TypePathMapper<DefaultTypeMapper>,
) -> String {
    let mut scope = template.scope();
    scope.var("name", &field.name);
    match &field.ty {
        FieldType::Ref(Referenced::Message(v)) => {
            scope.var("type_name", type_path_by_name.get(&v.name));
            match field.optional {
                true => scope.render("decl", &["field", "msg_optional"]).unwrap(),
                false => scope.render("decl", &["field", "msg"]).unwrap(),
            }
        }
        _ => scope.render("decl", &["field"]).unwrap(),
    }
}

pub fn gen_message_offsets_decl(msg: &Message, type_path_by_name: &TypePathMap) -> String {
    let type_path_by_name = TypePathMapper::new(type_path_by_name, DefaultTypeMapper);
    let mut template = Template::compile(TEMPLATE).unwrap();
    template.var("msg_name", &msg.name).var("generics", RustUtils::get_generics(msg));
    let fields = msg
        .fields
        .iter()
        .map(|field| gen_message_offset_field(field, &template, &type_path_by_name))
        .join("");
    let mut code = template.var("fields", fields).render("", &["decl"]).unwrap();
    code += "\n";
    code += &generate_from_slice_impl::<RustUtils, _>(msg, &template, &type_path_by_name);
    code
}
