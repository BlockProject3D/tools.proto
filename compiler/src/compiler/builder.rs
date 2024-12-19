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

use crate::compiler::message::{Field, FieldType, HeaderField, SizeInfo};
use crate::model::protocol::{Description, Endianness};

pub struct FieldBuilder {
    name: String,
    optional: bool,
    endianness: Endianness,
    header: Option<HeaderField>,
    codec: Option<String>,
    size_info: SizeInfo,
    description: Option<Description>,
}

impl FieldBuilder {
    pub fn new(name: String, optional: bool, endianness: Endianness) -> Self {
        Self {
            name,
            description: None,
            endianness,
            optional,
            header: None,
            codec: None,
            size_info: SizeInfo {
                is_element_dyn_sized: false,
                is_dyn_sized: true,
            },
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn has_codec(&self) -> bool {
        self.codec.is_some()
    }

    pub fn description(mut self, description: Option<Description>) -> Self {
        self.description = description;
        self
    }

    pub fn header(mut self, header: Option<HeaderField>) -> Self {
        self.header = header;
        self
    }

    pub fn codec(mut self, codec: Option<String>) -> Self {
        self.codec = codec;
        self
    }

    pub fn size_info(mut self, size_info: SizeInfo) -> Self {
        self.size_info = size_info;
        self
    }

    pub fn fixed_size(mut self) -> Self {
        self.size_info = SizeInfo {
            is_dyn_sized: false,
            is_element_dyn_sized: false,
        };
        self
    }

    pub fn dynamic_size(mut self) -> Self {
        self.size_info = SizeInfo {
            is_dyn_sized: true,
            is_element_dyn_sized: true,
        };
        self
    }

    pub fn build(self, ty: FieldType) -> Field {
        Field {
            name: self.name,
            header: self.header,
            ty,
            optional: self.optional,
            size: self.size_info,
            endianness: self.endianness,
            description: self.description,
            codec: self.codec,
        }
    }
}
