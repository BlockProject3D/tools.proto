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
use crate::compiler::util::TypePathMap;
use crate::gen::rust::util::{gen_field_type, gen_optional, Generics};

fn gen_field_write_impl(field: &Field<FieldType>, type_path_by_name: &TypePathMap) -> String {
    match &field.ty {
        FieldType::Fixed(ty) => format!("        {}::write_to(&input.{}, &mut out)?;\n", gen_optional(field.optional, gen_field_type(ty.ty)), field.name),
        FieldType::Ref(v) => match v {
            Referenced::Struct(v) => format!("        {}::write_to(&input.{}, &mut out)?;\n", gen_optional(field.optional, type_path_by_name.get(&v.name)), field.name),
            Referenced::Message(v) => format!("        {}::write_to(&input.{}, &mut out)?;\n", gen_optional(field.optional, type_path_by_name.get(&v.name)), field.name),
        }
        FieldType::NullTerminatedString => format!("        {}::write_to(&input.{}, &mut out)?;\n", gen_optional(field.optional, "bp3d_proto::message::util::NullTerminatedString"), field.name),
        FieldType::VarcharString(v) => format!("        {}::write_to(&input.{}, &mut out)?;\n", gen_optional(field.optional, &format!("bp3d_proto::message::util::VarcharString<{}>", gen_field_type(v.ty))), field.name),
        FieldType::FixedList(v) => format!("        {}::write_to(&input.{}, &mut out)?;\n", gen_optional(field.optional, &format!("bp3d_proto::message::util::Array<{}, {}>", gen_field_type(v.ty), type_path_by_name.get(&v.item_type.name))), field.name),
        FieldType::Union(v) => format!("{}::write_to(&input.{}, &input.{}, &mut out)?;\n", type_path_by_name.get(&v.r.name), field.name, v.on_name)
    }
}

fn gen_payload_write_impl(field: &Field<Payload>, type_path_by_name: &TypePathMap) -> String {
    match &field.ty {
        Payload::List(v) => format!("        {}::write_to(&input.{}, &mut out)?;\n", gen_optional(field.optional, &format!("bp3d_proto::message::util::List<{}, {}>", gen_field_type(v.ty), type_path_by_name.get(&v.item_type.name))), field.name),
        Payload::Data => format!("        {}::write_to(&input.{}, &mut out)?;\n", gen_optional(field.optional, "bp3d_proto::message::util::Buffer"), field.name)
    }
}

pub fn gen_message_write_impl(msg: &Message, type_path_by_name: &TypePathMap) -> String {
    let generics = Generics::from_message(msg).to_code();
    let mut code = format!("impl{} bp3d_proto::message::WriteTo for {}{} {{\n", generics, msg.name, generics);
    code += "    type Input = Self;\n\n";
    code += "    fn write_to<W: std::io::Write>(input: &Self, mut out: W) -> bp3d_proto::message::Result<()> {\n";
    for field in &msg.fields {
        code += &gen_field_write_impl(field, type_path_by_name);
    }
    if let Some(payload) = &msg.payload {
        code += &gen_payload_write_impl(payload, type_path_by_name);
    }
    code += "        Ok(())\n";
    code += "    }";
    code += "\n}";
    code
}
