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

use std::error::Error;
use std::fmt::Display;
use crate::buffer::buffer::Buffer;
use crate::buffer::unsafe_buffer::UnsafeBuffer;

#[derive(Debug)]
pub struct InvalidHex;
impl Error for InvalidHex {}
impl Display for InvalidHex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("invalid hexadecimal string")
    }
}

pub struct ByteBuf {
    pub(super) unsafe_buffer: UnsafeBuffer<'static>
}

impl<'a, 'b> From<&'a Buffer<'b>> for ByteBuf {
    fn from(buffer: &'a Buffer<'b>) -> Self {
        Self {
            unsafe_buffer: UnsafeBuffer::from_copy(buffer.as_bytes())
        }
    }
}

impl<'a> From<&'a [u8]> for ByteBuf {
    fn from(buffer: &'a [u8]) -> Self {
        Self {
            unsafe_buffer: UnsafeBuffer::from_copy(buffer)
        }
    }
}

impl ByteBuf {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            unsafe_buffer: UnsafeBuffer::with_capacity(capacity)
        }
    }

    pub fn new() -> Self {
        Self {
            unsafe_buffer: UnsafeBuffer::Borrowed(&[])
        }
    }

    pub fn append(&mut self, bytes: &[u8]) {
        self.unsafe_buffer.append(bytes)
    }

    pub fn fill_hex(&mut self, start: usize, hex: &str) -> Result<(), InvalidHex> {
        self.unsafe_buffer.fill_hex(start, hex)?;
        Ok(())
    }

    pub fn from_hex(hex: &str) -> Result<Self, InvalidHex> {
        let mut buf = ByteBuf::with_capacity(hex.len() / 2);
        buf.unsafe_buffer.fill_hex(0, hex)?;
        Ok(buf)
    }

    pub fn to_hex(&self) -> String {
        let s = format!("{:02X?}", self.as_bytes()).replace(", ", "");
        String::from(&s[1..s.len() - 1])
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.unsafe_buffer.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.unsafe_buffer.len() == 0
    }

    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        self.unsafe_buffer.as_bytes()
    }

    #[inline(always)]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.unsafe_buffer.as_bytes_mut()
    }
}

impl Drop for ByteBuf {
    fn drop(&mut self) {
        unsafe { self.unsafe_buffer.delete() }
    }
}
