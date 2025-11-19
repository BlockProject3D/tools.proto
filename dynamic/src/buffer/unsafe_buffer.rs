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

use crate::buffer::bytes::Bytes;
use std::ops::{Range, RangeFrom, RangeTo};
use std::ptr::NonNull;

pub trait Index {
    type Output<'a>;
    fn index<'a>(self, buffer: &UnsafeBuffer<'a>) -> Self::Output<'a>;
}

impl Index for usize {
    type Output<'a> = u8;
    fn index<'a>(self, buffer: &UnsafeBuffer<'a>) -> Self::Output<'a> {
        match buffer {
            UnsafeBuffer::Owned(v) => v.index(self),
            UnsafeBuffer::Borrowed(v) => v[self],
        }
    }
}

macro_rules! impl_index {
    ($ty: ident) => {
        impl Index for $ty<usize> {
            type Output<'a> = UnsafeBuffer<'a>;

            fn index<'a>(self, buffer: &UnsafeBuffer<'a>) -> Self::Output<'a> {
                match buffer {
                    UnsafeBuffer::Owned(v) => UnsafeBuffer::Owned(v.index(self)),
                    UnsafeBuffer::Borrowed(v) => UnsafeBuffer::Borrowed(&v[self]),
                }
            }
        }
    };
}

impl_index!(Range);
impl_index!(RangeFrom);
impl_index!(RangeTo);

#[derive(Debug)]
pub enum UnsafeBuffer<'a> {
    Owned(Bytes),
    Borrowed(&'a [u8]),
}

impl<'a> UnsafeBuffer<'a> {
    #[inline(always)]
    pub fn from_copy(slice: &[u8]) -> UnsafeBuffer<'a> {
        UnsafeBuffer::Owned(Bytes::from_slice(slice))
    }

    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> UnsafeBuffer<'a> {
        UnsafeBuffer::Owned(Bytes::with_capacity(capacity))
    }

    pub unsafe fn copy(&mut self, slice: &[u8]) -> bool {
        match self {
            UnsafeBuffer::Owned(v) => v.copy(slice),
            UnsafeBuffer::Borrowed(_) => {
                *self = UnsafeBuffer::from_copy(slice);
                false
            }
        }
    }

    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            UnsafeBuffer::Owned(v) => v.as_bytes(),
            UnsafeBuffer::Borrowed(v) => v,
        }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        match self {
            UnsafeBuffer::Owned(v) => v.as_bytes_mut(),
            UnsafeBuffer::Borrowed(v) => {
                *self = UnsafeBuffer::from_copy(v);
                self.as_bytes_mut()
            }
        }
    }

    #[inline(always)]
    pub fn index<I: Index>(&self, index: I) -> I::Output<'a> {
        index.index(self)
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        match self {
            UnsafeBuffer::Owned(v) => v.len(),
            UnsafeBuffer::Borrowed(v) => v.len(),
        }
    }

    #[inline(always)]
    pub unsafe fn delete(&mut self) {
        match self {
            UnsafeBuffer::Owned(v) => v.delete(),
            _ => (),
        }
    }

    fn extend(&mut self, size: usize) -> (NonNull<u8>, bool) {
        if let UnsafeBuffer::Borrowed(v) = self {
            *self = UnsafeBuffer::from_copy(v);
        }
        match self {
            UnsafeBuffer::Owned(v) => {
                let alloc = unsafe { v.resize(size) }.map(|v| v == 0).unwrap_or(false);
                (v.as_ptr(), alloc)
            }
            UnsafeBuffer::Borrowed(_) => std::unreachable!(),
        }
    }

    pub fn append(&mut self, bytes: &[u8]) {
        let (ptr, _) = self.extend(self.len() + bytes.len());
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr.offset(self.len() as _).as_ptr(), bytes.len());
        }
    }

    pub fn fill_hex(&mut self, start: usize, hex: &str) -> Result<bool, crate::buffer::byte_buf::InvalidHex> {
        if hex.len() % 2 != 0 {
            return Err(crate::buffer::byte_buf::InvalidHex);
        }
        let old_len = self.len();
        let len = hex.len() / 2;
        let mut alloc = false;
        if start + len > self.len() {
            let (ptr, alloc1) = self.extend(len);
            if start > old_len {
                for i in start..old_len {
                    unsafe { ptr.offset(i as _).write(0) };
                }
            }
            alloc = alloc1;
        }
        let mut idx = start;
        for pair in hex.as_bytes().chunks_exact(2) {
            let byte = u8::from_str_radix(
                std::str::from_utf8(pair).map_err(|_| crate::buffer::byte_buf::InvalidHex)?,
                16,
            )
            .map_err(|_| crate::buffer::byte_buf::InvalidHex)?;
            self.as_bytes_mut()[idx] = byte;
            idx += 1;
        }
        Ok(alloc)
    }
}
