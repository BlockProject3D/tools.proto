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
use std::slice::{Chunks, ChunksMut};
use bytesutil::ReadBytes;
use crate::message::{Error, FromSlice, Message};
use crate::message::util::list_base::impl_list_base;
use crate::util::{FixedSize, ToUsize};
use crate::message::WriteTo;

#[derive(Copy, Clone, Debug)]
pub struct Array<B, T, Item> {
    data: B,
    len: usize,
    useless: PhantomData<T>,
    useless1: PhantomData<Item>
}

impl_list_base!(Array);

pub struct IterMut<'a, Item> {
    data: ChunksMut<'a, u8>,
    useless: PhantomData<Item>
}

impl<'a, Item: From<&'a mut [u8]>> Iterator for IterMut<'a, Item> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(Item::from)
    }
}

pub struct Iter<'a, Item> {
    data: Chunks<'a, u8>,
    useless: PhantomData<Item>
}

impl<'a, Item: From<&'a [u8]>> Iterator for Iter<'a, Item> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(Item::from)
    }
}

impl<B: AsRef<[u8]>, T, Item: FixedSize> Array<B, T, Item> {
    pub fn from_parts(data: B, len: usize) -> Option<Array<B, T, Item>> {
        match data.as_ref().len() == len * Item::SIZE {
            true => Some(unsafe { Array::from_raw_parts(data, len) }),
            false => None
        }
    }
}

impl<'a, B: AsRef<[u8]>, T, I> Array<B, T, I> {
    pub fn get<Item: FixedSize + From<&'a [u8]>>(&'a self, index: usize) -> Item {
        let start = index * Item::SIZE;
        let end = start + Item::SIZE;
        Item::from(&self.data.as_ref()[start..end])
    }

    pub fn iter<Item: FixedSize + From<&'a [u8]>>(&'a self) -> Iter<'a, Item> {
        Iter {
            data: self.data.as_ref().chunks(Item::SIZE),
            useless: PhantomData::default()
        }
    }
}

impl<'a, B: AsMut<[u8]>, T, I> Array<B, T, I> {
    pub fn get_mut<Item: FixedSize + From<&'a mut [u8]>>(&'a mut self, index: usize) -> Item {
        let start = index * Item::SIZE;
        let end = start + Item::SIZE;
        Item::from(&mut self.data.as_mut()[start..end])
    }

    pub fn iter_mut<Item: FixedSize + From<&'a mut [u8]>>(&'a mut self) -> IterMut<'a, Item> {
        IterMut {
            data: self.data.as_mut().chunks_mut(Item::SIZE),
            useless: PhantomData::default()
        }
    }
}

impl<'a, T: ReadBytes + ToUsize, Item: FixedSize + FromSlice<'a, Output = Item>> FromSlice<'a> for Array<&'a [u8], T, Item> {
    type Output = Array<&'a [u8], T, Item>;

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
