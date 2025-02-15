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

use crate::codec::{ByteCodecBE, ByteCodecLE, Codec};

pub struct BitCodecLE {
    bit_size: usize,
    bit_offset: usize,
}

impl BitCodecLE {
    pub fn new(bit_offset: usize, bit_size: usize) -> Self {
        Self {
            bit_size,
            bit_offset
        }
    }
}

impl Codec for BitCodecLE {
    fn read(&self, buffer: &[u8]) -> u64 {
        let value = ByteCodecLE.read(buffer);
        let mask = (1 << self.bit_size) - 1;
        (value >> self.bit_offset) & mask
    }

    fn write(&self, buffer: &mut [u8], value: u64) {
        let mask = (1 << self.bit_size) - 1;
        let reset_mask = !(mask << self.bit_offset);
        let original = ByteCodecLE.read(buffer);
        let clean = original & reset_mask;
        let value = (value & mask) << self.bit_offset;
        let new = clean | value;
        ByteCodecLE.write(buffer, new);
    }
}

pub struct BitCodecBE {
    bit_size: usize,
    bit_offset: usize,
}

impl BitCodecBE {
    pub fn new(bit_offset: usize, bit_size: usize) -> Self {
        Self {
            bit_size,
            bit_offset
        }
    }
}

impl Codec for BitCodecBE {
    fn read(&self, buffer: &[u8]) -> u64 {
        let mask = (1 << self.bit_size) - 1;
        let value = ByteCodecBE.read(buffer);
        (value >> (8 - (self.bit_size % 8) - self.bit_offset)) & mask
    }

    fn write(&self, buffer: &mut [u8], value: u64) {
        let mask = (1 << self.bit_size) - 1;
        let reset_mask = !(mask << (8 - (self.bit_size % 8) - self.bit_offset));
        let original = ByteCodecBE.read(buffer);
        let clean = original & reset_mask;
        let value = (value & mask) << (8 - (self.bit_size % 8) - self.bit_offset);
        let new = clean | value;
        ByteCodecBE.write(buffer, new);
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::bits::{BitCodecBE, BitCodecLE};
    use crate::codec::Codec;

    #[test]
    fn little_endian() {
        let buffer = [0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(BitCodecLE::new(0, 32).read(&buffer[0..4]), 0xFFFFFFFF);
        assert_eq!(BitCodecLE::new(0, 1).read(&buffer[0..1]), 1);
        assert_eq!(BitCodecLE::new(0, 4).read(&buffer[0..1]), 0xF);
        assert_eq!(BitCodecLE::new(4, 4).read(&buffer[0..1]), 0xF);
    }

    #[test]
    fn big_endian() {
        let buffer = [0xAB, 0xF0];
        assert_eq!(BitCodecBE::new(0, 12).read(&buffer[0..2]), 0xABF);
        let mut buffer = [0x0, 0x0];
        BitCodecBE::new(0, 4).write(&mut buffer[0..1], 0xF);
        assert_eq!(BitCodecBE::new(0, 4).read(&buffer[0..1]), 0xF);
        BitCodecBE::new(0, 12).write(&mut buffer[0..2], 0xABF);
        assert_eq!(BitCodecBE::new(0, 12).read(&buffer[0..2]), 0xABF);
        BitCodecBE::new(1, 7).write(&mut buffer[1..2], 127);
        assert_eq!(BitCodecBE::new(1, 7).read(&buffer[1..2]), 127);
    }
}
