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
    private var _raw: Bits2RawNumbers<T>
    public static var size: Int { 4 }
    public init(_ data: T) {
        self._raw = Bits2RawNumbers(bin: Bits2BinNumbers(data: data));
    }
    public var raw: Bits2RawNumbers<T> {
        return _raw;
    }
    public var bin: Bits2BinNumbers<T> {
        return _raw.bin;
    }
}
extension Bits2Numbers<BP3DProto.DataBuffer> {
    public init() {
        self._raw = Bits2RawNumbers(bin: Bits2BinNumbers(data: BP3DProto.DataBuffer(size: 4)));
    }
}
public let SIZE_BITS2_NUMBERS: Int = 4;
extension Bits2Numbers: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = Bits2Numbers;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...4].toData());
    }
}
extension Bits2Numbers: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = Bits2Numbers;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 4 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 4, data: Bits2Numbers(bytes[...4]));
    }
}
extension Bits2Numbers where T: BP3DProto.Buffer {
    public var a: Int8 {
        self._raw.a
    }
    public var b: UInt8 {
        self._raw.b
    }
    public var c: Int32 {
        self._raw.c
    }
    public var d: UInt8 {
        self._raw.d
    }

}
extension Bits2Numbers where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setA(_ value: Int8) -> Self {
        self._raw.setA(value);
        return self;
    }
    @discardableResult
    public func setB(_ value: UInt8) -> Self {
        self._raw.setB(value);
        return self;
    }
    @discardableResult
    public func setC(_ value: Int32) -> Self {
        self._raw.setC(value);
        return self;
    }
    @discardableResult
    public func setD(_ value: UInt8) -> Self {
        self._raw.setD(value);
        return self;
    }

}
/// Definition of the bits layout for Numbers structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct Bits2BinNumbers<T> {
    var data: T
}
extension Bits2BinNumbers where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field a: UInt8, big endian (bytes 0..1, bits 0..4).
    ///
    /// 
    public var a: UInt8 {
        BP3DProto.BitCodecBE.readAligned(UInt8.self, self.data[0...1], bitOffset: 0, bitSize: 4)

    }
    /// Bit pattern accessor for field b: UInt8, big endian (bytes 0..1, bits 4..8).
    ///
    /// 
    public var b: UInt8 {
        BP3DProto.BitCodecBE.readAligned(UInt8.self, self.data[0...1], bitOffset: 4, bitSize: 4)

    }
    /// Bit pattern accessor for field c: UInt32, big endian (bytes 1..4, bits 0..17).
    ///
    /// 
    public var c: UInt32 {
        BP3DProto.BitCodecBE.readUnaligned(UInt32.self, self.data[1...4], bitOffset: 0, bitSize: 17)

    }
    /// Bit pattern accessor for field d: UInt8, big endian (bytes 3..4, bits 1..8).
    ///
    /// 
    public var d: UInt8 {
        BP3DProto.BitCodecBE.readAligned(UInt8.self, self.data[3...4], bitOffset: 1, bitSize: 7)

    }

}
extension Bits2BinNumbers where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field a: UInt8, big endian (bytes 0..1, bits 0..4).
    ///
    /// 
    public func setA(_ value: UInt8) {
        var buffer = self.data[0...1];
        BP3DProto.BitCodecBE.writeAligned(UInt8.self, &buffer, bitOffset: 0, bitSize: 4, value: value);

    }
    /// Bit pattern setter for field b: UInt8, big endian (bytes 0..1, bits 4..8).
    ///
    /// 
    public func setB(_ value: UInt8) {
        var buffer = self.data[0...1];
        BP3DProto.BitCodecBE.writeAligned(UInt8.self, &buffer, bitOffset: 4, bitSize: 4, value: value);

    }
    /// Bit pattern setter for field c: UInt32, big endian (bytes 1..4, bits 0..17).
    ///
    /// 
    public func setC(_ value: UInt32) {
        var buffer = self.data[1...4];
        BP3DProto.BitCodecBE.writeUnaligned(UInt32.self, &buffer, bitOffset: 0, bitSize: 17, value: value);

    }
    /// Bit pattern setter for field d: UInt8, big endian (bytes 3..4, bits 1..8).
    ///
    /// 
    public func setD(_ value: UInt8) {
        var buffer = self.data[3...4];
        BP3DProto.BitCodecBE.writeAligned(UInt8.self, &buffer, bitOffset: 1, bitSize: 7, value: value);

    }

}
/// Definition of the raw layout for Numbers structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct Bits2RawNumbers<T> {
    var bin: Bits2BinNumbers<T>
}
extension Bits2RawNumbers where T: BP3DProto.Buffer {
    /// Raw field accessor for a: UInt8, big endian (bytes 0..1, bits 0..4).
    ///
    /// 
    public var a: Int8 {
        let rawValue = self.bin.a;
        if rawValue > 7 {
            return -(Int8(((~rawValue) & 7) + 1));
        } else {
            return Int8(rawValue & 7);
        }

    }
    /// Raw field accessor for b: UInt8, big endian (bytes 0..1, bits 4..8).
    ///
    /// 
    public var b: UInt8 {
        self.bin.b

    }
    /// Raw field accessor for c: UInt32, big endian (bytes 1..4, bits 0..17).
    ///
    /// 
    public var c: Int32 {
        let rawValue = self.bin.c;
        if rawValue > 65535 {
            return -(Int32(((~rawValue) & 65535) + 1));
        } else {
            return Int32(rawValue & 65535);
        }

    }
    /// Raw field accessor for d: UInt8, big endian (bytes 3..4, bits 1..8).
    ///
    /// 
    public var d: UInt8 {
        self.bin.d

    }

}
extension Bits2RawNumbers where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for a: UInt8, big endian (bytes 0..1, bits 0..4).
    ///
    /// 
    public func setA(_ value: Int8) {
        self.bin.setA(BP3DProto.transmute(value));

    }
    /// Raw field setter for b: UInt8, big endian (bytes 0..1, bits 4..8).
    ///
    /// 
    public func setB(_ value: UInt8) {
        self.bin.setB(value);

    }
    /// Raw field setter for c: UInt32, big endian (bytes 1..4, bits 0..17).
    ///
    /// 
    public func setC(_ value: Int32) {
        self.bin.setC(BP3DProto.transmute(value));

    }
    /// Raw field setter for d: UInt8, big endian (bytes 3..4, bits 1..8).
    ///
    /// 
    public func setD(_ value: UInt8) {
        self.bin.setD(value);

    }

}
