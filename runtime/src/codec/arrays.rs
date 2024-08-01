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
use std::ops::{BitAnd, BitOr, Shl, Shr};
use bytesutil::{ReadBytes, WriteBytes};
use crate::codec::ByteCodec;
use crate::util::ToUsize;

pub struct ArrayCodec<B, Item, const ITEM_BIT_SIZE: usize> {
    buffer: B,
    useless: PhantomData<Item>
}

impl<B, Item, const ITEM_BIT_SIZE: usize> ArrayCodec<B, Item, ITEM_BIT_SIZE> {
    pub fn new(buffer: B) -> Self {
        Self {
            buffer,
            useless: PhantomData::default()
        }
    }
}

impl<B: AsRef<[u8]>, Item: ReadBytes + ToUsize + BitAnd<Output = Item> + Shr<Output = Item>, const ITEM_BIT_SIZE: usize> ArrayCodec<B, Item, ITEM_BIT_SIZE> {
    pub fn get_raw(&self, index: usize) -> Item {
        let byte_size = ITEM_BIT_SIZE / 8;
        let pos = index * byte_size;
        let end = pos + byte_size;
        ByteCodec::new(&self.buffer.as_ref()[pos..end]).read::<Item>()
    }

    pub fn len(&self) -> usize {
        let byte_size = ITEM_BIT_SIZE / 8;
        self.buffer.as_ref().len() / byte_size
    }

    pub fn iter_raw(&self) -> impl Iterator<Item = Item> + '_ {
        let byte_size = ITEM_BIT_SIZE / 8;
        self.buffer.as_ref().chunks(byte_size).map(|v| ByteCodec::new(v).read::<Item>())
    }
}

impl<B: AsMut<[u8]>, Item: ReadBytes + WriteBytes + ToUsize + BitAnd<Output = Item> + BitOr<Output = Item> + Shr<Output = Item> + Shl<Output = Item>, const ITEM_BIT_SIZE: usize> ArrayCodec<B, Item, ITEM_BIT_SIZE> {
    pub fn set_raw(&mut self, index: usize, value: Item) {
        let byte_size = ITEM_BIT_SIZE / 8;
        let pos = index * byte_size;
        let end = pos + byte_size;
        ByteCodec::new(&mut self.buffer.as_mut()[pos..end]).write::<Item>(value);
    }
}
