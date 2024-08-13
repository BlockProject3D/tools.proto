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

use crate::util::ToUsize;
use bytesutil::{ReadBytes, WriteBytes};
use std::ops::{BitAnd, BitOr, Shl, Shr};

pub trait BitCodec {
    unsafe fn read_aligned<
        T: ToUsize + ReadBytes + Shr<Output = T> + BitAnd<Output = T>,
        const BIT_OFFSET: usize,
        const BIT_SIZE: usize,
    >(
        buffer: &[u8],
    ) -> T;

    fn read_unaligned<
        T: ToUsize + ReadBytes + Shr<Output = T> + BitAnd<Output = T>,
        const BIT_OFFSET: usize,
        const BIT_SIZE: usize,
    >(
        buffer: &[u8],
    ) -> T {
        let mut data = [0; 8];
        data[..buffer.len()].copy_from_slice(buffer);
        unsafe { Self::read_aligned::<T, BIT_OFFSET, BIT_SIZE>(&data) }
    }

    fn read<
        T: ToUsize + ReadBytes + Shr<Output = T> + BitAnd<Output = T>,
        const BIT_OFFSET: usize,
        const BIT_SIZE: usize,
    >(
        buffer: &[u8],
    ) -> T {
        if size_of::<T>() != buffer.len() {
            Self::read_unaligned::<T, BIT_OFFSET, BIT_SIZE>(buffer)
        } else {
            unsafe { Self::read_aligned::<T, BIT_OFFSET, BIT_SIZE>(buffer) }
        }
    }

    unsafe fn write_aligned<
        T: ToUsize
            + ReadBytes
            + WriteBytes
            + Shl<Output = T>
            + Shr<Output = T>
            + BitAnd<Output = T>
            + BitOr<Output = T>,
        const BIT_OFFSET: usize,
        const BIT_SIZE: usize,
    >(
        buffer: &mut [u8],
        value: T,
    );

    fn write_unaligned<
        T: ToUsize
            + ReadBytes
            + WriteBytes
            + Shl<Output = T>
            + Shr<Output = T>
            + BitAnd<Output = T>
            + BitOr<Output = T>,
        const BIT_OFFSET: usize,
        const BIT_SIZE: usize,
    >(
        buffer: &mut [u8],
        value: T,
    ) {
        let mut data = [0; 8];
        data[..buffer.len()].copy_from_slice(buffer);
        unsafe { Self::write_aligned::<T, BIT_OFFSET, BIT_SIZE>(&mut data, value) };
        let motherfuckingrust = buffer.len();
        buffer.copy_from_slice(&data[..motherfuckingrust]);
    }

    fn write<
        T: ToUsize
            + ReadBytes
            + WriteBytes
            + Shl<Output = T>
            + Shr<Output = T>
            + BitAnd<Output = T>
            + BitOr<Output = T>,
        const BIT_OFFSET: usize,
        const BIT_SIZE: usize,
    >(
        buffer: &mut [u8],
        value: T,
    ) {
        if size_of::<T>() != buffer.len() {
            Self::write_unaligned::<T, BIT_OFFSET, BIT_SIZE>(buffer, value);
        } else {
            unsafe { Self::write_aligned::<T, BIT_OFFSET, BIT_SIZE>(buffer, value) };
        }
    }
}

pub struct BitCodecLE;
pub struct BitCodecBE;

impl BitCodec for BitCodecLE {
    unsafe fn read_aligned<
        T: ToUsize + ReadBytes + Shr<Output = T> + BitAnd<Output = T>,
        const BIT_OFFSET: usize,
        const BIT_SIZE: usize,
    >(
        buffer: &[u8],
    ) -> T {
        let mask: usize = (1 << BIT_SIZE) - 1;
        let value = T::read_bytes_le(buffer);
        (value >> T::from_usize(BIT_OFFSET)) & T::from_usize(mask)
    }

    unsafe fn write_aligned<
        T: ToUsize
            + ReadBytes
            + WriteBytes
            + Shl<Output = T>
            + Shr<Output = T>
            + BitAnd<Output = T>
            + BitOr<Output = T>,
        const BIT_OFFSET: usize,
        const BIT_SIZE: usize,
    >(
        buffer: &mut [u8],
        value: T,
    ) {
        let mask: usize = (1 << BIT_SIZE) - 1;
        let reset_mask = !(mask << BIT_OFFSET);
        let original = T::read_bytes_le(buffer);
        let clean = original & T::from_usize(reset_mask);
        let value = (value & T::from_usize(mask)) << T::from_usize(BIT_OFFSET);
        (clean | value).write_bytes_le(buffer);
    }
}

impl BitCodec for BitCodecBE {
    unsafe fn read_aligned<
        T: ToUsize + ReadBytes + Shr<Output = T> + BitAnd<Output = T>,
        const BIT_OFFSET: usize,
        const BIT_SIZE: usize,
    >(
        buffer: &[u8],
    ) -> T {
        let mask: usize = (1 << BIT_SIZE) - 1;
        let value = T::read_bytes_be(buffer);
        (value >> T::from_usize(8 - (BIT_SIZE % 8) - BIT_OFFSET)) & T::from_usize(mask)
    }

    unsafe fn write_aligned<
        T: ToUsize
            + ReadBytes
            + WriteBytes
            + Shl<Output = T>
            + Shr<Output = T>
            + BitAnd<Output = T>
            + BitOr<Output = T>,
        const BIT_OFFSET: usize,
        const BIT_SIZE: usize,
    >(
        buffer: &mut [u8],
        value: T,
    ) {
        let mask: usize = (1 << BIT_SIZE) - 1;
        let reset_mask = !(mask << BIT_OFFSET);
        let original = T::read_bytes_be(buffer);
        let clean = original & T::from_usize(reset_mask);
        let value = (value & T::from_usize(mask)) << T::from_usize(8 - (BIT_SIZE % 8) - BIT_OFFSET);
        (clean | value).write_bytes_be(buffer);
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::{BitCodec, BitCodecBE, BitCodecLE};

    #[test]
    fn little_endian() {
        let buffer = [0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(BitCodecLE::read::<u32, 0, 32>(&buffer[0..4]), 0xFFFFFFFF);
        assert_eq!(BitCodecLE::read::<u8, 0, 1>(&buffer[0..1]), 1);
        assert_eq!(BitCodecLE::read::<u8, 0, 4>(&buffer[0..1]), 0xF);
        assert_eq!(BitCodecLE::read::<u8, 4, 4>(&buffer[0..1]), 0xF);
    }

    #[test]
    fn big_endian() {
        let buffer = [0xAB, 0xF0];
        assert_eq!(BitCodecBE::read::<u16, 0, 12>(&buffer[0..2]), 0xABF);
        let mut buffer = [0x0, 0x0];
        BitCodecBE::write::<u16, 0, 12>(&mut buffer[0..2], 0xABF);
        assert_eq!(BitCodecBE::read::<u16, 0, 12>(&buffer[0..2]), 0xABF);
    }
}
