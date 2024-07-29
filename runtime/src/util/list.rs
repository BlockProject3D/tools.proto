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
use crate::message::{Error, FromSlice, Message, WriteTo};
use crate::util::ToUsize;

pub struct Array<'a, T, Item> {
    data: &'a [u8],
    len: usize,
    useless: PhantomData<T>,
    useless1: PhantomData<Item>
}

impl<'a, T: ReadBytes + ToUsize, Item: FixedSize + FromSlice<'a, Output = Item>> FromSlice<'a> for Array<'a, T, Item> {
    type Output = Array<'a, T, Item>;

    fn from_slice(slice: &'a [u8]) -> Result<Message<Self::Output>, Error> {
        let msg = T::from_slice(slice)?;
        let control_size = msg.size();
        let len = msg.into_inner().to_usize();
        let total_size = control_size + (len * Item::SIZE);
        if slice.len() < total_size {
            Err(Error::Truncated)
        } else {
            let data = &slice[control_size..total_size];
            Ok(Message::new(total_size, Array {
                data,
                len,
                useless: PhantomData::default(),
                useless1: PhantomData::default()
            }))
        }
    }
}

impl<'a, T: ToUsize + WriteTo<Input = T>, Item: FixedSize> WriteTo for Array<'a, T, Item> {
    type Input = Array<'a, T, Item>;

    fn write_to<W: std::io::Write>(input: &Self::Input, mut out: W) -> std::io::Result<()> {
        T::write_to(&T::from_usize(input.len), &mut out)?;
        out.write_all(input.data)
    }
}

pub struct List<'a, T, Item> {
    data: &'a [u8],
    len: usize,
    useless: PhantomData<T>,
    useless1: PhantomData<Item>
}

impl<'a, T: ReadBytes + ToUsize, Item: FromSlice<'a, Output = Item>> FromSlice<'a> for List<'a, T, Item> {
    type Output = List<'a, T, Item>;

    fn from_slice(slice: &'a [u8]) -> Result<Message<Self::Output>, Error> {
        let msg = T::from_slice(slice)?;
        let control_size = msg.size();
        let len = msg.into_inner().to_usize();
        let data = &slice[control_size..];
        Ok(Message::new(slice.len(), List {
            data,
            len,
            useless: PhantomData::default(),
            useless1: PhantomData::default()
        }))
    }
}

impl<'a, T: ToUsize + WriteTo<Input = T>, Item: FixedSize> WriteTo for List<'a, T, Item> {
    type Input = List<'a, T, Item>;

    fn write_to<W: std::io::Write>(input: &Self::Input, mut out: W) -> std::io::Result<()> {
        T::write_to(&T::from_usize(input.len), &mut out)?;
        out.write_all(input.data)
    }
}
