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

use crate::custom_codec::Header;
use bp3d_proto::message::{FromBytesWithHeader, Message, ShapeHeader, WriteToWithHeader};
use std::io::Write;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContainerHeader<'a> {
    buffer: &'a [u8],
}

impl<'a> ContainerHeader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        ContainerHeader { buffer }
    }
}

impl<'a> FromBytesWithHeader<'a, Header<&'a [u8]>> for ContainerHeader<'a> {
    type Output = ContainerHeader<'a>;

    fn from_bytes_with_header(
        slice: &'a [u8],
        header: &Header<&'a [u8]>,
    ) -> bp3d_proto::message::Result<Message<Self::Output>> {
        let msg = ContainerHeader {
            buffer: &slice[..header.get_size() as _],
        };
        Ok(Message::new(msg.buffer.len(), msg))
    }
}

impl<'a> WriteToWithHeader<Header<&'a [u8]>> for ContainerHeader<'a> {
    type Input<'b> = ContainerHeader<'b>;

    fn write_to_with_header<W: Write>(
        input: &Self::Input<'_>,
        header: &Header<&'a [u8]>,
        mut out: W,
    ) -> bp3d_proto::message::Result<()> {
        out.write_all(&input.buffer[..header.get_size() as _])?;
        Ok(())
    }
}

impl<T: AsMut<[u8]>> ShapeHeader<Header<T>> for ContainerHeader<'_> {
    fn shape_header(&self, header: &mut Header<T>) -> bp3d_proto::message::Result<()> {
        header.set_size(self.buffer.len() as _);
        Ok(())
    }
}
