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

use crate::field::primitive::{PrimitiveType, Value};

pub struct PrimitiveValue<'a> {
    bytes: &'a [u8],
    ty: &'a dyn PrimitiveType,
}

pub struct PrimitiveValueMut<'a> {
    bytes: &'a mut [u8],
    ty: &'a dyn PrimitiveType,
}

impl<'a> PrimitiveValue<'a> {
    pub fn new(bytes: &'a [u8], ty: &'a dyn PrimitiveType) -> Self {
        Self { bytes, ty }
    }

    pub fn get_bin(&self) -> u64 {
        self.ty.get_bin(self.bytes)
    }

    pub fn get_raw(&self) -> Value {
        self.ty.get_raw(self.bytes)
    }

    pub fn get(&self) -> Value {
        self.ty.get(self.bytes)
    }
}

impl<'a> PrimitiveValueMut<'a> {
    pub fn new(bytes: &'a mut [u8], ty: &'a dyn PrimitiveType) -> Self {
        Self { bytes, ty }
    }

    pub fn get_bin(&self) -> u64 {
        self.ty.get_bin(self.bytes)
    }

    pub fn get_raw(&self) -> Value {
        self.ty.get_raw(self.bytes)
    }

    pub fn get(&self) -> Value {
        self.ty.get(self.bytes)
    }

    pub fn set_bin(&mut self, val: u64) {
        self.ty.set_bin(self.bytes, val);
    }

    pub fn set_raw(&mut self, val: impl Into<Value>) {
        self.ty.set_raw(self.bytes, val.into());
    }

    pub fn set(&mut self, val: impl Into<Value>) {
        self.ty.set(self.bytes, val.into())
    }
}
