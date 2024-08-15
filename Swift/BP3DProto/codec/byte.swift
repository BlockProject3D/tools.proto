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

public protocol ByteCodec {
    static func readAligned<B: Buffer, T: Scalar>(_: T.Type, _ buffer: B) -> T;

    static func readUnaligned<B: Buffer, T: Scalar>(_: T.Type, _ buffer: B) -> T;

    static func read<B: Buffer, T: Scalar>(_: T.Type, _ buffer: B) -> T;

    static func writeAligned<B: WritableBuffer, T: Scalar>(_: T.Type, _ buffer: inout B, value: T) where B: Buffer;

    static func writeUnaligned<B: WritableBuffer, T: Scalar>(_: T.Type, _ buffer: inout B, value: T) where B: Buffer;

    static func write<B: WritableBuffer, T: Scalar>(_: T.Type, _ buffer: inout B, value: T) where B: Buffer;
}

extension ByteCodec {
    public static func readUnaligned<B: Buffer, T: Scalar>(_ ty: T.Type, _ buffer: B) -> T {
        var data: UInt64 = 0;
        return withUnsafeMutableBytes(of: &data, { ptr in
            buffer.copyTo(ptr: ptr, size: buffer.size);
            return readAligned(ty, PtrBuffer(ptr));
        });
    }

    public static func read<B: Buffer, T: Scalar>(_ ty: T.Type, _ buffer: B) -> T {
        if T.size != buffer.size {
            return readUnaligned(ty, buffer);
        } else {
            return readAligned(ty, buffer);
        }
    }

    public static func writeUnaligned<B: WritableBuffer, T: Scalar>(_ ty: T.Type, _ buffer: inout B, value: T) where B: Buffer {
        var data: UInt64 = 0;
        return withUnsafeMutableBytes(of: &data, { ptr in
            buffer.copyTo(ptr: ptr, size: buffer.size);
            var useless = MutPtrBuffer(ptr)
            writeAligned(ty, &useless, value: value);
            buffer.write(bytes: ptr);
        });
    }

    public static func write<B: WritableBuffer, T: Scalar>(_ ty: T.Type, _ buffer: inout B, value: T) where B: Buffer {
        if T.size != buffer.size {
            writeUnaligned(ty, &buffer, value: value);
        } else {
            writeAligned(ty, &buffer, value: value);
        }
    }
}

public struct ByteCodecLE: ByteCodec {
    public static func readAligned<B, T>(_: T.Type, _ buffer: B) -> T where B : Buffer, T : Scalar {
        return T(fromBytesLE: buffer);
    }

    public static func writeAligned<B: WritableBuffer, T>(_: T.Type, _ buffer: inout B, value: T) where B: Buffer, T : Scalar {
        buffer.write(bytes: value.toBytesLE());
    }
}

public struct ByteCodecBE: ByteCodec {
    public static func readAligned<B, T>(_: T.Type, _ buffer: B) -> T where B : Buffer, T : Scalar {
        return T(fromBytesBE: buffer);
    }

    public static func writeAligned<B: WritableBuffer, T>(_: T.Type, _ buffer: inout B, value: T) where B: Buffer, T : Scalar {
        buffer.write(bytes: value.toBytesBE());
    }
}
