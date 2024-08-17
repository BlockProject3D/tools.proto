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

use crate::compiler::message::{FieldType, Message};
use crate::compiler::util::TypePathMap;
use crate::gen::base::message::{generate, Utilities};
use crate::gen::rust::util::RustUtils;
use crate::gen::template::Template;
use itertools::Itertools;
use crate::gen::base::{DefaultTypeMapper, TypePathMapper};

const TEMPLATE: &[u8] = include_bytes!("./message.template");
const TEMPLATE_EXT: &[u8] = include_bytes!("./message.ext.template");

fn gen_message_array_type_decls(msg: &Message, type_path_by_name: &TypePathMapper<DefaultTypeMapper>) -> String {
    let mut template = Template::compile(TEMPLATE_EXT).unwrap();
    template.var("msg_name", &msg.name);
    msg.fields
        .iter()
        .filter_map(|field| {
            template.var("name", &field.name);
            match &field.ty {
                FieldType::Array(v) => Some(
                    template
                        .var("item_type", type_path_by_name.get(&v.item_type.name))
                        .var("codec", RustUtils::get_value_type(field.endianness, v.ty))
                        .render("", &["decl_array"])
                        .unwrap(),
                ),
                FieldType::List(v) => Some(
                    template
                        .var("item_type", type_path_by_name.get(&v.item_type.name))
                        .var("codec", RustUtils::get_value_type(field.endianness, v.ty))
                        .render("", &["decl_list"])
                        .unwrap(),
                ),
                _ => None,
            }
        })
        .join("")
}

pub fn gen_message_decl(msg: &Message, type_path_by_name: &TypePathMap) -> String {
    let type_path_by_name = TypePathMapper::new(type_path_by_name, DefaultTypeMapper);
    let mut code = generate::<RustUtils, _>(Template::compile(TEMPLATE).unwrap(), msg, &type_path_by_name);
    code += "\n";
    code += &gen_message_array_type_decls(msg, &type_path_by_name);
    code
}
