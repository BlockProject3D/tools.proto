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

import Foundation

public protocol BitCodec {
    static func readAligned<B: Buffer, T: Scalar>(_: T.Type, _ buffer: B, bitOffset: UInt, bitSize: UInt) -> T;

    static func readUnaligned<B: Buffer, T: Scalar>(_: T.Type, _ buffer: B, bitOffset: UInt, bitSize: UInt) -> T;

    static func read<B: Buffer, T: Scalar>(_: T.Type, _ buffer: B, bitOffset: UInt, bitSize: UInt) -> T;

    static func writeAligned<B: WritableBuffer, T: Scalar>(_: T.Type, _ buffer: inout B, bitOffset: UInt, bitSize: UInt, value: T) where B: Buffer;

    static func writeUnaligned<B: WritableBuffer, T: Scalar>(_: T.Type, _ buffer: inout B, bitOffset: UInt, bitSize: UInt, value: T) where B: Buffer;

    static func write<B: WritableBuffer, T: Scalar>(_: T.Type, _ buffer: inout B, bitOffset: UInt, bitSize: UInt, value: T) where B: Buffer;
}

extension BitCodec {
    public static func readUnaligned<B: Buffer, T: Scalar>(_ ty: T.Type, _ buffer: B, bitOffset: UInt, bitSize: UInt) -> T {
        var data: UInt64 = 0;
        return withUnsafeMutableBytes(of: &data, { ptr in
            buffer.copyTo(ptr: ptr, size: buffer.size);
            return readAligned(ty, DataBuffer(ptr), bitOffset: bitOffset, bitSize: bitSize);
        });
    }

    public static func read<B: Buffer, T: Scalar>(_ ty: T.Type, _ buffer: B, bitOffset: UInt, bitSize: UInt) -> T {
        if T.size != buffer.size {
            return readUnaligned(ty, buffer, bitOffset: bitOffset, bitSize: bitSize);
        } else {
            return readAligned(ty, buffer, bitOffset: bitOffset, bitSize: bitSize);
        }
    }

    public static func writeUnaligned<B: WritableBuffer, T: Scalar>(_ ty: T.Type, _ buffer: inout B, bitOffset: UInt, bitSize: UInt, value: T) where B: Buffer {
        var data: UInt64 = 0;
        return withUnsafeMutableBytes(of: &data, { ptr in
            buffer.copyTo(ptr: ptr, size: buffer.size);
            var useless = DataBuffer(ptr)
            writeAligned(ty, &useless, bitOffset: bitOffset, bitSize: bitSize, value: value);
            buffer.write(bytes: ptr);
        });
    }

    public static func write<B: WritableBuffer, T: Scalar>(_ ty: T.Type, _ buffer: inout B, bitOffset: UInt, bitSize: UInt, value: T) where B: Buffer {
        if T.size != buffer.size {
            writeUnaligned(ty, &buffer, bitOffset: bitOffset, bitSize: bitSize, value: value);
        } else {
            writeAligned(ty, &buffer, bitOffset: bitOffset, bitSize: bitSize, value: value);
        }
    }
}

public struct BitCodecLE: BitCodec {
    public static func readAligned<B, T>(_: T.Type, _ buffer: B, bitOffset: UInt, bitSize: UInt) -> T where B : Buffer, T : Scalar {
        let mask = (UInt(1) << bitSize) - 1;
        let value = T(fromBytesLE: buffer);
        return (value >> T(fromUInt: bitOffset)) & T(fromUInt: mask);
    }

    public static func writeAligned<B: WritableBuffer, T>(_: T.Type, _ buffer: inout B, bitOffset: UInt, bitSize: UInt, value: T) where B: Buffer, T : Scalar {
        let mask = (UInt(1) << bitSize) - 1;
        let reset_mask = ~(mask << bitOffset);
        let original = T(fromBytesLE: buffer);
        let clean = original & T(fromUInt: reset_mask);
        let value1 = (value & T(fromUInt: mask)) << T(fromUInt: bitOffset);
        buffer.write(bytes: (clean | value1).toBytesLE());
    }
}

public struct BitCodecBE: BitCodec {
    public static func readAligned<B, T>(_: T.Type, _ buffer: B, bitOffset: UInt, bitSize: UInt) -> T where B : Buffer, T : Scalar {
        let mask = (UInt(1) << bitSize) - 1;
        let value = T(fromBytesBE: buffer);
        return (value >> T(fromUInt: 8 - (bitSize % 8) - bitOffset)) & T(fromUInt: mask);
    }

    public static func writeAligned<B: WritableBuffer, T>(_: T.Type, _ buffer: inout B, bitOffset: UInt, bitSize: UInt, value: T) where B: Buffer, T : Scalar {
        let mask = (UInt(1) << bitSize) - 1;
        let reset_mask = ~(mask << bitOffset);
        let original = T(fromBytesBE: buffer);
        let clean = original & T(fromUInt: reset_mask);
        let value1 = (value & T(fromUInt: mask)) << T(fromUInt: 8 - (bitSize % 8) - bitOffset);
        buffer.write(bytes: (clean | value1).toBytesBE());
    }
}
