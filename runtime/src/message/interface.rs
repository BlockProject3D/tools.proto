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

use bp3d_util::simple_error;

simple_error! {
    pub Error {
        InvalidUtf8 => "invalid UTF-8 string",
        Truncated => "truncated input",
        InvalidUnionDiscriminant(usize) => "invalid union discriminant ({})",
        (impl From) Io(std::io::Error) => "io error: {}"
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Default, Copy, Clone, Debug)]
pub struct FieldOffset {
    pub start: usize,
    pub end: usize,
}

impl FieldOffset {
    pub fn size(&self) -> usize {
        self.end - self.start
    }
}

pub struct Message<T> {
    data: T,
    size: usize,
}

impl<T> Message<T> {
    pub fn new(size: usize, data: T) -> Self {
        Self { data, size }
    }

    pub fn into_inner(self) -> T {
        self.data
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn map<T1, F: FnOnce(T) -> T1>(self, f: F) -> Message<T1> {
        Message::new(self.size(), f(self.into_inner()))
    }
}

pub trait FromSlice<'a> {
    type Output: Sized;

    fn from_slice(slice: &'a [u8]) -> Result<Message<Self::Output>>;
    //fn copy_to_slice(&self, out_slice: &mut [u8]);
}

pub trait FromSliceWithOffsets<'a>: FromSlice<'a> {
    type Offsets: Sized;

    fn from_slice_with_offsets(slice: &'a [u8]) -> Result<Message<(Self::Output, Self::Offsets)>>;
}

pub trait WriteTo {
    type Input<'a>: Sized;

    fn write_to<W: std::io::Write>(input: &Self::Input<'_>, out: W) -> Result<()>;
}

#[cfg(feature = "tokio")]
pub trait WriteToAsync: WriteTo {
    fn write_to_async<W: tokio::io::AsyncWriteExt + Unpin>(input: &Self::Input<'_>, out: W) -> impl std::future::Future<Output = Result<()>>;
}

pub trait WriteSelf {
    fn write_self<W: std::io::Write>(&self, out: W) -> Result<()>;
    fn size(&self) -> Result<usize>;
}

#[cfg(feature = "tokio")]
pub trait WriteSelfAsync {
    fn write_self_async<W: tokio::io::AsyncWriteExt + Unpin>(&self, out: W) -> impl std::future::Future<Output=Result<()>>;
}

impl<'a, T: WriteTo<Input<'a>=T>> WriteSelf for T {
    fn write_self<W: std::io::Write>(&self, out: W) -> Result<()> {
        T::write_to(self, out)
    }

    fn size(&self) -> Result<usize> {
        crate::message::util::size_of(self)
    }
}

#[cfg(feature = "tokio")]
impl<T> WriteSelfAsync for T where for<'a> T: WriteToAsync<Input<'a> = T> {
    async fn write_self_async<W: tokio::io::AsyncWriteExt + Unpin>(&self, out: W) -> Result<()> {
        T::write_to_async(self, out).await
    }
}
