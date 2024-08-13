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

use bytesutil::{ReadBytes, WriteBytes};

pub trait ByteCodec {
    unsafe fn read_aligned<T: ReadBytes>(buffer: &[u8]) -> T;

    fn read_unaligned<T: ReadBytes>(buffer: &[u8]) -> T {
        let mut data = [0; 8];
        data[..buffer.len()].copy_from_slice(buffer);
        unsafe { Self::read_aligned::<T>(&data) }
    }

    fn read<T: ReadBytes>(buffer: &[u8]) -> T {
        if size_of::<T>() != buffer.len() {
            Self::read_unaligned::<T>(buffer)
        } else {
            unsafe { Self::read_aligned::<T>(buffer) }
        }
    }

    unsafe fn write_aligned<T: WriteBytes>(buffer: &mut [u8], value: T);

    fn write_unaligned<T: WriteBytes>(buffer: &mut [u8], value: T) {
        let mut data = [0; 8];
        data[..buffer.len()].copy_from_slice(buffer);
        unsafe { Self::write_aligned::<T>(&mut data, value) };
        let motherfuckingrust = buffer.len();
        buffer.copy_from_slice(&data[..motherfuckingrust]);
    }

    fn write<T: WriteBytes>(buffer: &mut [u8], value: T) {
        if size_of::<T>() != buffer.len() {
            Self::write_unaligned::<T>(buffer, value);
        } else {
            unsafe { Self::write_aligned::<T>(buffer, value) };
        }
    }
}

pub struct ByteCodecBE;
pub struct ByteCodecLE;

impl ByteCodec for ByteCodecLE {
    unsafe fn read_aligned<T: ReadBytes>(buffer: &[u8]) -> T {
        T::read_bytes_le(buffer)
    }

    unsafe fn write_aligned<T: WriteBytes>(buffer: &mut [u8], value: T) {
        value.write_bytes_le(buffer);
    }
}

impl ByteCodec for ByteCodecBE {
    unsafe fn read_aligned<T: ReadBytes>(buffer: &[u8]) -> T {
        T::read_bytes_be(buffer)
    }

    unsafe fn write_aligned<T: WriteBytes>(buffer: &mut [u8], value: T) {
        value.write_bytes_be(buffer);
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::{ByteCodec, ByteCodecLE};

    #[test]
    fn basic() {
        let buffer = [0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(ByteCodecLE::read::<u32>(&buffer[0..4]), 0xFFFFFFFF);
    }
}
