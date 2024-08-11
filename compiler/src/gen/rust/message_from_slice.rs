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
use crate::compiler::message::{Field, FieldType, Message, Referenced};
use crate::compiler::util::TypePathMap;
use crate::gen::rust::util::{gen_optional, Generics, get_value_type_inline, get_value_type};
use crate::gen::template::Template;

const TEMPLATE: &[u8] = include_bytes!("./message.from_slice.template");

pub fn gen_field_msg_type<'a>(msg: &Message, field: &'a Field, template: &Template, type_path_by_name: &TypePathMap) -> (String, Option<&'a str>) {
    let mut union = None;
    let msg_type = match &field.ty {
        FieldType::Fixed(ty) => gen_optional(field.optional, get_value_type_inline(field.endianness, ty.ty)),
        FieldType::Ref(v) => match v {
            Referenced::Struct(v) => gen_optional(field.optional, type_path_by_name.get(&v.name)),
            Referenced::Message(v) => gen_optional(field.optional, type_path_by_name.get(&v.name))
        }
        FieldType::NullTerminatedString => gen_optional(field.optional, "bp3d_proto::message::util::NullTerminatedString"),
        FieldType::VarcharString(v) => gen_optional(field.optional, &template.scope()
            .var("codec", get_value_type(field.endianness, v.ty)).render("", &["varchar"]).unwrap()),
        FieldType::Array(v) => gen_optional(field.optional, &template.scope()
            .var("codec", get_value_type(field.endianness, v.ty))
            .var("type_name", type_path_by_name.get(&v.item_type.name))
            .render("", &["array"]).unwrap()),
        FieldType::Union(v) => {
            union = Some(&*v.on_name);
            gen_optional(field.optional, type_path_by_name.get(&v.r.name))
        },
        FieldType::List(v) => {
            match msg.is_embedded() {
                false => gen_optional(field.optional, &template.scope()
                    .var("codec", get_value_type(field.endianness, v.ty))
                    .var("type_name", type_path_by_name.get(&v.item_type.name))
                    .render("", &["unsized"]).unwrap()),
                true => gen_optional(field.optional, &template.scope()
                    .var("codec", get_value_type(field.endianness, v.ty))
                    .var("type_name", type_path_by_name.get(&v.item_type.name))
                    .render("", &["list"]).unwrap()),
            }
        },
        FieldType::Payload => gen_optional(field.optional, "bp3d_proto::message::util::Buffer")
    };
    (msg_type, union)
}

fn gen_field_from_slice_impl(msg: &Message, field: &Field, template: &Template, type_path_by_name: &TypePathMap) -> String {
    let mut scope = template.scope();
    scope.var("name", &field.name);
    let (msg_type, union) = gen_field_msg_type(msg, field, template, type_path_by_name);
    if let Some(on_name) = union {
        scope.var("on_name", on_name);
    }
    scope.var("type", msg_type);
    if union.is_some() {
        scope.render("impl", &["field_union"]).unwrap()
    } else if field.ty.is_message_reference() {
        scope.render("impl", &["field_msg"]).unwrap()
    } else {
        scope.render("impl", &["field"]).unwrap()
    }
}

pub fn render_message_from_slice_impl(msg: &Message, template: &Template, type_path_by_name: &TypePathMap) -> String {
    let fields = msg.fields.iter().map(|field|
        gen_field_from_slice_impl(msg, field, &template, type_path_by_name)).join("");
    let field_names = msg.fields.iter().map(|field| template.scope()
        .var("name", &field.name).render("impl", &["field_name"]).unwrap()).join("");
    template.scope().var("fields", fields).var("field_names", field_names)
        .render("", &["impl"]).unwrap()
}

pub fn gen_message_from_slice_impl(msg: &Message, type_path_by_name: &TypePathMap) -> String {
    let mut template = Template::compile(TEMPLATE).unwrap();
    template.var("msg_name", &msg.name).var("generics", Generics::from_message(msg).to_code());
    render_message_from_slice_impl(msg, &template, type_path_by_name)
}
