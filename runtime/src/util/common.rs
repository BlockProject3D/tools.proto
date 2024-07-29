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

use std::marker::PhantomData;
use bytesutil::ReadBytes;
use crate::FixedSize;
use crate::message::{Error, FromSlice, Message};

pub struct Optional<T>(PhantomData<T>);

impl<'a, T: FromSlice<'a, Output = T>> FromSlice<'a> for Optional<T> {
    type Output = Option<T>;

    fn from_slice(slice: &'a [u8]) -> Result<Message<Option<T>>, Error> {
        if slice.len() < 2 {
            Err(Error::Truncated)
        } else {
            let b = slice[0] > 0;
            if b {
                let msg = T::from_slice(&slice[1..])?;
                Ok(Message::new(msg.size() + 1, Some(msg.into_inner())))
            } else {
                Ok(Message::new(1, None))
            }
        }
    }
}

impl<'a, T: ReadBytes> FromSlice<'a> for T {
    type Output = Self;

    fn from_slice(slice: &'a [u8]) -> Result<Message<Self::Output>, Error> {
        let size = std::mem::size_of::<T>();
        if slice.len() < size {
            Err(Error::Truncated)
        } else {
            let value = T::read_bytes_be(slice);
            Ok(Message::new(size, value))
        }
    }
}

pub struct Buffer;

impl<'a> FromSlice<'a> for Buffer {
    type Output = &'a [u8];

    fn from_slice(slice: &'a [u8]) -> Result<Message<Self::Output>, Error> {
        Ok(Message::new(slice.len(), slice))
    }
}
