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

mod scalar;

pub use scalar::*;

pub trait Size {
    const SIZE: usize;

    fn size(&self) -> usize {
        Self::SIZE
    }
}

pub trait MultiOptionExt {
    type Output;

    fn to_single(self) -> Option<Self::Output>;
}

impl<T, T1> MultiOptionExt for (Option<T>, Option<T1>) {
    type Output = (T, T1);

    fn to_single(self) -> Option<Self::Output> {
        let t = self.0?;
        let t1 = self.1?;
        Some((t, t1))
    }
}

#[macro_export]
macro_rules! transmute {
    (<f64, u64>($value: expr)) => {
        $value.to_bits()
    };
    (<u64, f64>($value: expr)) => {
        f64::from_bits($value)
    };
    (<f32, u32>($value: expr)) => {
        $value.to_bits()
    };
    (<u32, f32>($value: expr)) => {
        f32::from_bits($value)
    };
    (<$src: ty, $dst: ty>($value: expr)) => {
        unsafe { std::mem::transmute::<$src, $dst>($value) }
    };
}

/// This trait represents any structure type.
pub trait Wrap<T: AsRef<[u8]>>: Sized + Size {
    /// Wraps the given data buffer.
    ///
    /// # Arguments
    ///
    /// * `data`: the data buffer to wrap as this structure.
    ///
    /// returns: Self
    ///
    /// # Panics
    ///
    /// This function will panic if the passed data buffer is too small to store the entire
    /// structure.
    fn wrap(data: T) -> Self {
        if data.as_ref().len() < Self::SIZE {
            panic!("attempt to wrap a too small buffer");
        }
        unsafe { Self::wrap_unchecked(data) }
    }

    /// Wraps the given data buffer.
    ///
    /// # Arguments
    ///
    /// * `data`: the data buffer to wrap as this structure type.
    ///
    /// returns: Self
    ///
    /// # Safety
    ///
    /// This function assumes the size of the buffer passed in is always at least the size of this
    /// structure.
    unsafe fn wrap_unchecked(data: T) -> Self;
}
