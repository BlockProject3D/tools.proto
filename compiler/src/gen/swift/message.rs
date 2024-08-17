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
use crate::compiler::message::{FieldType, Message};
use crate::compiler::Protocol;
use crate::gen::base::{DefaultTypeMapper, TypePathMapper};
use crate::gen::base::message::{gen_msg_field_decl, generate, Utilities};
use crate::gen::swift::util::{SwiftTypeMapper, SwiftUtils};
use crate::gen::template::Template;

const TEMPLATE: &[u8] = include_bytes!("./message.template");
const TEMPLATE_EXT: &[u8] = include_bytes!("./message.ext.template");

fn gen_message_array_type_decls(template: &Template, msg: &Message, type_path_by_name: &TypePathMapper<SwiftTypeMapper>) -> String {
    let mut scope = template.scope();
    msg.fields
        .iter()
        .filter_map(|field| {
            scope.var("name", &field.name);
            match &field.ty {
                FieldType::Array(v) => Some(
                    scope
                        .var("item_type", type_path_by_name.get(&v.item_type.name))
                        .var("codec", SwiftUtils::get_value_type(field.endianness, v.ty))
                        .render("", &["decl_array"])
                        .unwrap(),
                ),
                _ => None,
            }
        })
        .join("")
}

fn gen_initializer(template: &Template, msg: &Message, type_path_by_name: &TypePathMapper<SwiftTypeMapper>) -> String {
    let init_field_list = msg.fields.iter()
        .map(|field| gen_msg_field_decl::<SwiftUtils, _>(field, template, type_path_by_name))
        .map(|v| format!("{}", &v[..v.len() - 1])).join(", ");
    let initializers = msg.fields.iter()
        .map(|field| template.scope().var("name", &field.name).render("decl", &["initializer"]).unwrap())
        .join("");
    template.scope().var("init_field_list", init_field_list).var("initializers", initializers).render("", &["decl"]).unwrap()
}

pub fn gen_message_decl(proto: &Protocol, msg: &Message) -> String {
    let type_path_by_name = TypePathMapper::new(&proto.type_path_by_name, SwiftTypeMapper::from_protocol(proto));
    let mut template_ext = Template::compile(TEMPLATE_EXT).unwrap();
    template_ext.var("proto_name", &proto.name).var("msg_name", &msg.name);
    let initializer = gen_initializer(&template_ext, msg, &type_path_by_name);
    let mut template = Template::compile(TEMPLATE).unwrap();
    template.var("proto_name", &proto.name).var("initializer", initializer);
    let mut code = generate::<SwiftUtils, _>(template, msg, &type_path_by_name);
    code += "\n";
    code += &gen_message_array_type_decls(&template_ext, msg, &type_path_by_name);
    code
}
