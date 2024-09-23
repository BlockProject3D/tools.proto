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

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type")]
pub enum StructFieldView {
    Enum { name: String },
    FloatRange { min: f64, max: f64 },
    FloatMultiplier { multiplier: f64 },
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type")]
pub enum StructFieldType {
    Signed { bits: usize },
    Unsigned { bits: usize },
    Float { bits: usize },
    Boolean { bits: usize },
    Struct { item_type: String },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SimpleType {
    Signed,
    Unsigned,
    Float,
    Boolean,
    Struct,
}

impl StructFieldType {
    pub fn get_simple_type(&self) -> SimpleType {
        match self {
            StructFieldType::Signed { .. } => SimpleType::Signed,
            StructFieldType::Unsigned { .. } => SimpleType::Unsigned,
            StructFieldType::Float { .. } => SimpleType::Float,
            StructFieldType::Boolean { .. } => SimpleType::Boolean,
            StructFieldType::Struct { .. } => SimpleType::Struct,
        }
    }

    pub fn get_bit_size(&self) -> Option<usize> {
        match self {
            StructFieldType::Signed { bits } => Some(*bits),
            StructFieldType::Unsigned { bits } => Some(*bits),
            StructFieldType::Float { bits } => Some(*bits),
            StructFieldType::Boolean { bits } => Some(*bits),
            StructFieldType::Struct { .. } => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct StructField {
    pub name: String,
    pub description: Option<String>,
    pub info: StructFieldType,
    pub view: Option<StructFieldView>,
    pub array_len: Option<usize>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Structure {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<StructField>,
}
