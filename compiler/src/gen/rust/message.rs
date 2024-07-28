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

fn gen_field(field: Field<FieldType>) -> String {
    let mut code = format!("    pub {}: ", field.name);
    if field.optional {
        code += "Option<"
    }
    match field.ty {
        FieldType::Fixed(ty) => code += gen_field_type(ty),
        FieldType::Ref(v) => match v {
            Referenced::Struct(v) => code += &format!("{}<T>", v.name),
            Referenced::Message(v) => code += &format!("{}<'a>", v.name),
        },
        FieldType::NullTerminatedString => code += "&'a str",
        FieldType::VarcharString(_) => code += "&'a str",
        FieldType::FixedList(v) => code += &format!("&'a [{}]", v.item_type.name),
    }
    if field.optional {
        code += ">"
    }
    code
}

pub fn gen_message_decl(msg: &Message) -> String {
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
    let mut generics = Vec::new();
    if has_lifetime {
        generics.push("'a");
    }
    if has_structures {
        generics.push("T");
    }
    if has_payload {
        generics.push("P");
    }
    let mut code = format!("pub struct {}", msg.name);
    if generics.len() > 0 {
        let motherfuckingrust = String::from("<") + &*generics.join(",") + ">";
        code += &motherfuckingrust;
    }
    code += " {\n";
    let fields = msg.fields.iter()
        .map(|v| gen_field(v.clone()))
        .join(",\n");
    code += &fields;
    code += "\n}";
    code
}

pub fn gen_message_impl(msg: &Message) -> String {
    
    todo!()
}
