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

use std::ops::{Range, RangeFrom, RangeTo};
use crate::buffer::bytes::Bytes;

pub trait Index {
    type Output<'a>;
    fn index<'a>(self, buffer: &UnsafeBuffer<'a>) -> Self::Output<'a>;
}

impl Index for usize {
    type Output<'a> = u8;
    fn index<'a>(self, buffer: &UnsafeBuffer<'a>) -> Self::Output<'a> {
        match buffer {
            UnsafeBuffer::Owned(v) => v.index(self),
            UnsafeBuffer::Borrowed(v) => v[self]
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
                    UnsafeBuffer::Borrowed(v) => UnsafeBuffer::Borrowed(&v[self])
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
    Borrowed(&'a [u8])
}

impl<'a> UnsafeBuffer<'a> {
    pub fn from_copy(slice: &[u8]) -> UnsafeBuffer<'a> {
        UnsafeBuffer::Owned(Bytes::from_slice(slice))
    }

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

    pub fn as_bytes(&self) -> &[u8] {
        match self {
            UnsafeBuffer::Owned(v) => v.as_bytes(),
            UnsafeBuffer::Borrowed(v) => v
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

    pub fn index<I: Index>(&self, index: I) -> I::Output<'a> {
        index.index(self)
    }

    pub fn len(&self) -> usize {
        match self {
            UnsafeBuffer::Owned(v) => v.len(),
            UnsafeBuffer::Borrowed(v) => v.len()
        }
    }

    pub unsafe fn delete(&mut self) {
        match self {
            UnsafeBuffer::Owned(v) => v.delete(),
            _ => ()
        }
    }
}
