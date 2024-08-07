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
use crate::gen::rust::util::Generics;

fn gen_message_from_slice_offsets_impl(msg: &Message, type_path_by_name: &TypePathMap) -> String {
    let generics = Generics::from_message(msg).to_code();
    let mut code = format!("impl<'a> bp3d_proto::message::FromSliceWithOffsets<'a> for {}{} {{\n", msg.name, generics);
    code += &format!("    type Offsets = {}Offsets;\n\n", msg.name);
    code += "    fn from_slice_with_offsets(slice: &'a [u8]) -> bp3d_proto::message::Result<bp3d_proto::message::Message<(Self, Self::Offsets)>> {\n";
    code += "        use bp3d_proto::message::FromSlice;\n";
    code += "        let mut offsets = Self::Offsets::default();\n";
    code += "        let mut byte_offset: usize = 0;\n";
    for field in &msg.fields {
        code += &crate::gen::rust::message_from_slice::gen_field_from_slice_impl(msg, field, type_path_by_name, true);
    }
    code += &format!("        let data = {} {{\n", msg.name);
    for field in &msg.fields {
        code += &format!("            {},\n", field.name);
    }
    code += "        };\n";
    code += "        Ok(bp3d_proto::message::Message::new(byte_offset, (data, offsets)))\n";
    code += "    }";
    code += "\n}";
    code
}

fn gen_message_offset_field(field: &Field) -> String {
    match &field.ty {
        FieldType::Ref(v) => match v {
            Referenced::Message(v) => if field.optional {
                format!("    pub {}: bp3d_proto::message::FieldOffset,\n    pub {}_offsets: Option<{}Offsets>", field.name, field.name, v.name)
            } else {
                format!("    pub {}: bp3d_proto::message::FieldOffset,\n    pub {}_offsets: {}Offsets", field.name, field.name, v.name)
            },
            _ => format!("    pub {}: bp3d_proto::message::FieldOffset", field.name)
        },
        _ => format!("    pub {}: bp3d_proto::message::FieldOffset", field.name)
    }
}

pub fn gen_message_offsets_decl(msg: &Message, type_path_by_name: &TypePathMap) -> String {
    let mut code = format!("#[derive(Copy, Clone, Debug, Default)]\npub struct {}Offsets {{\n", msg.name);
    let fields = msg.fields.iter()
        .map(gen_message_offset_field)
        .join(",\n");
    code += &fields;
    code += "\n}\n";
    code += &gen_message_from_slice_offsets_impl(msg, type_path_by_name);
    code
}
