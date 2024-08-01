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

use std::ops::{BitAnd, BitOr, Shl, Shr};
use bytesutil::{ReadBytes, WriteBytes};
use crate::util::ToUsize;

pub struct BitCodec<B>(B);

impl<B> BitCodec<B> {
    pub fn new(slice: B) -> Self {
        Self(slice)
    }
}

impl<B: AsRef<[u8]>> BitCodec<B> {
    pub unsafe fn read_aligned<T: ToUsize + ReadBytes + Shr<Output = T> + BitAnd<Output = T>, const BIT_OFFSET: usize, const BIT_SIZE: usize>(&self) -> T {
        let mask: usize = (1 << BIT_SIZE) - 1;
        let value = T::read_bytes_le(self.0.as_ref());
        (value >> T::from_usize(BIT_OFFSET)) & T::from_usize(mask)
    }

    pub unsafe fn read_unaligned<T: ToUsize + ReadBytes + Shr<Output = T> + BitAnd<Output = T>, const BIT_OFFSET: usize, const BIT_SIZE: usize>(&self) -> T {
        let mut data = [0; 8];
        data[..self.0.as_ref().len()].copy_from_slice(self.0.as_ref());
        BitCodec::new(&data).read_aligned::<T, BIT_OFFSET, BIT_SIZE>()
    }

    pub fn read<T: ToUsize + ReadBytes + Shr<Output = T> + BitAnd<Output = T>, const BIT_OFFSET: usize, const BIT_SIZE: usize>(&self) -> T {
        if size_of::<T>() != self.0.as_ref().len() {
            unsafe { self.read_unaligned::<T, BIT_OFFSET, BIT_SIZE>() }
        } else {
            unsafe { self.read_aligned::<T, BIT_OFFSET, BIT_SIZE>() }
        }
    }
}

impl<B: AsMut<[u8]> + AsRef<[u8]>> BitCodec<B> {
    pub unsafe fn write_aligned<T: ToUsize + ReadBytes + WriteBytes + Shl<Output = T> + Shr<Output = T> + BitAnd<Output = T> + BitOr<Output = T>, const BIT_OFFSET: usize, const BIT_SIZE: usize>(&mut self, value: T) {
        let mask: usize = (1 << BIT_SIZE) - 1;
        let reset_mask = !(mask << BIT_OFFSET);
        let original = T::read_bytes_le(self.0.as_ref());
        let clean = original & T::from_usize(reset_mask);
        let value = (value & T::from_usize(mask)) << T::from_usize(BIT_OFFSET);
        (clean | value).write_bytes_le(self.0.as_mut());
    }

    pub unsafe fn write_unaligned<T: ToUsize + ReadBytes + WriteBytes + Shl<Output = T> + Shr<Output = T> + BitAnd<Output = T> + BitOr<Output = T>, const BIT_OFFSET: usize, const BIT_SIZE: usize>(&mut self, value: T) {
        let mut data = [0; 8];
        data[..self.0.as_ref().len()].copy_from_slice(self.0.as_ref());
        BitCodec::new(&mut data).write_aligned::<T, BIT_OFFSET, BIT_SIZE>(value);
        let motherfuckingrust = self.0.as_ref().len();
        self.0.as_mut().copy_from_slice(&data[..motherfuckingrust]);
    }

    pub fn write<T: ToUsize + ReadBytes + WriteBytes + Shl<Output = T> + Shr<Output = T> + BitAnd<Output = T> + BitOr<Output = T>, const BIT_OFFSET: usize, const BIT_SIZE: usize>(&mut self, value: T) {
        if size_of::<T>() != self.0.as_ref().len() {
            unsafe { self.write_unaligned::<T, BIT_OFFSET, BIT_SIZE>(value); }
        } else {
            unsafe { self.write_aligned::<T, BIT_OFFSET, BIT_SIZE>(value); }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::BitCodec;

    #[test]
    fn basic() {
        let buffer = [0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(BitCodec::new(&buffer[0..4]).read::<u32, 0, 32>(), 0xFFFFFFFF);
        assert_eq!(BitCodec::new(&buffer[0..1]).read::<u8, 0, 1>(), 1);
        assert_eq!(BitCodec::new(&buffer[0..1]).read::<u8, 0, 4>(), 0xF);
        assert_eq!(BitCodec::new(&buffer[0..1]).read::<u8, 4, 4>(), 0xF);
    }
}
