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
    /// Reads a value of type T from the buffer argument with a custom bit offset and size assuming
    /// the buffer size is greater or equal to the size of T.
    ///
    /// # Arguments
    ///
    /// * `buffer`: the buffer to read from.
    ///
    /// returns: T
    ///
    /// # Safety
    ///
    /// This function assumes that the length of the buffer passed in as argument is at least as
    /// large as the size of T. Currently, this relies on bytesutil which does not apply any
    /// optimization and as such passing a too small buffer will only panic, however a future
    /// optimization might remove the panic check from release builds, essentially causing UB in
    /// such build.
    unsafe fn read_aligned<
        T: ToUsize + ReadBytes + Shr<Output = T> + BitAnd<Output = T>,
        const BIT_OFFSET: usize,
        const BIT_SIZE: usize,
    >(
        buffer: &[u8],
    ) -> T;

    /// Reads a value of type T from the buffer argument with a custom bit offset and size assuming
    /// the buffer size is always less than the size of T. This is not unsafe as will always cause
    /// a copy into an 8 bytes buffer (the maximum size for T is 8).
    ///
    /// # Arguments
    ///
    /// * `buffer`: the buffer to read from.
    ///
    /// returns: T
    fn read_unaligned<
        T: ToUsize + ReadBytes + Shr<Output = T> + BitAnd<Output = T>,
        const BIT_OFFSET: usize,
        const BIT_SIZE: usize,
    >(
        buffer: &[u8],
    ) -> T;

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

    /// Writes a value of type T in the buffer argument with a custom bit offset and size assuming
    /// the buffer size is greater or equal to the size of T.
    ///
    /// # Arguments
    ///
    /// * `buffer`: the buffer to write to.
    /// * `value`: the value to write.
    ///
    /// # Safety
    ///
    /// This function assumes that the length of the buffer passed in as argument is at least as
    /// large as the size of T. Currently, this relies on bytesutil which does not apply any
    /// optimization and as such passing a too small buffer will only panic, however a future
    /// optimization might remove the panic check from release builds, essentially causing UB in
    /// such build.
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
    );

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
    fn read_unaligned<
        T: ToUsize + ReadBytes + Shr<Output = T> + BitAnd<Output = T>,
        const BIT_OFFSET: usize,
        const BIT_SIZE: usize,
    >(
        buffer: &[u8],
    ) -> T {
        let offset = size_of::<T>() - buffer.len();
        let mut data = [0; 8];
        data[offset..buffer.len() + offset].copy_from_slice(buffer);
        unsafe { Self::read_aligned::<T, BIT_OFFSET, BIT_SIZE>(&data) }
    }

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
        let offset = size_of::<T>() - buffer.len();
        let mut data = [0; 8];
        data[offset..buffer.len() + offset].copy_from_slice(buffer);
        unsafe { Self::write_aligned::<T, BIT_OFFSET, BIT_SIZE>(&mut data, value) };
        let motherfuckingrust = buffer.len();
        buffer.copy_from_slice(&data[offset..motherfuckingrust + offset]);
    }

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
        let reset_mask = !(mask << 8 - (BIT_SIZE % 8) - BIT_OFFSET);
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
        BitCodecBE::write::<u8, 0, 4>(&mut buffer[0..1], 0xF);
        assert_eq!(BitCodecBE::read::<u8, 0, 4>(&buffer[0..1]), 0xF);
        BitCodecBE::write::<u16, 0, 12>(&mut buffer[0..2], 0xABF);
        assert_eq!(BitCodecBE::read::<u16, 0, 12>(&buffer[0..2]), 0xABF);
        BitCodecBE::write::<u8, 1, 7>(&mut buffer[1..2], 127);
        assert_eq!(BitCodecBE::read::<u8, 1, 7>(&buffer[1..2]), 127);
    }
}
