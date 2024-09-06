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

use crate::message::{FromSlice, Message, WriteSelf};

pub trait FromValue<T> {
    fn from_value(value: T) -> Self;
}

pub trait IntoUnion<U> {
    fn into_union(self) -> U;
}

pub trait UFromSlice<'a, D> {
    type Output: Sized;

    fn u_from_slice(slice: &'a [u8], discriminant: &D) -> crate::message::Result<Message<Self::Output>>;
}

pub trait UWriteTo<D> {
    type Input<'a>: Sized;

    fn u_write_to<W: std::io::Write>(input: &Self::Input<'_>, discriminant: &D, out: W) -> crate::message::Result<()>;
}

#[cfg(feature = "tokio")]
pub trait UWriteToAsync<D>: UWriteTo<D> {
    fn u_write_to_async<W: tokio::io::AsyncWriteExt + Unpin>(
        input: &Self::Input<'_>,
        discriminant: &D,
        out: W,
    ) -> impl std::future::Future<Output = crate::message::Result<()>>;
}

impl<D, T: WriteSelf> UWriteTo<D> for T {
    type Input<'b> = T;

    fn u_write_to<W: std::io::Write>(input: &Self::Input<'_>, _: &D, out: W) -> crate::message::Result<()> {
        input.write_self(out)
    }
}

impl<T, U: FromValue<T>> IntoUnion<U> for T {
    fn into_union(self) -> U {
        U::from_value(self)
    }
}

impl<'a, D, T: FromSlice<'a>> UFromSlice<'a, D> for T {
    type Output = T::Output;

    fn u_from_slice(slice: &'a [u8], _: &D) -> crate::message::Result<Message<Self::Output>> {
        T::from_slice(slice)
    }
}
