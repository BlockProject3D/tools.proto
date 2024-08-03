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
use crate::util::FixedSize;
use crate::message::{Error, FromSlice, Message, WriteTo};
use crate::util::ToUsize;

macro_rules! impl_list_base {
    ($t: ident) => {
        impl<B, T, Item> $t<B, T, Item> {
            pub unsafe fn from_parts(data: B, len: usize) -> $t<B, T, Item> {
                $t {
                    data,
                    len,
                    useless: PhantomData::default(),
                    useless1: PhantomData::default()
                }
            }
        }

        impl<B: AsRef<[u8]>, T: ToUsize + WriteTo<Input = T>, Item> WriteTo for $t<B, T, Item> {
            type Input = $t<B, T, Item>;

            fn write_to<W: std::io::Write>(input: &Self::Input, mut out: W) -> Result<(), Error> {
                T::write_to(&T::from_usize(input.len), &mut out)?;
                out.write_all(input.data.as_ref())?;
                Ok(())
            }
        }

        impl<B: std::io::Write, T, Item: WriteTo<Input = Item>> $t<B, T, Item> {
            pub fn write_item(&mut self, item: &Item) -> Result<(), Error> {
                Item::write_to(item, &mut self.data)?;
                self.len += 1;
                Ok(())
            }

            pub fn write_items(&mut self, items: &[Item]) -> Result<(), Error> {
                for item in items {
                    self.write_item(item)?;
                }
                Ok(())
            }
        }

        impl<B: AsRef<[u8]>, T, Item> $t<B, T, Item> {
            pub fn as_ref(&self) -> $t<&[u8], T, Item> {
                unsafe { $t::from_parts(self.data.as_ref(), self.len) }
            }
        }
    };
}

#[derive(Copy, Clone, Debug)]
pub struct Array<B, T, Item> {
    data: B,
    len: usize,
    useless: PhantomData<T>,
    useless1: PhantomData<Item>
}

impl<B: AsRef<[u8]>, T, Item: FixedSize> Array<B, T, Item> {
    pub fn from_parts_checked(data: B, len: usize) -> Option<Array<B, T, Item>> {
        match data.as_ref().len() == len * Item::SIZE {
            true => Some(unsafe { Array::from_parts(data, len) }),
            false => None
        }
    }
}

impl_list_base!(Array);

impl<'a, B: AsRef<[u8]>, T, Item: FixedSize + From<&'a [u8]>> Array<B, T, Item> {
    pub fn get(&'a self, index: usize) -> Item {
        let start = index * Item::SIZE;
        let end = start + Item::SIZE;
        Item::from(&self.data.as_ref()[start..end])
    }

    pub fn iter(&'a self) -> impl Iterator<Item = Item> + 'a {
        self.data.as_ref().chunks(Item::SIZE).map(|v| Item::from(v))
    }
}

impl<'a, B: AsMut<[u8]>, T, Item: FixedSize + From<&'a mut [u8]>> Array<B, T, Item> {
    pub fn get_mut(&'a mut self, index: usize) -> Item {
        let start = index * Item::SIZE;
        let end = start + Item::SIZE;
        Item::from(&mut self.data.as_mut()[start..end])
    }

    pub fn iter_mut(&'a mut self) -> impl Iterator<Item = Item> + 'a {
        self.data.as_mut().chunks_mut(Item::SIZE).map(|v| Item::from(v))
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
        Ok(Message::new(total_size + control_size, unsafe { List::from_parts(data, len) }))
    }
}

pub struct UnsizedList<T, Item> {
    useless: PhantomData<T>,
    useless1: PhantomData<Item>
}

impl<'a, T: ReadBytes + ToUsize, Item> FromSlice<'a> for UnsizedList<T, Item> {
    type Output = List<&'a [u8], T, Item>;

    fn from_slice(slice: &'a [u8]) -> crate::message::Result<Message<Self::Output>> {
        let msg = T::from_slice(slice)?;
        let control_size = msg.size();
        let len = msg.into_inner().to_usize();
        let data = &slice[control_size..];
        Ok(Message::new(slice.len(), unsafe { List::from_parts(data, len) }))
    }
}
