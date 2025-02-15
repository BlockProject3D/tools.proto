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

use crate::buffer::BufferView;

pub enum Value {
    Unsigned(u64),
    Signed(i64),
    Float(f64),
    Bool(bool)
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Self::Unsigned(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Signed(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl Value {
    pub fn to_signed(&self) -> i64 {
        match self {
            Value::Unsigned(v) => *v as i64,
            Value::Signed(v) => *v,
            Value::Float(v) => *v as i64,
            Value::Bool(v) => if *v { 1 } else { 0 },
        }
    }

    pub fn to_unsigned(&self) -> u64 {
        match self {
            Value::Unsigned(v) => *v,
            Value::Signed(v) => *v as u64,
            Value::Float(v) => *v as u64,
            Value::Bool(v) => if *v { 1 } else { 0 },
        }
    }

    pub fn to_float(&self) -> f64 {
        match self {
            Value::Unsigned(v) => *v as f64,
            Value::Signed(v) => *v as f64,
            Value::Float(v) => *v,
            Value::Bool(v) => if *v { 1.0 } else { 0.0 },
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Value::Unsigned(v) => *v != 0,
            Value::Signed(v) => *v != 0,
            Value::Float(v) => *v != 0.0,
            Value::Bool(v) => *v
        }
    }
}

pub trait PrimitiveType {
    fn get_bits(&self, view: &BufferView) -> u64;
    fn set_bits(&self, view: &mut BufferView, bits: u64);
    fn get_raw(&self, view: &BufferView) -> Value;
    fn set_raw(&self, view: &mut BufferView, raw: Value);
    fn get(&self, view: &BufferView) -> Value;
    fn set(&self, view: &mut BufferView, view: Value);
}
