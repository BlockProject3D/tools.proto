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
use crate::gen::rust::util::{gen_field_type, Generics};

fn gen_field_decl(field: &Field, type_path_by_name: &TypePathMap) -> String {
    let mut code = format!("    pub {}: ", field.name);
    if field.optional {
        code += "Option<"
    }
    match &field.ty {
        FieldType::Fixed(ty) => code += gen_field_type(ty.ty),
        FieldType::Ref(v) => match v {
            Referenced::Struct(v) => code += &format!("{}<&'a [u8]>", type_path_by_name.get(&v.name)),
            Referenced::Message(v) => code += &format!("{}<'a>", type_path_by_name.get(&v.name)),
        },
        FieldType::NullTerminatedString => code += "&'a str",
        FieldType::VarcharString(_) => code += "&'a str",
        FieldType::Array(v) => code += &format!("bp3d_proto::message::util::Array::<&'a [u8], {}, {}<&'a [u8]>>", gen_field_type(v.ty), type_path_by_name.get(&v.item_type.name)),
        FieldType::Union(v) => code += &format!("{}<'a>", type_path_by_name.get(&v.r.name)),
        FieldType::List(v) => code += &format!("bp3d_proto::message::util::List::<&'a [u8], {}, {}<&'a [u8]>>", gen_field_type(v.ty), type_path_by_name.get(&v.item_type.name)),
        FieldType::Payload => code += "&'a [u8]"

    }
    if field.optional {
        code += ">"
    }
    code
}

fn capitalize(s: &str) -> String {
    s[..1].to_ascii_uppercase() + &s[1..]
}

fn gen_message_array_type_decls(msg: &Message, type_path_by_name: &TypePathMap) -> String {
    let flag = msg.fields.iter().any(|v| match v.ty {
        FieldType::Array(_) | FieldType::List(_) => true,
        _ => false
    });
    if !flag {
        return String::new();
    }
    let mut code = format!("pub mod {} {{\n", msg.name.to_ascii_lowercase());
    for field in &msg.fields {
        match &field.ty {
            FieldType::Array(v) => {
                code += &format!("    pub type {}Type<'a, T> = bp3d_proto::message::util::Array<T, {}, {}<&'a [u8]>>;\n", capitalize(&field.name), gen_field_type(v.ty), type_path_by_name.get_with_default_prefix(&v.item_type.name, "super::"));
                code += &format!("    pub type {}Ref<'a> = {}<&'a [u8]>;\n", capitalize(&field.name), type_path_by_name.get_with_default_prefix(&v.item_type.name, "super::"));
                code += &format!("    pub type {}Mut<'a> = {}<&'a mut [u8]>;\n", capitalize(&field.name), type_path_by_name.get_with_default_prefix(&v.item_type.name, "super::"));
            },
            FieldType::List(v) => {
                code += &format!("    pub type {}Type<'a, T> = bp3d_proto::message::util::List<T, {}, {}<&'a [u8]>>;\n", capitalize(&field.name), gen_field_type(v.ty), type_path_by_name.get_with_default_prefix(&v.item_type.name, "super::"));
                code += &format!("    pub type {}Ref<'a> = {}<&'a [u8]>;\n", capitalize(&field.name), type_path_by_name.get_with_default_prefix(&v.item_type.name, "super::"));
                code += &format!("    pub type {}Mut<'a> = {}<&'a mut [u8]>;\n", capitalize(&field.name), type_path_by_name.get_with_default_prefix(&v.item_type.name, "super::"));
            },
            _ => ()
        }
    }
    code += "}\n";
    code
}

pub fn gen_message_decl(msg: &Message, type_path_by_name: &TypePathMap) -> String {
    let generics = Generics::from_message(msg).to_code();
    let mut code = format!("#[derive(Copy, Clone, Debug)]\npub struct {}", msg.name);
    code += &generics;
    code += " {\n";
    let fields = msg.fields.iter()
        .map(|v| gen_field_decl(v, type_path_by_name))
        .join(",\n");
    code += &fields;
    code += "\n}\n\n";
    code += &gen_message_array_type_decls(msg, type_path_by_name);
    code += "\n";
    code
}
