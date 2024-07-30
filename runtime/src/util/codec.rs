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
use crate::util::ToUsize;

pub struct ArrayCodec<B, Item, const ItemBitSize: usize> {
    buffer: B,
    useless: PhantomData<Item>
}

impl<B, Item, const ItemBitSize: usize> ArrayCodec<B, Item, ItemBitSize> {
    pub fn new(buffer: B) -> Self {
        Self {
            buffer,
            useless: PhantomData::default()
        }
    }
}

impl<B: AsRef<[u8]>, Item: ReadBytes + ToUsize + BitAnd<Output = Item> + Shr<Output = Item>, const ItemBitSize: usize> ArrayCodec<B, Item, ItemBitSize> {
    pub fn get_raw(&self, index: usize) -> Item {
        let byte_size = ItemBitSize / 8;
        let pos = index * byte_size;
        let end = pos + byte_size;
        Codec::new(&self.buffer.as_ref()[pos..end]).read::<Item, 0, ItemBitSize>()
    }

    pub fn len(&self) -> usize {
        let byte_size = ItemBitSize / 8;
        self.buffer.as_ref().len() / byte_size
    }

    pub fn iter_raw(&self) -> impl Iterator<Item = Item> + '_ {
        let byte_size = ItemBitSize / 8;
        self.buffer.as_ref().chunks(byte_size).map(|v| Codec::new(v).read::<Item, 0, ItemBitSize>())
    }
}

impl<B: AsMut<[u8]>, Item: ReadBytes + WriteBytes + ToUsize + BitAnd<Output = Item> + BitOr<Output = Item> + Shr<Output = Item> + Shl<Output = Item>, const ItemBitSize: usize> ArrayCodec<B, Item, ItemBitSize> {
    pub fn set_raw(&mut self, index: usize, value: Item) {
        let byte_size = ItemBitSize / 8;
        let pos = index * byte_size;
        let end = pos + byte_size;
        Codec::new(&mut self.buffer.as_mut()[pos..end]).write::<Item, 0, ItemBitSize>(value);
    }
}

pub struct Codec<B>(B);

impl<B> Codec<B> {
    pub fn new(slice: B) -> Self {
        Self(slice)
    }
}

impl<B: AsRef<[u8]>> Codec<B> {
    fn read_unchecked<T: ToUsize + ReadBytes + Shr<Output = T> + BitAnd<Output = T>, const BitOffset: usize, const BitSize: usize>(&self) -> T {
        let mask: usize = (1 << BitSize) - 1;
        let value = T::read_bytes_be(self.0.as_ref());
        (value >> T::from_usize(BitOffset)) & T::from_usize(mask)
    }

    pub fn read<T: ToUsize + ReadBytes + Shr<Output = T> + BitAnd<Output = T>, const BitOffset: usize, const BitSize: usize>(&self) -> T {
        if std::mem::size_of::<T>() != self.0.as_ref().len() {
            let mut data = [0; 8];
            data[..self.0.as_ref().len()].copy_from_slice(self.0.as_ref());
            Codec::new(&data).read_unchecked::<T, BitOffset, BitSize>()
        } else {
            self.read_unchecked::<T, BitOffset, BitSize>()
        }
    }
}

impl<B: AsMut<[u8]> + AsRef<[u8]>> Codec<B> {
    fn write_unchecked<T: ToUsize + ReadBytes + WriteBytes + Shl<Output = T> + Shr<Output = T> + BitAnd<Output = T> + BitOr<Output = T>, const BitOffset: usize, const BitSize: usize>(&mut self, value: T) {
        let mask: usize = (1 << BitSize) - 1;
        let reset_mask = !(mask << BitOffset);
        let original = T::read_bytes_be(self.0.as_ref());
        let clean = original & T::from_usize(reset_mask);
        let value = (value & T::from_usize(mask)) << T::from_usize(BitOffset);
        (clean | value).write_bytes_be(self.0.as_mut());
    }

    pub fn write<T: ToUsize + ReadBytes + WriteBytes + Shl<Output = T> + Shr<Output = T> + BitAnd<Output = T> + BitOr<Output = T>, const BitOffset: usize, const BitSize: usize>(&mut self, value: T) {
        if std::mem::size_of::<T>() != self.0.as_ref().len() {
            let mut data = [0; 8];
            data[..self.0.as_ref().len()].copy_from_slice(self.0.as_ref());
            Codec::new(&mut data).write_unchecked::<T, BitOffset, BitSize>(value);
            let motherfuckingrust = self.0.as_ref().len();
            self.0.as_mut().copy_from_slice(&data[..motherfuckingrust]);
        } else {
            self.write_unchecked::<T, BitOffset, BitSize>(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::Codec;

    #[test]
    fn basic() {
        let buffer = [0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(Codec::new(&buffer[0..4]).read::<u32, 0, 32>(), 0xFFFFFFFF);
        assert_eq!(Codec::new(&buffer[0..1]).read::<u8, 0, 1>(), 1);
        assert_eq!(Codec::new(&buffer[0..1]).read::<u8, 0, 4>(), 0xF);
        assert_eq!(Codec::new(&buffer[0..1]).read::<u8, 4, 4>(), 0xF);
    }
}
