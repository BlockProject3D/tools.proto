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

use crate::compiler::structure::FixedFieldType;
use crate::compiler::Error;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Enum {
    pub name: String,
    pub largest: usize,
    pub repr_type: FixedFieldType,
    pub variants: Vec<(String, usize)>,
    pub variants_map: HashMap<String, usize>,
}

impl Enum {
    pub fn from_model(value: crate::model::protocol::Enum) -> Result<Enum, Error> {
        let mut variants: Vec<(String, usize)> = value.variants.into_iter().collect();
        variants.sort_by(|(_, v), (_, v1)| v.cmp(v1));
        let mut variants_map = HashMap::new();
        let largest = variants.last().map(|(_, v)| *v).ok_or(Error::ZeroEnum)?;
        for (k, v) in &variants {
            variants_map.insert(k.clone(), *v);
        }
        let repr_type = FixedFieldType::from_max_value(largest)?;
        Ok(Enum {
            name: value.name,
            repr_type,
            variants,
            variants_map,
            largest,
        })
    }
}
