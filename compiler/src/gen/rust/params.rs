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

use std::collections::HashSet;

#[derive(Default, Debug)]
pub struct Params<'a> {
    pub(crate) enable_struct_to_mut: bool,
    pub(crate) enable_struct_dupe: HashSet<&'a str>,
    pub(crate) enable_write_async: bool,
    pub(crate) enable_union_set_discriminant: bool,
    pub(crate) enable_list_wrappers: bool,
    pub(crate) enable_message_offsets: bool,
    pub(crate) disable_read: HashSet<&'a str>,
    pub(crate) disable_write: HashSet<&'a str>,
}

impl<'a> Params<'a> {
    pub fn enable_struct_to_mut(mut self, flag: bool) -> Self {
        self.enable_struct_to_mut = flag;
        self
    }

    pub fn enable_struct_dupe(mut self, name: &'a str) -> Self {
        self.enable_struct_dupe.insert(name);
        self
    }

    pub fn enable_union_set_discriminant(mut self, flag: bool) -> Self {
        self.enable_union_set_discriminant = flag;
        self
    }

    pub fn enable_list_wrappers(mut self, flag: bool) -> Self {
        self.enable_list_wrappers = flag;
        self
    }

    pub fn enable_message_offsets(mut self, flag: bool) -> Self {
        self.enable_message_offsets = flag;
        self
    }

    pub fn enable_write_async(mut self, flag: bool) -> Self {
        self.enable_write_async = flag;
        self
    }

    pub fn disable_read(mut self, name: &'a str) -> Self {
        self.disable_read.insert(name);
        self
    }

    pub fn disable_write(mut self, name: &'a str) -> Self {
        self.disable_write.insert(name);
        self
    }
}
