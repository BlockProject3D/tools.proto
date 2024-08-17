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
    var data: T
    public static var size: Int { 58 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension StructArraysBasic<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 58)
    }
}
public let SIZE_STRUCT_ARRAYS_BASIC: Int = 58;
extension StructArraysBasic: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = StructArraysBasic;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...58].toData());
    }
}
extension StructArraysBasic: BP3DProto.FromSlice where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = StructArraysBasic;
    public static func from(slice: T) throws -> BP3DProto.Message<Output> {
        if slice.size < 58 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 58, data: StructArraysBasic(slice[...58]));
    }
}
extension StructArraysBasic where T: BP3DProto.Buffer {
    public var rawP1: UInt32 {
        BP3DProto.ByteCodecLE.readAligned(UInt32.self, self.data[0...4])

    }
    public var p1: UInt32 {
        self.rawP1
    }
    public var p2: BP3DProto.ArrayCodec<T, BP3DProto.ByteCodecLE, UInt8> {
        BP3DProto.ArrayCodec(buffer: self.data[4...36], itemBitSize: 8)
    }
    public var p3: BP3DProto.ArrayCodec<T, BP3DProto.ByteCodecLE, Float32> {
        BP3DProto.ArrayCodec(buffer: self.data[36...52], itemBitSize: 32)
    }
    public var p4: BP3DProto.ArrayCodec<T, BP3DProto.ByteCodecLE, UInt32> {
        BP3DProto.ArrayCodec(buffer: self.data[52...58], itemBitSize: 24)
    }

}
extension StructArraysBasic where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawP1(_ value: UInt32) {
        var buffer = self.data[0...4];
        BP3DProto.ByteCodecLE.writeAligned(UInt32.self, &buffer, value: value);

    }
    @discardableResult
    public func setP1(_ value: UInt32) -> Self {
        self.setRawP1(value);
        return self;
    }

}
