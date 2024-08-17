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

public struct Bits2Numbers<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    var data: T
    public static var size: Int { 4 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension Bits2Numbers<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 4)
    }
}
public let SIZE_BITS2_NUMBERS: Int = 4;
extension Bits2Numbers: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = Bits2Numbers;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...4].toData());
    }
}
extension Bits2Numbers: BP3DProto.FromSlice where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = Bits2Numbers;
    public static func from(slice: T) throws -> BP3DProto.Message<Output> {
        if slice.size < 4 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 4, data: Bits2Numbers(slice[...4]));
    }
}
extension Bits2Numbers where T: BP3DProto.Buffer {
    public var rawA: UInt8 {
        BP3DProto.BitCodecBE.readAligned(UInt8.self, self.data[0...1], bitOffset: 0, bitSize: 4)

    }
    public var a: Int8 {
        let rawValue = self.rawA;
        if rawValue > 7 {
            return -(Int8(((~rawValue) & 7) + 1));
        } else {
            return Int8(rawValue & 7);
        }
    }
    public var rawB: UInt8 {
        BP3DProto.BitCodecBE.readAligned(UInt8.self, self.data[0...1], bitOffset: 4, bitSize: 4)

    }
    public var b: UInt8 {
        self.rawB
    }
    public var rawC: UInt32 {
        BP3DProto.BitCodecBE.readUnaligned(UInt32.self, self.data[1...4], bitOffset: 0, bitSize: 17)

    }
    public var c: Int32 {
        let rawValue = self.rawC;
        if rawValue > 65535 {
            return -(Int32(((~rawValue) & 65535) + 1));
        } else {
            return Int32(rawValue & 65535);
        }
    }
    public var rawD: UInt8 {
        BP3DProto.BitCodecBE.readAligned(UInt8.self, self.data[3...4], bitOffset: 1, bitSize: 7)

    }
    public var d: UInt8 {
        self.rawD
    }

}
extension Bits2Numbers where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawA(_ value: UInt8) {
        var buffer = self.data[0...1];
        BP3DProto.BitCodecBE.writeAligned(UInt8.self, &buffer, bitOffset: 0, bitSize: 4, value: value);

    }
    @discardableResult
    public func setA(_ value: Int8) -> Self {
        self.setRawA(BP3DProto.transmute(value));

        return self;
    }
    public func setRawB(_ value: UInt8) {
        var buffer = self.data[0...1];
        BP3DProto.BitCodecBE.writeAligned(UInt8.self, &buffer, bitOffset: 4, bitSize: 4, value: value);

    }
    @discardableResult
    public func setB(_ value: UInt8) -> Self {
        self.setRawB(value);
        return self;
    }
    public func setRawC(_ value: UInt32) {
        var buffer = self.data[1...4];
        BP3DProto.BitCodecBE.writeUnaligned(UInt32.self, &buffer, bitOffset: 0, bitSize: 17, value: value);

    }
    @discardableResult
    public func setC(_ value: Int32) -> Self {
        self.setRawC(BP3DProto.transmute(value));

        return self;
    }
    public func setRawD(_ value: UInt8) {
        var buffer = self.data[3...4];
        BP3DProto.BitCodecBE.writeAligned(UInt8.self, &buffer, bitOffset: 1, bitSize: 7, value: value);

    }
    @discardableResult
    public func setD(_ value: UInt8) -> Self {
        self.setRawD(value);
        return self;
    }

}
