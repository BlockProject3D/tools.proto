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

use crate::message::{Error, FromSlice, Message, WriteTo};
use crate::util::ToUsize;
use std::io::Write;
use std::marker::PhantomData;

pub struct NullTerminatedString;

impl<'a> FromSlice<'a> for NullTerminatedString {
    type Output = &'a str;

    fn from_slice(slice: &'a [u8]) -> Result<Message<Self::Output>, Error> {
        let string = slice
            .iter()
            .enumerate()
            .find_map(|(id, v)| match *v == 0x0 {
                true => Some(id),
                false => None,
            })
            .map(|pos| std::str::from_utf8(&slice[0..pos]))
            .ok_or(Error::Truncated)?
            .map_err(|_| Error::InvalidUtf8)?;
        Ok(Message::new(string.len() + 1, string))
    }
}

impl<'a> WriteTo<'a> for NullTerminatedString {
    type Input = &'a str;

    fn write_to<W: Write>(input: &Self::Input, mut out: W) -> Result<(), Error> {
        out.write_all(input.as_bytes())?;
        out.write_all(&[0x0])?;
        Ok(())
    }
}

#[cfg(feature = "tokio")]
impl<'a> crate::message::WriteToAsync<'a> for NullTerminatedString {
    async fn write_to_async<W: tokio::io::AsyncWriteExt + Unpin>(input: &Self::Input, mut out: W) -> crate::message::Result<()> {
        out.write_all(input.as_bytes()).await?;
        out.write_all(&[0x0]).await?;
        Ok(())
    }
}

pub struct VarcharString<T>(PhantomData<T>);

impl<'a, T: FromSlice<'a, Output: ToUsize>> FromSlice<'a> for VarcharString<T> {
    type Output = &'a str;

    fn from_slice(slice: &'a [u8]) -> Result<Message<Self::Output>, Error> {
        let msg = T::from_slice(slice)?;
        let size = msg.size();
        let subslice = &slice[size..size + msg.into_inner().to_usize()];
        let string = std::str::from_utf8(subslice).map_err(|_| Error::InvalidUtf8)?;
        Ok(Message::new(size + string.len(), string))
    }
}

impl<'a, T: WriteTo<'a, Input: ToUsize>> WriteTo<'a> for VarcharString<T> {
    type Input = &'a str;

    fn write_to<W: Write>(input: &Self::Input, mut out: W) -> Result<(), Error> {
        T::write_to(&T::Input::from_usize(input.len()), &mut out)?;
        out.write_all(input.as_bytes())?;
        Ok(())
    }
}

#[cfg(feature = "tokio")]
impl<'a, T: crate::message::WriteToAsync<'a, Input: ToUsize>> crate::message::WriteToAsync<'a> for VarcharString<T> {
    async fn write_to_async<W: tokio::io::AsyncWriteExt + Unpin>(input: &Self::Input, mut out: W) -> crate::message::Result<()> {
        T::write_to_async(&T::Input::from_usize(input.len()), &mut out).await?;
        out.write_all(input.as_bytes()).await?;
        Ok(())
    }
}
