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

use crate::compiler::message::{Field, FieldType, Message, Payload, Referenced};
use crate::gen::rust::util::{gen_field_type, Generics};

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
