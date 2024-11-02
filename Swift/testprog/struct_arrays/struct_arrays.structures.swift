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

import Foundation;
import BP3DProto;

public struct StructArraysBasic<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: StructArraysRawBasic<T>
    public static var size: Int { 58 }
    public init(_ data: T) {
        self._raw = StructArraysRawBasic(bin: StructArraysBinBasic(data: data));
    }
    public var raw: StructArraysRawBasic<T> {
        return _raw;
    }
    public var bin: StructArraysBinBasic<T> {
        return _raw.bin;
    }
}
extension StructArraysBasic<BP3DProto.DataBuffer> {
    public init() {
        self._raw = StructArraysRawBasic(bin: StructArraysBinBasic(data: BP3DProto.DataBuffer(size: 58)));
    }
}
public let SIZE_STRUCT_ARRAYS_BASIC: Int = 58;
extension StructArraysBasic: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = StructArraysBasic;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...58].toData());
    }
}
extension StructArraysBasic: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = StructArraysBasic;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 58 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 58, data: StructArraysBasic(bytes[...58]));
    }
}
extension StructArraysBasic where T: BP3DProto.Buffer {
    public var p1: UInt32 {
        self._raw.p1
    }
    public var p2: BP3DProto.ArrayCodec<T, BP3DProto.ByteCodecLE, UInt8> {
        BP3DProto.ArrayCodec(buffer: self._raw.bin.data[4...36], itemBitSize: 8)
    }
    public var p3: BP3DProto.ArrayCodec<T, BP3DProto.ByteCodecLE, Float32> {
        BP3DProto.ArrayCodec(buffer: self._raw.bin.data[36...52], itemBitSize: 32)
    }
    public var p4: BP3DProto.ArrayCodec<T, BP3DProto.ByteCodecLE, UInt32> {
        BP3DProto.ArrayCodec(buffer: self._raw.bin.data[52...58], itemBitSize: 24)
    }

}
extension StructArraysBasic where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setP1(_ value: UInt32) -> Self {
        self._raw.setP1(value);
        return self;
    }

}
/// Definition of the bits layout for Basic structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct StructArraysBinBasic<T> {
    var data: T
}
extension StructArraysBinBasic where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field p1: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public var p1: UInt32 {
        BP3DProto.ByteCodecLE.readAligned(UInt32.self, self.data[0...4])

    }

}
extension StructArraysBinBasic where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field p1: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public func setP1(_ value: UInt32) {
        var buffer = self.data[0...4];
        BP3DProto.ByteCodecLE.writeAligned(UInt32.self, &buffer, value: value);

    }

}
/// Definition of the raw layout for Basic structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct StructArraysRawBasic<T> {
    var bin: StructArraysBinBasic<T>
}
extension StructArraysRawBasic where T: BP3DProto.Buffer {
    /// Raw field accessor for p1: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public var p1: UInt32 {
        self.bin.p1

    }

}
extension StructArraysRawBasic where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for p1: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public func setP1(_ value: UInt32) {
        self.bin.setP1(value);

    }

}
