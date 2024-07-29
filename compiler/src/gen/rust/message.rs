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

use std::fmt::format;
use itertools::Itertools;
use crate::compiler::message::{Field, FieldType, Message, Payload, Referenced};
use crate::compiler::structure::FixedFieldType;

fn gen_field_type(ty: FixedFieldType) -> &'static str {
    match ty {
        FixedFieldType::Int8 => "i8",
        FixedFieldType::Int16 => "i16",
        FixedFieldType::Int32 => "i32",
        FixedFieldType::Int64 => "i64",
        FixedFieldType::UInt8 => "u8",
        FixedFieldType::UInt16 => "u16",
        FixedFieldType::UInt32 => "u32",
        FixedFieldType::UInt64 => "u64",
        FixedFieldType::Float32 => "f32",
        FixedFieldType::Float64 => "f64",
        FixedFieldType::Bool => "bool"
    }
}

fn gen_field_decl(field: &Field<FieldType>) -> String {
    let mut code = format!("    pub {}: ", field.name);
    if field.optional {
        code += "Option<"
    }
    match &field.ty {
        FieldType::Fixed(ty) => code += gen_field_type(ty.ty),
        FieldType::Ref(v) => match v {
            Referenced::Struct(v) => code += &format!("{}<T>", v.name),
            Referenced::Message(v) => code += &format!("{}<'a>", v.name),
        },
        FieldType::NullTerminatedString => code += "&'a str",
        FieldType::VarcharString(_) => code += "&'a str",
        FieldType::FixedList(v) => code += &format!("bp3d_proto::util::Array<'a, {}, {}>", gen_field_type(v.ty), v.item_type.name),
    }
    if field.optional {
        code += ">"
    }
    code
}

fn gen_payload_decl(field: &Field<Payload>) -> String {
    let mut code = format!("    pub {}: ", field.name);
    if field.optional {
        code += "Option<"
    }
    match &field.ty {
        Payload::List(v) => code += &format!("bp3d_proto::util::List<'a, {}, {}>", gen_field_type(v.ty), v.item_type.name),
        Payload::Data => code += "&'a [u8]"
    }
    if field.optional {
        code += ">"
    }
    code
}

//TODO: Implement support for payloads

struct Generics<'a> {
    has_lifetime: bool,
    has_structures: bool,
    has_payload: bool,
    lifetime: &'a str,
    structures: &'a str,
    payload: &'a str
}

impl<'a> Generics<'a> {
    fn from_message(msg: &Message) -> Self {
        let has_lifetime = msg.fields.iter().any(|v| match v.ty {
            FieldType::Ref(_) => true,
            FieldType::NullTerminatedString => true,
            FieldType::VarcharString(_) => true,
            FieldType::FixedList(_) => true,
            _ => false
        });
        let has_structures = msg.fields.iter().any(|v| match &v.ty {
            FieldType::Ref(v) => match v {
                Referenced::Struct(_) => true,
                Referenced::Message(_) => false
            },
            _ => false
        });
        let has_payload = msg.payload.is_some();
        Generics {
            has_lifetime,
            has_structures,
            has_payload,
            lifetime: "'a",
            structures: "T",
            payload: "P"
        }
    }

    fn set_lifetime(&mut self, lifetime: &'a str) -> &mut Self {
        self.lifetime = lifetime;
        self
    }

    fn set_structures(&mut self, structures: &'a str) -> &mut Self {
        self.structures = structures;
        self
    }

    fn set_payload(&mut self, payload: &'a str) -> &mut Self {
        self.payload = payload;
        self
    }

    fn to_vec(&self) -> Vec<&'a str> {
        let mut generics = Vec::new();
        if self.has_lifetime {
            generics.push(self.lifetime);
        }
        if self.has_structures {
            generics.push(self.structures);
        }
        if self.has_payload {
            generics.push(self.payload);
        }
        generics
    }

    fn to_code(&self) -> String {
        let generics = self.to_vec();
        if generics.len() > 0 {
            String::from("<") + &*generics.join(",") + ">"
        } else {
            String::from("")
        }
    }
}

pub fn gen_message_decl(msg: &Message) -> String {
    let generics = Generics::from_message(msg).to_code();
    let mut code = format!("pub struct {}", msg.name);
    code += &generics;
    code += " {\n";
    let fields = msg.fields.iter()
        .map(|v| gen_field_decl(v))
        .join(",\n");
    code += &fields;
    if let Some(payload) = &msg.payload {
        code += &format!("{},\n", gen_payload_decl(payload));
    }
    code += "\n}";
    code
}

fn gen_optional(optional: bool, type_name: &str) -> String {
    if optional {
        format!("bp3d_proto::util::Optional::<{}>", type_name)
    } else {
        type_name.into()
    }
}

fn gen_field_from_slice_impl(field: &Field<FieldType>) -> String {
    let msg_code = match &field.ty {
        FieldType::Fixed(ty) => format!("{}::from_slice(&slice[byte_offset..])", gen_optional(field.optional, gen_field_type(ty.ty))),
        FieldType::Ref(v) => match v {
            Referenced::Struct(v) => format!("{}::from_slice(&slice[byte_offset..])", gen_optional(field.optional, &v.name)),
            Referenced::Message(v) => format!("{}::from_slice(&slice[byte_offset..])", gen_optional(field.optional, &v.name)),
        }
        FieldType::NullTerminatedString => format!("{}::from_slice(&slice[byte_offset..])", gen_optional(field.optional, "bp3d_proto::util::NullTerminatedString")),
        FieldType::VarcharString(v) => format!("{}::from_slice(&slice[byte_offset..])", gen_optional(field.optional, &format!("bp3d_proto::util::VarcharString<{}>", gen_field_type(v.ty)))),
        FieldType::FixedList(v) => format!("{}::from_slice(&slice[byte_offset..])", gen_optional(field.optional, &format!("bp3d_proto::util::Array<{}, {}>", gen_field_type(v.ty), v.item_type.name)))
    };
    let mut code = format!("        let {}_msg = {}?;\n", field.name, msg_code);
    code += &format!("        byte_offset += {}_msg.size();\n", field.name);
    code += &format!("        let {} = {}_msg.into_inner();\n", field.name, field.name);
    code
}

fn gen_payload_from_slice_impl(field: &Field<Payload>) -> String {
    let msg_code = match &field.ty {
        Payload::List(v) => format!("{}::from_slice(&slice[byte_offset..])", gen_optional(field.optional, &format!("bp3d_proto::util::List<{}, {}>", gen_field_type(v.ty), v.item_type.name))),
        Payload::Data => format!("{}::from_slice(&slice[byte_offset..])", gen_optional(field.optional, "bp3d_proto::util::Buffer"))
    };
    let mut code = format!("        let {}_msg = {}?;\n", field.name, msg_code);
    code += &format!("        byte_offset += {}_msg.size();\n", field.name);
    code += &format!("        let {} = {}_msg.into_inner();\n", field.name, field.name);
    code
}

pub fn gen_message_from_slice_impl(msg: &Message) -> String {
    let mut generics = Generics::from_message(msg);
    generics.has_structures = false;
    generics.has_lifetime = true;
    let generics = generics.to_code();
    let instance = Generics::from_message(msg).set_structures("&'a [u8]").to_code();
    let mut code = format!("impl{} bp3d_proto::message::FromSlice<'a> for {}{} {{\n", generics, msg.name, instance);
    code += "    type Output = Self;\n\n";
    code += "    fn from_slice(slice: &'a [u8]) -> Result<bp3d_proto::message::Message<Self>, bp3d_proto::message::Error> {\n";
    code += "        use bp3d_proto::message::FromSlice;\n";
    code += "        let mut byte_offset: usize = 0;\n";
    for field in &msg.fields {
        code += &gen_field_from_slice_impl(field);
    }
    if let Some(payload) = &msg.payload {
        code += &gen_payload_from_slice_impl(payload);
    }
    code += &format!("        let data = {} {{\n", msg.name);
    for field in &msg.fields {
        code += &format!("            {},\n", field.name);
    }
    if let Some(payload) = &msg.payload {
        code += &format!("            {},\n", payload.name);
    }
    code += "        };\n";
    code += "        Message::new(byte_offset, data)\n";
    code += "    }";
    code += "\n}";
    code
}
