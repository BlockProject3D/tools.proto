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

use crate::compiler::message::{FieldType, Message};
use crate::compiler::structure::FixedFieldType;

pub struct Generics {
    pub has_lifetime: bool
}

impl Generics {
    pub fn from_message(msg: &Message) -> Self {
        let has_lifetime = msg.fields.iter().any(|v| match v.ty {
            FieldType::Ref(_) => true,
            FieldType::NullTerminatedString => true,
            FieldType::VarcharString(_) => true,
            FieldType::Array(_) => true,
            FieldType::Union(_) => true,
            FieldType::List(_) => true,
            FieldType::Payload => true,
            _ => false
        });
        Generics {
            has_lifetime
        }
    }

    pub fn to_vec(&self) -> Vec<&'static str> {
        let mut generics = Vec::new();
        if self.has_lifetime {
            generics.push("'a");
        }
        generics
    }

    pub fn to_code(&self) -> String {
        let generics = self.to_vec();
        if generics.len() > 0 {
            String::from("<") + &*generics.join(",") + ">"
        } else {
            String::from("")
        }
    }
}

pub fn gen_field_type(ty: FixedFieldType) -> &'static str {
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

pub fn gen_optional(optional: bool, type_name: &str) -> String {
    if optional {
        format!("bp3d_proto::message::util::Optional::<{}>", type_name)
    } else {
        type_name.into()
    }
}
