// Copyright (c) 2025, BlockProject 3D
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

use std::collections::HashMap;
use bp3d_util::simple_error;
use bp3d_protoc::compiler::structure::Structure;
use crate::buffer::BufferView;
use crate::field::primitive::{PrimitiveType, Value};

simple_error! {
    pub Error {
        FieldNotFound(String) => "Field {} not found in PrimitiveReader, is the field a primitive?"
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct PrimitiveReader(HashMap<String, Box<dyn PrimitiveType>>);

impl PrimitiveReader {
    pub fn from_struct(st: &Structure) -> Self {
        let mut map = HashMap::new();
        for field in &st.fields {
            if let Some(ty) = super::primitive::from_field(field) {
                map.insert(field.name.clone(), ty);
            }
        }
        Self(map)
    }

    pub fn get(&self, view: &BufferView) -> Result<Value> {
        let field = self.0.get(view.name()).ok_or_else(|| Error::FieldNotFound(view.name().into()))?;
        Ok(field.get(view))
    }

    pub fn set(&self, view: &mut BufferView, value: Value) -> Result<()> {
        let field = self.0.get(view.name()).ok_or_else(|| Error::FieldNotFound(view.name().into()))?;
        field.set(view, value);
        Ok(())
    }

    pub fn get_raw(&self, view: &BufferView) -> Result<Value> {
        let field = self.0.get(view.name()).ok_or_else(|| Error::FieldNotFound(view.name().into()))?;
        Ok(field.get_raw(view))
    }

    pub fn set_raw(&self, view: &mut BufferView, value: Value) -> Result<()> {
        let field = self.0.get(view.name()).ok_or_else(|| Error::FieldNotFound(view.name().into()))?;
        field.set_raw(view, value);
        Ok(())
    }

    pub fn get_bin(&self, view: &BufferView) -> Result<u64> {
        let field = self.0.get(view.name()).ok_or_else(|| Error::FieldNotFound(view.name().into()))?;
        Ok(field.get_bits(view))
    }

    pub fn set_bin(&self, view: &mut BufferView, value: u64) -> Result<()> {
        let field = self.0.get(view.name()).ok_or_else(|| Error::FieldNotFound(view.name().into()))?;
        field.set_bits(view, value);
        Ok(())
    }
}
