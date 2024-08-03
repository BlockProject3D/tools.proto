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
use crate::gen::rust::util::{gen_field_type, gen_optional, Generics};

fn gen_field_from_slice_impl(field: &Field, type_path_by_name: &TypePathMap) -> String {
    let msg_code = match &field.ty {
        FieldType::Fixed(ty) => format!("{}::from_slice(&slice[byte_offset..])", gen_optional(field.optional, gen_field_type(ty.ty))),
        FieldType::Ref(v) => match v {
            Referenced::Struct(v) => format!("{}::from_slice(&slice[byte_offset..])", gen_optional(field.optional, type_path_by_name.get(&v.name))),
            Referenced::Message(v) => format!("{}::from_slice(&slice[byte_offset..])", gen_optional(field.optional, type_path_by_name.get(&v.name))),
        }
        FieldType::NullTerminatedString => format!("{}::from_slice(&slice[byte_offset..])", gen_optional(field.optional, "bp3d_proto::message::util::NullTerminatedString")),
        FieldType::VarcharString(v) => format!("{}::from_slice(&slice[byte_offset..])", gen_optional(field.optional, &format!("bp3d_proto::message::util::VarcharString<{}>", gen_field_type(v.ty)))),
        FieldType::Array(v) => format!("{}::from_slice(&slice[byte_offset..])", gen_optional(field.optional, &format!("bp3d_proto::message::util::Array<&'a [u8], {}, {}>", gen_field_type(v.ty), type_path_by_name.get(&v.item_type.name)))),
        FieldType::Union(v) => format!("{}::from_slice(&slice[byte_offset..], &{})", gen_optional(field.optional, type_path_by_name.get(&v.r.name)), v.on_name),
        FieldType::List(v) => format!("{}::from_slice(&slice[byte_offset..])", gen_optional(field.optional, &format!("bp3d_proto::message::util::List<&'a [u8], {}, {}>", gen_field_type(v.ty), type_path_by_name.get(&v.item_type.name)))),
        FieldType::Payload => format!("{}::from_slice(&slice[byte_offset..])", gen_optional(field.optional, "bp3d_proto::message::util::Buffer"))
    };
    let mut code = format!("        let {}_msg = {}?;\n", field.name, msg_code);
    code += &format!("        byte_offset += {}_msg.size();\n", field.name);
    code += &format!("        let {} = {}_msg.into_inner();\n", field.name, field.name);
    code
}

pub fn gen_message_from_slice_impl(msg: &Message, type_path_by_name: &TypePathMap) -> String {
    let generics = Generics::from_message(msg).to_code();
    let mut code = format!("impl<'a> bp3d_proto::message::FromSlice<'a> for {}{} {{\n", msg.name, generics);
    code += "    type Output = Self;\n\n";
    code += "    fn from_slice(slice: &'a [u8]) -> bp3d_proto::message::Result<bp3d_proto::message::Message<Self>> {\n";
    code += "        let mut byte_offset: usize = 0;\n";
    for field in &msg.fields {
        code += &gen_field_from_slice_impl(field, type_path_by_name);
    }
    code += &format!("        let data = {} {{\n", msg.name);
    for field in &msg.fields {
        code += &format!("            {},\n", field.name);
    }
    code += "        };\n";
    code += "        Ok(bp3d_proto::message::Message::new(byte_offset, data))\n";
    code += "    }";
    code += "\n}";
    code
}
