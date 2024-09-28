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

use crate::message::{Error, FromBytes, FromBytesWithOffsets, Message, WriteTo};
use bytesutil::ReadBytes;
use std::io::Write;
use std::marker::PhantomData;

pub struct Optional<T>(PhantomData<T>);

impl<'a, T: FromBytes<'a>> FromBytes<'a> for Optional<T> {
    type Output = Option<T::Output>;

    fn from_bytes(slice: &'a [u8]) -> Result<Message<Option<T::Output>>, Error> {
        if slice.is_empty() {
            Err(Error::Truncated)
        } else {
            let b = slice[0] > 0;
            if b {
                let msg = T::from_bytes(&slice[1..])?;
                Ok(Message::new(msg.size() + 1, Some(msg.into_inner())))
            } else {
                Ok(Message::new(1, None))
            }
        }
    }
}

impl<'a, T: FromBytes<'a> + FromBytesWithOffsets<'a>> FromBytesWithOffsets<'a> for Optional<T> {
    type Offsets = Option<T::Offsets>;

    fn from_bytes_with_offsets(slice: &'a [u8]) -> crate::message::Result<Message<(Self::Output, Self::Offsets)>> {
        if slice.is_empty() {
            Err(Error::Truncated)
        } else {
            let b = slice[0] > 0;
            if b {
                let msg = T::from_bytes_with_offsets(&slice[1..])?;
                let size = msg.size();
                let (msg, offsets) = msg.into_inner();
                Ok(Message::new(size + 1, (Some(msg), Some(offsets))))
            } else {
                Ok(Message::new(1, (None, None)))
            }
        }
    }
}

impl<T: WriteTo> WriteTo for Optional<T> {
    type Input<'a> = Option<T::Input<'a>>;

    fn write_to<W: Write>(input: &Self::Input<'_>, mut out: W) -> Result<(), Error> {
        match input {
            None => out.write_all(&[0x0])?,
            Some(v) => {
                out.write_all(&[0x1])?;
                T::write_to(v, out)?;
            }
        }
        Ok(())
    }
}

#[cfg(feature = "tokio")]
impl<T: crate::message::WriteToAsync> crate::message::WriteToAsync for Optional<T> {
    async fn write_to_async<W: tokio::io::AsyncWriteExt + Unpin>(
        input: &Self::Input<'_>,
        mut out: W,
    ) -> crate::message::Result<()> {
        match input {
            None => out.write_all(&[0x0]).await?,
            Some(v) => {
                out.write_all(&[0x1]).await?;
                T::write_to_async(v, out).await?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ValueLE<T>(PhantomData<T>);
#[derive(Debug, Copy, Clone)]
pub struct ValueBE<T>(PhantomData<T>);

impl<'a, T: ReadBytes> FromBytes<'a> for ValueLE<T> {
    type Output = T;

    fn from_bytes(slice: &'a [u8]) -> Result<Message<Self::Output>, Error> {
        let size = size_of::<T>();
        if slice.len() < size {
            Err(Error::Truncated)
        } else {
            let value = T::read_bytes_le(slice);
            Ok(Message::new(size, value))
        }
    }
}

impl<T: bytesutil::WriteTo> WriteTo for ValueLE<T> {
    type Input<'a> = T;

    fn write_to<W: Write>(input: &Self::Input<'_>, out: W) -> Result<(), Error> {
        input.write_to_le(out)?;
        Ok(())
    }
}

#[cfg(feature = "tokio")]
impl<T: bytesutil::WriteTo + bytesutil::WriteBytes> crate::message::WriteToAsync for ValueLE<T> {
    async fn write_to_async<W: tokio::io::AsyncWriteExt + Unpin>(
        input: &Self::Input<'_>,
        mut out: W,
    ) -> crate::message::Result<()> {
        let mut buffer = [0; 8];
        T::write_bytes_le(input, &mut buffer);
        out.write_all(&buffer[..size_of::<T>()]).await?;
        Ok(())
    }
}

impl<'a, T: ReadBytes> FromBytes<'a> for ValueBE<T> {
    type Output = T;

    fn from_bytes(slice: &'a [u8]) -> Result<Message<Self::Output>, Error> {
        let size = size_of::<T>();
        if slice.len() < size {
            Err(Error::Truncated)
        } else {
            let value = T::read_bytes_be(slice);
            Ok(Message::new(size, value))
        }
    }
}

impl<T: bytesutil::WriteTo> WriteTo for ValueBE<T> {
    type Input<'a> = T;

    fn write_to<W: Write>(input: &Self::Input<'_>, out: W) -> Result<(), Error> {
        input.write_to_be(out)?;
        Ok(())
    }
}

#[cfg(feature = "tokio")]
impl<T: bytesutil::WriteTo + bytesutil::WriteBytes> crate::message::WriteToAsync for ValueBE<T> {
    async fn write_to_async<W: tokio::io::AsyncWriteExt + Unpin>(
        input: &Self::Input<'_>,
        mut out: W,
    ) -> crate::message::Result<()> {
        let mut buffer = [0; 8];
        T::write_bytes_be(input, &mut buffer);
        out.write_all(&buffer[..size_of::<T>()]).await?;
        Ok(())
    }
}
