// Copyright (c) 2025, BlockProject 3D
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

use crate::field::codec::Codec;

pub struct ByteCodecLE;
pub struct ByteCodecBE;

impl Codec for ByteCodecLE {
    fn read(&self, buffer: &[u8]) -> u64 {
        let mut block: [u8; 8] = [0; 8];
        unsafe {
            std::ptr::copy_nonoverlapping(buffer.as_ptr(), block.as_mut_ptr(), buffer.len());
        }
        u64::from_le_bytes(block)
    }

    fn write(&self, buffer: &mut [u8], value: u64) {
        let block = value.to_le_bytes();
        unsafe {
            std::ptr::copy_nonoverlapping(block.as_ptr(), buffer.as_mut_ptr(), buffer.len());
        }
    }
}

impl Codec for ByteCodecBE {
    fn read(&self, buffer: &[u8]) -> u64 {
        let mut block: [u8; 8] = [0; 8];
        let offset = 8 - buffer.len();
        block[offset..buffer.len() + offset].copy_from_slice(buffer);
        u64::from_be_bytes(block)
    }

    fn write(&self, buffer: &mut [u8], value: u64) {
        let offset = 8 - buffer.len();
        let block = value.to_be_bytes();
        let motherfuckingrust = buffer.len();
        buffer.copy_from_slice(&block[offset..motherfuckingrust + offset]);
    }
}

#[cfg(test)]
mod tests {
    use crate::field::codec::bytes::ByteCodecLE;
    use crate::field::codec::Codec;

    #[test]
    fn basic() {
        let buffer = [0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(ByteCodecLE.read(&buffer[0..4]), 0xFFFFFFFF);
    }

    #[test]
    fn other() {
        let mut buffer = [0, 0, 0, 0];
        ByteCodecLE.write(&mut buffer[0..1], 128);
        assert_eq!(ByteCodecLE.read(&buffer[0..1]), 128);
        ByteCodecLE.write(&mut buffer[1..3], 4242);
        assert_eq!(ByteCodecLE.read(&buffer[1..3]), 4242);
    }
}
