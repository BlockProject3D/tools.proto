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

use std::borrow::Cow;
use std::collections::HashMap;

fn capitalize(value: &str) -> Cow<str> {
    if value.len() == 0 {
        return value.into();
    }
    if value.as_bytes()[0] >= b'A' && value.as_bytes()[0] <= b'Z' {
        value.into()
    } else {
        (value[..1].to_ascii_uppercase() + &value[1..]).into()
    }
}

pub struct FunctionMap<'fragment> {
    map: HashMap<&'fragment str, fn(&str) -> Cow<str>>,
}

impl<'fragment> Default for FunctionMap<'fragment> {
    fn default() -> Self {
        let mut map = Self::new();
        map.add_defaults();
        map
    }
}

impl<'fragment> FunctionMap<'fragment> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add_defaults(&mut self) {
        self.add("upper", |v| v.to_uppercase().into())
            .add("lower", |v| v.to_lowercase().into())
            .add("capitalize", capitalize);
    }

    pub fn add(&mut self, name: &'fragment str, f: fn(&str) -> Cow<str>) -> &mut Self {
        self.map.insert(name, f);
        self
    }

    pub(crate) fn get(&self, name: &str) -> Option<fn(&str) -> Cow<str>> {
        self.map.get(name).map(|v| *v)
    }
}
