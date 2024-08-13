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

use crate::codec::ByteCodec;
use bytesutil::{ReadBytes, WriteBytes};
use std::marker::PhantomData;

pub struct ArrayCodec<B, Item, C, const ITEM_BIT_SIZE: usize> {
    buffer: B,
    useless: PhantomData<Item>,
    useless1: PhantomData<C>,
}

impl<B, Item, C, const ITEM_BIT_SIZE: usize> ArrayCodec<B, Item, C, ITEM_BIT_SIZE> {
    pub fn new(buffer: B) -> Self {
        Self {
            buffer,
            useless: PhantomData,
            useless1: PhantomData,
        }
    }
}

impl<B: AsRef<[u8]>, Item: ReadBytes, C: ByteCodec, const ITEM_BIT_SIZE: usize>
    ArrayCodec<B, Item, C, ITEM_BIT_SIZE>
{
    pub fn get_raw(&self, index: usize) -> Item {
        let byte_size = ITEM_BIT_SIZE / 8;
        let pos = index * byte_size;
        let end = pos + byte_size;
        C::read::<Item>(&self.buffer.as_ref()[pos..end])
    }

    pub fn len(&self) -> usize {
        let byte_size = ITEM_BIT_SIZE / 8;
        self.buffer.as_ref().len() / byte_size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter_raw(&self) -> impl Iterator<Item = Item> + '_ {
        let byte_size = ITEM_BIT_SIZE / 8;
        self.buffer.as_ref().chunks(byte_size).map(|v| C::read::<Item>(v))
    }
}

impl<B: AsMut<[u8]>, Item: WriteBytes, C: ByteCodec, const ITEM_BIT_SIZE: usize>
    ArrayCodec<B, Item, C, ITEM_BIT_SIZE>
{
    pub fn set_raw(&mut self, index: usize, value: Item) -> &mut Self {
        let byte_size = ITEM_BIT_SIZE / 8;
        let pos = index * byte_size;
        let end = pos + byte_size;
        C::write::<Item>(&mut self.buffer.as_mut()[pos..end], value);
        self
    }
}

impl<B: AsMut<[u8]>, C> AsMut<[u8]> for ArrayCodec<B, u8, C, 8> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.buffer.as_mut()
    }
}

impl<B: AsRef<[u8]>, C> AsRef<[u8]> for ArrayCodec<B, u8, C, 8> {
    fn as_ref(&self) -> &[u8] {
        self.buffer.as_ref()
    }
}
