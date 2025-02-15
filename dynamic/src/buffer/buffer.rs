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
    fn index<'a>(self, buffer: &Buffer<'a>) -> Self::Output<'a>;
}

impl Index for usize {
    type Output<'a> = u8;
    fn index<'a>(self, buffer: &Buffer<'a>) -> Self::Output<'a> {
        match buffer {
            Buffer::Owned(v) => v.index(self),
            Buffer::Borrowed(v) => v[self]
        }
    }
}

macro_rules! impl_index {
    ($ty: ident) => {
        impl Index for $ty<usize> {
            type Output<'a> = Buffer<'a>;

            fn index<'a>(self, buffer: &Buffer<'a>) -> Self::Output<'a> {
                match buffer {
                    Buffer::Owned(v) => Buffer::Owned(v.index(self)),
                    Buffer::Borrowed(v) => Buffer::Borrowed(&v[self])
                }
            }
        }
    };
}

impl_index!(Range);
impl_index!(RangeFrom);
impl_index!(RangeTo);

#[derive(Debug)]
pub enum Buffer<'a> {
    Owned(Bytes),
    Borrowed(&'a [u8])
}

impl<'a> Buffer<'a> {
    pub fn from_slice(slice: &'a [u8]) -> Buffer<'a> {
        Buffer::Borrowed(slice)
    }

    pub fn from_copy(slice: &[u8]) -> Buffer<'a> {
        Buffer::Owned(Bytes::from_slice(slice))
    }

    pub unsafe fn copy(&mut self, slice: &[u8]) -> bool {
        match self {
            Buffer::Owned(v) => v.copy(slice),
            Buffer::Borrowed(_) => {
                *self = Buffer::from_copy(slice);
                false
            }
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        match self {
            Buffer::Owned(v) => v.write(data),
            Buffer::Borrowed(_) => {
                let bytes = Bytes::from_slice(data);
                *self = Buffer::Owned(bytes);
            }
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Buffer::Owned(v) => v.as_bytes(),
            Buffer::Borrowed(v) => v
        }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        match self {
            Buffer::Owned(v) => v.as_bytes_mut(),
            Buffer::Borrowed(v) => {
                *self = Buffer::from_copy(v);
                self.as_bytes_mut()
            }
        }
    }

    pub fn index<I: Index>(&self, index: I) -> I::Output<'a> {
        index.index(self)
    }

    pub fn read(&self, data: &mut [u8]) {
        match self {
            Buffer::Owned(v) => v.read(data),
            Buffer::Borrowed(bytes) => data.copy_from_slice(bytes),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Buffer::Owned(v) => v.len(),
            Buffer::Borrowed(v) => v.len()
        }
    }

    pub unsafe fn delete(&mut self) {
        match self {
            Buffer::Owned(v) => v.delete(),
            _ => ()
        }
    }
}
