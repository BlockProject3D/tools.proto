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

use crate::compiler::Protocol;
use std::borrow::Cow;
use std::collections::HashMap;

pub trait ImportResolver {
    fn get_protocol_by_name(&self, name: &str) -> Option<&Protocol>;
    fn get_full_type_path(&self, protocol: &str, type_name: &str) -> Option<String>;
}

#[derive(Clone, Debug)]
pub struct TypePathMap {
    type_path_by_name: HashMap<String, String>,
}

impl TypePathMap {
    pub fn new() -> Self {
        Self {
            type_path_by_name: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String, type_path: String) {
        self.type_path_by_name.insert(name, type_path);
    }

    pub fn get<'a>(&'a self, item_type: &'a str) -> &'a str {
        match self.type_path_by_name.get(item_type) {
            None => item_type,
            Some(v) => v,
        }
    }

    pub fn get_with_default_prefix<'a>(
        &'a self,
        item_type: &'a str,
        default_prefix: &str,
    ) -> Cow<'a, str> {
        match self.type_path_by_name.get(item_type) {
            None => Cow::Owned(format!("{default_prefix}{item_type}")),
            Some(v) => Cow::Borrowed(v),
        }
    }
}

impl ImportResolver for () {
    fn get_protocol_by_name(&self, _: &str) -> Option<&Protocol> {
        None
    }

    fn get_full_type_path(&self, _: &str, _: &str) -> Option<String> {
        None
    }
}
