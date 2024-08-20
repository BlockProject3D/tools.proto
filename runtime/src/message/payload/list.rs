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

use std::io::Write;
use crate::message::payload::list_base::impl_list_base;
use crate::message::{Error, FromSlice, FromSliceWithOffsets, Message, WriteTo};
use crate::util::ToUsize;
use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
pub struct List<B, T, Item> {
    data: B,
    len: usize,
    useless: PhantomData<T>,
    useless1: PhantomData<Item>,
}

impl<B, T, Item> List<B, T, Item> {
    /// Creates a list from raw parts.
    /// This function assumes that data has the number of items specified by len.
    /// A wrong length will simply cause an error (truncated) to be returned if the actual buffer
    /// has not enough bytes to contain all items.
    ///
    /// # Arguments
    ///
    /// * `data`: the data buffer.
    /// * `len`: the number of items to be read from the buffer.
    ///
    /// # Safety
    ///
    ///
    /// For all array types, (i.e. lists with fixed size items), a wrong length could result
    /// in UB where the array iterator, getter or setter attempts to slice out of bounds
    /// with a future optimization in release builds, currently it will result in a panic.
    pub fn from_raw_parts(data: B, len: usize) -> List<B, T, Item> {
        List {
            data,
            len,
            useless: PhantomData,
            useless1: PhantomData,
        }
    }

    pub fn new(data: B) -> List<B, T, Item> {
        List::from_raw_parts(data, 0)
    }
}

impl<B: AsRef<[u8]>, T, Item> List<B, T, Item> {
    pub fn to_ref<Item1>(&self) -> List<&[u8], T, Item1> {
        List::from_raw_parts(self.data.as_ref(), self.len)
    }
}

impl_list_base!(List);

impl<B: std::io::Write, T, I> List<B, T, I> {
    pub fn write_item<Item: WriteTo<Input = Item>>(&mut self, item: &Item) -> Result<(), Error> {
        Item::write_to(item, &mut self.data)?;
        self.len += 1;
        Ok(())
    }

    pub fn write_items<Item: WriteTo<Input = Item>>(&mut self, items: &[Item]) -> Result<(), Error> {
        for item in items {
            self.write_item(item)?;
        }
        Ok(())
    }
}

impl<'a, T: FromSlice<'a, Output: ToUsize>, Item: FromSlice<'a, Output = Item>> FromSlice<'a>
    for List<&'a [u8], T, Item>
{
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
        Ok(Message::new(total_size + control_size, List::from_raw_parts(data, len)))
    }
}

impl<B: AsRef<[u8]>, T, Item> List<B, T, Item> {
    pub fn iter(&self) -> Iter<Item> {
        Iter {
            data: self.data.as_ref(),
            len: self.len,
            useless: PhantomData,
        }
    }

    pub fn iter_offsets(&self) -> IterOffsets<Item> {
        IterOffsets {
            data: self.data.as_ref(),
            len: self.len,
            useless: PhantomData,
        }
    }
}

pub struct Unsized<T, Item> {
    useless: PhantomData<T>,
    useless1: PhantomData<Item>,
}

impl<'a, T: FromSlice<'a, Output: ToUsize>, Item> FromSlice<'a> for Unsized<T, Item> {
    type Output = List<&'a [u8], T, Item>;

    fn from_slice(slice: &'a [u8]) -> crate::message::Result<Message<Self::Output>> {
        let msg = T::from_slice(slice)?;
        let control_size = msg.size();
        let len = msg.into_inner().to_usize();
        let data = &slice[control_size..];
        Ok(Message::new(slice.len(), List::from_raw_parts(data, len)))
    }
}

pub struct Sized<B, T, S, Item> {
    useless: PhantomData<T>,
    useless1: PhantomData<S>,
    useless2: PhantomData<Item>,
    useless3: PhantomData<B>
}

impl<'a, B, T: FromSlice<'a, Output: ToUsize>, S: FromSlice<'a, Output: ToUsize>, Item> FromSlice<'a> for Sized<B, T, S, Item> {
    type Output = List<&'a [u8], T, Item>;

    fn from_slice(slice: &'a [u8]) -> crate::message::Result<Message<Self::Output>> {
        let msg = T::from_slice(slice)?;
        let mut control_size = msg.size();
        let len = msg.into_inner().to_usize();
        let msg = S::from_slice(&slice[control_size..])?;
        control_size += msg.size();
        let total_size = control_size + msg.into_inner().to_usize();
        let data = &slice[control_size..total_size];
        Ok(Message::new(total_size, List::from_raw_parts(data, len)))
    }
}

impl<B: AsRef<[u8]>, T: WriteTo<Input: ToUsize + std::marker::Sized>, S: WriteTo<Input: ToUsize + std::marker::Sized>, Item> WriteTo for Sized<B, T, S, Item> {
    type Input = List<B, T, Item>;

    fn write_to<W: Write>(input: &Self::Input, mut out: W) -> crate::message::Result<()> {
        T::write_to(&T::Input::from_usize(input.len), &mut out)?;
        S::write_to(&S::Input::from_usize(input.data.as_ref().len()), &mut out)?;
        out.write_all(input.data.as_ref())?;
        Ok(())
    }
}

pub struct Iter<'a, Item> {
    data: &'a [u8],
    len: usize,
    useless: PhantomData<Item>,
}

impl<'a, Item: FromSlice<'a, Output = Item>> Iterator for Iter<'a, Item> {
    type Item = crate::message::Result<Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        let msg = match Item::from_slice(self.data) {
            Err(e) => return Some(Err(e)),
            Ok(v) => v,
        };
        self.data = &self.data[msg.size()..];
        self.len -= 1;
        Some(Ok(msg.into_inner()))
    }
}

pub struct IterOffsets<'a, Item> {
    data: &'a [u8],
    len: usize,
    useless: PhantomData<Item>,
}

impl<'a, Item: FromSlice<'a, Output = Item> + FromSliceWithOffsets<'a>> Iterator for IterOffsets<'a, Item> {
    type Item = crate::message::Result<(Item, Item::Offsets)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        let msg = match Item::from_slice_with_offsets(self.data) {
            Err(e) => return Some(Err(e)),
            Ok(v) => v,
        };
        self.data = &self.data[msg.size()..];
        self.len -= 1;
        Some(Ok(msg.into_inner()))
    }
}
