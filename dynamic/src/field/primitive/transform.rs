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

use crate::field::primitive::Value;

pub trait RawTransform {
    fn bits_to_raw(&self, bits: u64) -> Value;
    fn raw_to_bits(&self, value: Value) -> u64;
}

pub trait ViewTransform {
    fn raw_to_view(&self, raw: Value) -> Value;
    fn view_to_raw(&self, view: Value) -> Value;
}

pub struct SignedTransform {
    pub max_positive: u64,
}

impl RawTransform for SignedTransform {
    fn bits_to_raw(&self, bits: u64) -> Value {
        if bits > self.max_positive {
            Value::Signed(-((((!bits) & { self.max_positive }) + 1) as i64))
        } else {
            Value::Signed((bits & self.max_positive) as i64)
        }
    }

    fn raw_to_bits(&self, value: Value) -> u64 {
        i64::cast_unsigned(value.to_signed())
    }
}

pub struct BoolTransform;

impl RawTransform for BoolTransform {
    fn bits_to_raw(&self, bits: u64) -> Value {
        Value::Bool(bits != 0)
    }

    fn raw_to_bits(&self, value: Value) -> u64 {
        if value.to_bool() {
            1
        } else {
            0
        }
    }
}

pub struct NoneTransform;

impl RawTransform for NoneTransform {
    fn bits_to_raw(&self, bits: u64) -> Value {
        Value::Unsigned(bits)
    }

    fn raw_to_bits(&self, value: Value) -> u64 {
        value.to_unsigned()
    }
}

impl ViewTransform for NoneTransform {
    fn raw_to_view(&self, raw: Value) -> Value {
        raw
    }

    fn view_to_raw(&self, view: Value) -> Value {
        view
    }
}

pub struct Float32Transform;

impl RawTransform for Float32Transform {
    fn bits_to_raw(&self, bits: u64) -> Value {
        let value: f32 = f32::from_bits(bits as u32);
        Value::Float(value as _)
    }

    fn raw_to_bits(&self, value: Value) -> u64 {
        let raw: u32 = (value.to_float() as f32).to_bits();
        raw as u64
    }
}

pub struct Float64Transform;

impl RawTransform for Float64Transform {
    fn bits_to_raw(&self, bits: u64) -> Value {
        Value::Float(f64::from_bits(bits))
    }

    fn raw_to_bits(&self, value: Value) -> u64 {
        value.to_float().to_bits()
    }
}

pub struct FloatTransform {
    pub a: f64,
    pub b: f64,
    pub a_inv: f64,
    pub b_inv: f64,
}

impl ViewTransform for FloatTransform {
    fn raw_to_view(&self, raw: Value) -> Value {
        (raw.to_float() * self.a + self.b).into()
    }

    fn view_to_raw(&self, view: Value) -> Value {
        (view.to_float() * self.a_inv + self.b_inv).into()
    }
}
