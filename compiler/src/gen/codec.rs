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

use crate::gen::template::Template;
use std::collections::HashMap;

pub struct Codec<'fragment, 'variable> {
    pub decl: Template<'fragment, 'variable>,
    pub from_bytes: Template<'fragment, 'variable>,
    pub write: Template<'fragment, 'variable>,
}

impl Codec<'_, '_> {
    pub fn from_static_bytes(decl: &'static [u8], from_bytes: &'static [u8], write: &'static [u8]) -> Self {
        Self {
            decl: Template::compile(decl).unwrap(),
            from_bytes: Template::compile(from_bytes).unwrap(),
            write: Template::compile(write).unwrap(),
        }
    }
}

pub struct CodecMap<'fragment, 'variable> {
    map: HashMap<&'fragment str, Codec<'fragment, 'variable>>,
}

impl Default for CodecMap<'_, '_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'fragment, 'variable> CodecMap<'fragment, 'variable> {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn with_default(codec: Codec<'fragment, 'variable>) -> Self {
        let mut map = Self::new();
        map.insert("default", codec);
        map
    }

    pub fn insert(&mut self, name: &'fragment str, codec: Codec<'fragment, 'variable>) {
        self.map.insert(name, codec);
    }

    pub fn get(&self, name: &str) -> Option<&Codec<'fragment, 'variable>> {
        self.map.get(name)
    }
}
