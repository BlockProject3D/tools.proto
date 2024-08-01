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

pub struct ByteCodec<B>(B);

impl<B> ByteCodec<B> {
    pub fn new(slice: B) -> Self {
        Self(slice)
    }
}

impl<B: AsRef<[u8]>> ByteCodec<B> {
    pub unsafe fn read_aligned<T: ToUsize + ReadBytes + Shr<Output = T> + BitAnd<Output = T>>(&self) -> T {
        T::read_bytes_be(self.0.as_ref())
    }

    pub unsafe fn read_unaligned<T: ToUsize + ReadBytes + Shr<Output = T> + BitAnd<Output = T>>(&self) -> T {
        let mut data = [0; 8];
        data[..self.0.as_ref().len()].copy_from_slice(self.0.as_ref());
        ByteCodec::new(&data).read_aligned::<T>()
    }

    pub fn read<T: ToUsize + ReadBytes + Shr<Output = T> + BitAnd<Output = T>>(&self) -> T {
        if size_of::<T>() != self.0.as_ref().len() {
            unsafe { self.read_unaligned::<T>() }
        } else {
            unsafe { self.read_aligned::<T>() }
        }
    }
}

impl<B: AsMut<[u8]> + AsRef<[u8]>> ByteCodec<B> {
    pub unsafe fn write_aligned<T: ToUsize + ReadBytes + WriteBytes + Shl<Output = T> + Shr<Output = T> + BitAnd<Output = T> + BitOr<Output = T>>(&mut self, value: T) {
        value.write_bytes_be(self.0.as_mut());
    }

    pub unsafe fn write_unaligned<T: ToUsize + ReadBytes + WriteBytes + Shl<Output = T> + Shr<Output = T> + BitAnd<Output = T> + BitOr<Output = T>>(&mut self, value: T) {
        let mut data = [0; 8];
        data[..self.0.as_ref().len()].copy_from_slice(self.0.as_ref());
        ByteCodec::new(&mut data).write_aligned::<T>(value);
        let motherfuckingrust = self.0.as_ref().len();
        self.0.as_mut().copy_from_slice(&data[..motherfuckingrust]);
    }

    pub fn write<T: ToUsize + ReadBytes + WriteBytes + Shl<Output = T> + Shr<Output = T> + BitAnd<Output = T> + BitOr<Output = T>>(&mut self, value: T) {
        if size_of::<T>() != self.0.as_ref().len() {
            unsafe { self.write_unaligned::<T>(value); }
        } else {
            unsafe { self.write_aligned::<T>(value); }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::ByteCodec;

    #[test]
    fn basic() {
        let buffer = [0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(ByteCodec::new(&buffer[0..4]).read::<u32>(), 0xFFFFFFFF);
    }
}
