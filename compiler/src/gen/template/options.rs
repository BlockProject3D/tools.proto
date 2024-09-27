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

use crate::gen::template::functions::FunctionMap;
use std::collections::HashSet;

pub struct Options<'a> {
    function_map: FunctionMap<'a>,
    disabled_fragments: HashSet<&'a str>,
}

impl<'a> Default for Options<'a> {
    fn default() -> Self {
        Self {
            function_map: FunctionMap::default(),
            disabled_fragments: HashSet::new(),
        }
    }
}

impl<'a> Options<'a> {
    pub fn new(function_map: FunctionMap<'a>) -> Self {
        Self {
            function_map,
            disabled_fragments: HashSet::new(),
        }
    }

    pub fn functions_mut(&mut self) -> &mut FunctionMap<'a> {
        &mut self.function_map
    }

    pub fn functions(&self) -> &FunctionMap<'a> {
        &self.function_map
    }

    pub fn disable(&mut self, fragment: &'a str) -> &mut Self {
        self.disabled_fragments.insert(fragment);
        self
    }

    pub fn is_fragment_disabled(&self, name: &str) -> bool {
        self.disabled_fragments.contains(name)
    }
}
