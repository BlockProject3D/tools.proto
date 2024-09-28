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

use crate::message::payload::list_base::impl_list_base;
use crate::message::WriteTo;
use crate::message::{Error, FromBytes, Message};
use crate::util::{Size, ToUsize, Wrap};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::slice::{Chunks, ChunksMut};

pub struct IterMut<'a, Item> {
    data: ChunksMut<'a, u8>,
    useless: PhantomData<Item>,
}

impl<'a, Item> IterMut<'a, Item> {
    pub fn new(chunks: ChunksMut<'a, u8>) -> Self {
        Self {
            data: chunks,
            useless: PhantomData,
        }
    }
}

impl<'a, Item: Wrap<&'a mut [u8]>> Iterator for IterMut<'a, Item> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|v| unsafe { Item::wrap_unchecked(v) })
    }
}

pub struct Iter<'a, Item> {
    data: Chunks<'a, u8>,
    useless: PhantomData<Item>,
}

impl<'a, Item> Iter<'a, Item> {
    pub fn new(chunks: Chunks<'a, u8>) -> Self {
        Self {
            data: chunks,
            useless: PhantomData,
        }
    }
}

impl<'a, Item: Wrap<&'a [u8]>> Iterator for Iter<'a, Item> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|v| unsafe { Item::wrap_unchecked(v) })
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Array<B, T, Item> {
    data: B,
    len: usize,
    useless: PhantomData<T>,
    useless1: PhantomData<Item>,
}

impl<B, T, Item> Array<B, T, Item> {
    /// Creates an array from raw parts.
    /// This function assumes that data has the number of items specified by len.
    ///
    /// # Arguments
    ///
    /// * `data`: the data buffer.
    /// * `len`: the number of items to be read from the buffer.
    ///
    /// # Safety
    ///
    /// A wrong length could result in UB where the array iterator, getter or setter attempts to
    /// slice out of bounds with a future optimization in release builds, currently it will only
    /// result in a panic.
    pub unsafe fn from_raw_parts(data: B, len: usize) -> Array<B, T, Item> {
        Array {
            data,
            len,
            useless: PhantomData,
            useless1: PhantomData,
        }
    }
}

impl<B: AsRef<[u8]>, T, Item: Size> Array<B, T, Item> {
    pub fn new(data: B) -> Array<B, T, Item> {
        let len = data.as_ref().len() / Item::SIZE;
        unsafe { Array::from_raw_parts(data, len) }
    }
}

impl_list_base!(Array);

impl<B: AsRef<[u8]>, T, Item> Array<B, T, Item> {
    pub fn to_ref<Item1>(&self) -> Array<&[u8], T, Item1> {
        unsafe { Array::from_raw_parts(self.data.as_ref(), self.len) }
    }
}

impl<B: AsRef<[u8]>, T, Item: Size> Array<B, T, Item> {
    pub fn from_parts(data: B, len: usize) -> Option<Array<B, T, Item>> {
        match data.as_ref().len() == len * Item::SIZE {
            true => Some(unsafe { Array::from_raw_parts(data, len) }),
            false => None,
        }
    }
}

impl<'a, B: AsRef<[u8]>, T, Item: Size> Array<B, T, Item> {
    pub fn get(&'a self, index: usize) -> Option<&'a [u8]> {
        if index >= self.len {
            None
        } else {
            let start = index * Item::SIZE;
            let end = start + Item::SIZE;
            Some(&self.data.as_ref()[start..end])
        }
    }

    pub fn iter(&'a self) -> Chunks<'a, u8> {
        self.data.as_ref().chunks(Item::SIZE)
    }
}

impl<'a, B: AsMut<[u8]>, T, Item: Size> Array<B, T, Item> {
    pub fn get_mut(&'a mut self, index: usize) -> Option<&'a mut [u8]> {
        if index >= self.len {
            None
        } else {
            let start = index * Item::SIZE;
            let end = start + Item::SIZE;
            Some(&mut self.data.as_mut()[start..end])
        }
    }

    pub fn iter_mut(&'a mut self) -> ChunksMut<'a, u8> {
        self.data.as_mut().chunks_mut(Item::SIZE)
    }
}

impl<B: AsRef<[u8]>, T, Item: Size> Index<usize> for Array<B, T, Item> {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("attempt to index item out of bounds, index={}, len={}", index, self.len)
        }
        let start = index * Item::SIZE;
        let end = start + Item::SIZE;
        &self.data.as_ref()[start..end]
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>, T, Item: Size> IndexMut<usize> for Array<B, T, Item> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("attempt to index item out of bounds, index={}, len={}", index, self.len)
        }
        let start = index * Item::SIZE;
        let end = start + Item::SIZE;
        &mut self.data.as_mut()[start..end]
    }
}

impl<'a, T: FromBytes<'a, Output: ToUsize>, Item: Size> FromBytes<'a> for Array<&'a [u8], T, Item> {
    type Output = Array<&'a [u8], T, Item>;

    fn from_bytes(slice: &'a [u8]) -> Result<Message<Self::Output>, Error> {
        let msg = T::from_bytes(slice)?;
        let control_size = msg.size();
        let len = msg.into_inner().to_usize();
        let total_size = control_size + (len * Item::SIZE);
        if slice.len() < total_size {
            Err(Error::Truncated)
        } else {
            let data = &slice[control_size..total_size];
            Ok(Message::new(
                total_size,
                Array {
                    data,
                    len,
                    useless: PhantomData,
                    useless1: PhantomData,
                },
            ))
        }
    }
}
