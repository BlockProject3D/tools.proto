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

use crate::model::message::{MessageField, MessageFieldValue};
use crate::model::protocol::Description;
use crate::model::structure::{Offset, SimpleType, StructField, StructFieldRaw, StructFieldView};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Typedef {
    pub name: String,
    pub value: Option<MessageFieldValue>,
    pub optional: Option<bool>,
    pub description: Option<Description>,
    pub item_type: Option<String>,
    pub codec: Option<String>,
    pub raw: Option<StructFieldRaw>,
    pub view: Option<StructFieldView>,
    pub array_len: Option<usize>,
    pub offset: Option<Offset>,
}

impl Typedef {
    pub fn to_struct(&self) -> Option<StructField> {
        if self.raw.is_some() {
            Some(StructField {
                name: self.name.clone(),
                raw: self.raw.clone(),
                view: self.view.clone(),
                offset: self.offset.clone(),
                array_len: self.array_len,
                description: self.description.clone(),
                item_type: self.item_type.clone(),
            })
        } else {
            None
        }
    }

    pub fn to_message(&self) -> Option<MessageField> {
        match &self.raw {
            None => Some(MessageField {
                name: self.name.clone(),
                value: self.value.clone(),
                codec: self.codec.clone(),
                description: self.description.clone(),
                optional: self.optional,
                item_type: self.item_type.clone(),
            }),
            Some(v) => {
                let bit_size = v.get_bit_size();
                if v.get_simple_type() == SimpleType::Unsigned && bit_size % 8 == 0 {
                    Some(MessageField {
                        name: self.name.clone(),
                        description: self.description.clone(),
                        codec: self.codec.clone(),
                        optional: self.optional,
                        item_type: self.item_type.clone(),
                        value: Some(MessageFieldValue::Unsigned { bits: bit_size }),
                    })
                } else {
                    None
                }
            }
        }
    }
}
