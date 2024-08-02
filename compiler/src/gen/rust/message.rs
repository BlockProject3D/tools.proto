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
use crate::compiler::message::{Field, FieldType, Message, Payload, Referenced};
use crate::compiler::util::TypePathMap;
use crate::gen::rust::util::{gen_field_type, Generics};

fn gen_field_decl(field: &Field<FieldType>, type_path_by_name: &TypePathMap) -> String {
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
        FieldType::FixedList(v) => code += &format!("bp3d_proto::message::util::Array<'a, {}, {}>", gen_field_type(v.ty), type_path_by_name.get(&v.item_type.name)),
        FieldType::Union(v) => code += &format!("{}<'a>", type_path_by_name.get(&v.r.name))
    }
    if field.optional {
        code += ">"
    }
    code
}

fn gen_payload_decl(field: &Field<Payload>, type_path_by_name: &TypePathMap) -> String {
    let mut code = format!("    pub {}: ", field.name);
    if field.optional {
        code += "Option<"
    }
    match &field.ty {
        Payload::List(v) => code += &format!("bp3d_proto::message::util::List<'a, {}, {}>", gen_field_type(v.ty), type_path_by_name.get(&v.item_type.name)),
        Payload::Data => code += "&'a [u8]"
    }
    if field.optional {
        code += ">"
    }
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
    if let Some(payload) = &msg.payload {
        code += &format!("{},\n", gen_payload_decl(payload, type_path_by_name));
    }
    code += "\n}";
    code
}
