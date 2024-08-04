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
use crate::message::{Error, FromSlice, Message, WriteTo};
use crate::message::util::list_base::impl_list_base;
use crate::util::ToUsize;

#[derive(Copy, Clone, Debug)]
pub struct List<B, T, Item> {
    data: B,
    len: usize,
    useless: PhantomData<T>,
    useless1: PhantomData<Item>
}

impl_list_base!(List);

impl<'a, T: ReadBytes + ToUsize, Item: FromSlice<'a, Output = Item>> FromSlice<'a> for List<&'a [u8], T, Item> {
    type Output = List<&'a [u8], T, Item>;

    fn from_slice(slice: &'a [u8]) -> Result<Message<Self::Output>, Error> {
        let msg = T::from_slice(slice)?;
        let control_size = msg.size();
        let len = msg.into_inner().to_usize();
        let data = &slice[control_size..];
        let mut total_size: usize = 0;
        for _ in 0..len {
            let msg = Item::from_slice(&data[total_size..])?;
            total_size += msg.size();
        }
        let data = &slice[control_size..control_size + total_size];
        Ok(Message::new(total_size + control_size, unsafe { List::from_raw_parts(data, len) }))
    }
}

pub struct Unsized<T, Item> {
    useless: PhantomData<T>,
    useless1: PhantomData<Item>
}

impl<'a, T: ReadBytes + ToUsize, Item> FromSlice<'a> for Unsized<T, Item> {
    type Output = List<&'a [u8], T, Item>;

    fn from_slice(slice: &'a [u8]) -> crate::message::Result<Message<Self::Output>> {
        let msg = T::from_slice(slice)?;
        let control_size = msg.size();
        let len = msg.into_inner().to_usize();
        let data = &slice[control_size..];
        Ok(Message::new(slice.len(), unsafe { List::from_raw_parts(data, len) }))
    }
}
