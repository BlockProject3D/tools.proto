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

public struct StructsNumbers<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: StructsRawNumbers<T>
    public static var size: Int { 14 }
    public init(_ data: T) {
        self._raw = StructsRawNumbers(bin: StructsBinNumbers(data: data));
    }
    public var raw: StructsRawNumbers<T> {
        return _raw;
    }
    public var bin: StructsBinNumbers<T> {
        return _raw.bin;
    }
}
extension StructsNumbers<BP3DProto.DataBuffer> {
    public init() {
        self._raw = StructsRawNumbers(bin: StructsBinNumbers(data: BP3DProto.DataBuffer(size: 14)));
    }
}
public let SIZE_STRUCTS_NUMBERS: Int = 14;
extension StructsNumbers: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = StructsNumbers;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...14].toData());
    }
}
extension StructsNumbers: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = StructsNumbers;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 14 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 14, data: StructsNumbers(bytes[...14]));
    }
}
extension StructsNumbers where T: BP3DProto.Buffer {
    public var uA: UInt32 {
        self._raw.uA
    }
    public var bA: Bool {
        self._raw.bA
    }
    public var a: Int32 {
        self._raw.a
    }
    public var uB: UInt16 {
        self._raw.uB
    }
    public var b: Int16 {
        self._raw.b
    }
    public var uC: UInt8 {
        self._raw.uC
    }
    public var c: Int8 {
        self._raw.c
    }
    public var bC: Bool {
        self._raw.bC
    }

}
extension StructsNumbers where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setUA(_ value: UInt32) -> Self {
        self._raw.setUA(value);
        return self;
    }
    @discardableResult
    public func setBA(_ value: Bool) -> Self {
        self._raw.setBA(value);
        return self;
    }
    @discardableResult
    public func setA(_ value: Int32) -> Self {
        self._raw.setA(value);
        return self;
    }
    @discardableResult
    public func setUB(_ value: UInt16) -> Self {
        self._raw.setUB(value);
        return self;
    }
    @discardableResult
    public func setB(_ value: Int16) -> Self {
        self._raw.setB(value);
        return self;
    }
    @discardableResult
    public func setUC(_ value: UInt8) -> Self {
        self._raw.setUC(value);
        return self;
    }
    @discardableResult
    public func setC(_ value: Int8) -> Self {
        self._raw.setC(value);
        return self;
    }
    @discardableResult
    public func setBC(_ value: Bool) -> Self {
        self._raw.setBC(value);
        return self;
    }

}
/// Definition of the bits layout for Numbers structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct StructsBinNumbers<T> {
    var data: T
}
extension StructsBinNumbers where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field u_a: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public var uA: UInt32 {
        BP3DProto.ByteCodecLE.readAligned(UInt32.self, self.data[0...4])

    }
    /// Bit pattern accessor for field b_a: UInt8, little endian (bytes 0..1, bits 0..1).
    ///
    /// 
    public var bA: UInt8 {
        BP3DProto.BitCodecLE.readAligned(UInt8.self, self.data[0...1], bitOffset: 0, bitSize: 1)

    }
    /// Bit pattern accessor for field a: UInt32, little endian (bytes 4..8, bits 0..32).
    ///
    /// 
    public var a: UInt32 {
        BP3DProto.ByteCodecLE.readAligned(UInt32.self, self.data[4...8])

    }
    /// Bit pattern accessor for field u_b: UInt16, little endian (bytes 8..10, bits 0..16).
    ///
    /// 
    public var uB: UInt16 {
        BP3DProto.ByteCodecLE.readAligned(UInt16.self, self.data[8...10])

    }
    /// Bit pattern accessor for field b: UInt16, little endian (bytes 10..12, bits 0..16).
    ///
    /// 
    public var b: UInt16 {
        BP3DProto.ByteCodecLE.readAligned(UInt16.self, self.data[10...12])

    }
    /// Bit pattern accessor for field u_c: UInt8, little endian (bytes 12..13, bits 0..8).
    ///
    /// 
    public var uC: UInt8 {
        BP3DProto.ByteCodecLE.readAligned(UInt8.self, self.data[12...13])

    }
    /// Bit pattern accessor for field c: UInt8, little endian (bytes 13..14, bits 0..8).
    ///
    /// 
    public var c: UInt8 {
        BP3DProto.ByteCodecLE.readAligned(UInt8.self, self.data[13...14])

    }
    /// Bit pattern accessor for field b_c: UInt8, little endian (bytes 0..1, bits 0..1).
    ///
    /// 
    public var bC: UInt8 {
        BP3DProto.BitCodecLE.readAligned(UInt8.self, self.data[0...1], bitOffset: 0, bitSize: 1)

    }

}
extension StructsBinNumbers where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field u_a: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public func setUA(_ value: UInt32) {
        var buffer = self.data[0...4];
        BP3DProto.ByteCodecLE.writeAligned(UInt32.self, &buffer, value: value);

    }
    /// Bit pattern setter for field b_a: UInt8, little endian (bytes 0..1, bits 0..1).
    ///
    /// 
    public func setBA(_ value: UInt8) {
        var buffer = self.data[0...1];
        BP3DProto.BitCodecLE.writeAligned(UInt8.self, &buffer, bitOffset: 0, bitSize: 1, value: value);

    }
    /// Bit pattern setter for field a: UInt32, little endian (bytes 4..8, bits 0..32).
    ///
    /// 
    public func setA(_ value: UInt32) {
        var buffer = self.data[4...8];
        BP3DProto.ByteCodecLE.writeAligned(UInt32.self, &buffer, value: value);

    }
    /// Bit pattern setter for field u_b: UInt16, little endian (bytes 8..10, bits 0..16).
    ///
    /// 
    public func setUB(_ value: UInt16) {
        var buffer = self.data[8...10];
        BP3DProto.ByteCodecLE.writeAligned(UInt16.self, &buffer, value: value);

    }
    /// Bit pattern setter for field b: UInt16, little endian (bytes 10..12, bits 0..16).
    ///
    /// 
    public func setB(_ value: UInt16) {
        var buffer = self.data[10...12];
        BP3DProto.ByteCodecLE.writeAligned(UInt16.self, &buffer, value: value);

    }
    /// Bit pattern setter for field u_c: UInt8, little endian (bytes 12..13, bits 0..8).
    ///
    /// 
    public func setUC(_ value: UInt8) {
        var buffer = self.data[12...13];
        BP3DProto.ByteCodecLE.writeAligned(UInt8.self, &buffer, value: value);

    }
    /// Bit pattern setter for field c: UInt8, little endian (bytes 13..14, bits 0..8).
    ///
    /// 
    public func setC(_ value: UInt8) {
        var buffer = self.data[13...14];
        BP3DProto.ByteCodecLE.writeAligned(UInt8.self, &buffer, value: value);

    }
    /// Bit pattern setter for field b_c: UInt8, little endian (bytes 0..1, bits 0..1).
    ///
    /// 
    public func setBC(_ value: UInt8) {
        var buffer = self.data[0...1];
        BP3DProto.BitCodecLE.writeAligned(UInt8.self, &buffer, bitOffset: 0, bitSize: 1, value: value);

    }

}
/// Definition of the raw layout for Numbers structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct StructsRawNumbers<T> {
    var bin: StructsBinNumbers<T>
}
extension StructsRawNumbers where T: BP3DProto.Buffer {
    /// Raw field accessor for u_a: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public var uA: UInt32 {
        self.bin.uA

    }
    /// Raw field accessor for b_a: UInt8, little endian (bytes 0..1, bits 0..1).
    ///
    /// 
    public var bA: Bool {
        if self.bin.bA != 0 {
            return true;
        } else {
            return false;
        }

    }
    /// Raw field accessor for a: UInt32, little endian (bytes 4..8, bits 0..32).
    ///
    /// 
    public var a: Int32 {
        BP3DProto.transmute(self.bin.a)

    }
    /// Raw field accessor for u_b: UInt16, little endian (bytes 8..10, bits 0..16).
    ///
    /// 
    public var uB: UInt16 {
        self.bin.uB

    }
    /// Raw field accessor for b: UInt16, little endian (bytes 10..12, bits 0..16).
    ///
    /// 
    public var b: Int16 {
        BP3DProto.transmute(self.bin.b)

    }
    /// Raw field accessor for u_c: UInt8, little endian (bytes 12..13, bits 0..8).
    ///
    /// 
    public var uC: UInt8 {
        self.bin.uC

    }
    /// Raw field accessor for c: UInt8, little endian (bytes 13..14, bits 0..8).
    ///
    /// 
    public var c: Int8 {
        BP3DProto.transmute(self.bin.c)

    }
    /// Raw field accessor for b_c: UInt8, little endian (bytes 0..1, bits 0..1).
    ///
    /// 
    public var bC: Bool {
        if self.bin.bC != 0 {
            return true;
        } else {
            return false;
        }

    }

}
extension StructsRawNumbers where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for u_a: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public func setUA(_ value: UInt32) {
        self.bin.setUA(value);

    }
    /// Raw field setter for b_a: UInt8, little endian (bytes 0..1, bits 0..1).
    ///
    /// 
    public func setBA(_ value: Bool) {
        if value {
            self.bin.setBA(1);
        } else {
            self.bin.setBA(0);
        }

    }
    /// Raw field setter for a: UInt32, little endian (bytes 4..8, bits 0..32).
    ///
    /// 
    public func setA(_ value: Int32) {
        self.bin.setA(BP3DProto.transmute(value));

    }
    /// Raw field setter for u_b: UInt16, little endian (bytes 8..10, bits 0..16).
    ///
    /// 
    public func setUB(_ value: UInt16) {
        self.bin.setUB(value);

    }
    /// Raw field setter for b: UInt16, little endian (bytes 10..12, bits 0..16).
    ///
    /// 
    public func setB(_ value: Int16) {
        self.bin.setB(BP3DProto.transmute(value));

    }
    /// Raw field setter for u_c: UInt8, little endian (bytes 12..13, bits 0..8).
    ///
    /// 
    public func setUC(_ value: UInt8) {
        self.bin.setUC(value);

    }
    /// Raw field setter for c: UInt8, little endian (bytes 13..14, bits 0..8).
    ///
    /// 
    public func setC(_ value: Int8) {
        self.bin.setC(BP3DProto.transmute(value));

    }
    /// Raw field setter for b_c: UInt8, little endian (bytes 0..1, bits 0..1).
    ///
    /// 
    public func setBC(_ value: Bool) {
        if value {
            self.bin.setBC(1);
        } else {
            self.bin.setBC(0);
        }

    }

}

public struct StructsFlags<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: StructsRawFlags<T>
    public static var size: Int { 15 }
    public init(_ data: T) {
        self._raw = StructsRawFlags(bin: StructsBinFlags(data: data));
    }
    public var raw: StructsRawFlags<T> {
        return _raw;
    }
    public var bin: StructsBinFlags<T> {
        return _raw.bin;
    }
}
extension StructsFlags<BP3DProto.DataBuffer> {
    public init() {
        self._raw = StructsRawFlags(bin: StructsBinFlags(data: BP3DProto.DataBuffer(size: 15)));
    }
}
public let SIZE_STRUCTS_FLAGS: Int = 15;
extension StructsFlags: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = StructsFlags;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...15].toData());
    }
}
extension StructsFlags: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = StructsFlags;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 15 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 15, data: StructsFlags(bytes[...15]));
    }
}
extension StructsFlags where T: BP3DProto.Buffer {
    public var a: Bool {
        self._raw.a
    }
    public var b: Bool {
        self._raw.b
    }
    public var c: Bool {
        self._raw.c
    }
    public var d: Bool {
        self._raw.d
    }

}
extension StructsFlags where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setA(_ value: Bool) -> Self {
        self._raw.setA(value);
        return self;
    }
    @discardableResult
    public func setB(_ value: Bool) -> Self {
        self._raw.setB(value);
        return self;
    }
    @discardableResult
    public func setC(_ value: Bool) -> Self {
        self._raw.setC(value);
        return self;
    }
    @discardableResult
    public func setD(_ value: Bool) -> Self {
        self._raw.setD(value);
        return self;
    }

}
/// Definition of the bits layout for Flags structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct StructsBinFlags<T> {
    var data: T
}
extension StructsBinFlags where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field a: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public var a: UInt8 {
        BP3DProto.ByteCodecLE.readAligned(UInt8.self, self.data[0...1])

    }
    /// Bit pattern accessor for field b: UInt16, little endian (bytes 1..3, bits 0..16).
    ///
    /// 
    public var b: UInt16 {
        BP3DProto.ByteCodecLE.readAligned(UInt16.self, self.data[1...3])

    }
    /// Bit pattern accessor for field c: UInt32, little endian (bytes 3..7, bits 0..32).
    ///
    /// 
    public var c: UInt32 {
        BP3DProto.ByteCodecLE.readAligned(UInt32.self, self.data[3...7])

    }
    /// Bit pattern accessor for field d: UInt64, little endian (bytes 7..15, bits 0..64).
    ///
    /// 
    public var d: UInt64 {
        BP3DProto.ByteCodecLE.readAligned(UInt64.self, self.data[7...15])

    }

}
extension StructsBinFlags where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field a: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public func setA(_ value: UInt8) {
        var buffer = self.data[0...1];
        BP3DProto.ByteCodecLE.writeAligned(UInt8.self, &buffer, value: value);

    }
    /// Bit pattern setter for field b: UInt16, little endian (bytes 1..3, bits 0..16).
    ///
    /// 
    public func setB(_ value: UInt16) {
        var buffer = self.data[1...3];
        BP3DProto.ByteCodecLE.writeAligned(UInt16.self, &buffer, value: value);

    }
    /// Bit pattern setter for field c: UInt32, little endian (bytes 3..7, bits 0..32).
    ///
    /// 
    public func setC(_ value: UInt32) {
        var buffer = self.data[3...7];
        BP3DProto.ByteCodecLE.writeAligned(UInt32.self, &buffer, value: value);

    }
    /// Bit pattern setter for field d: UInt64, little endian (bytes 7..15, bits 0..64).
    ///
    /// 
    public func setD(_ value: UInt64) {
        var buffer = self.data[7...15];
        BP3DProto.ByteCodecLE.writeAligned(UInt64.self, &buffer, value: value);

    }

}
/// Definition of the raw layout for Flags structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct StructsRawFlags<T> {
    var bin: StructsBinFlags<T>
}
extension StructsRawFlags where T: BP3DProto.Buffer {
    /// Raw field accessor for a: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public var a: Bool {
        if self.bin.a != 0 {
            return true;
        } else {
            return false;
        }

    }
    /// Raw field accessor for b: UInt16, little endian (bytes 1..3, bits 0..16).
    ///
    /// 
    public var b: Bool {
        if self.bin.b != 0 {
            return true;
        } else {
            return false;
        }

    }
    /// Raw field accessor for c: UInt32, little endian (bytes 3..7, bits 0..32).
    ///
    /// 
    public var c: Bool {
        if self.bin.c != 0 {
            return true;
        } else {
            return false;
        }

    }
    /// Raw field accessor for d: UInt64, little endian (bytes 7..15, bits 0..64).
    ///
    /// 
    public var d: Bool {
        if self.bin.d != 0 {
            return true;
        } else {
            return false;
        }

    }

}
extension StructsRawFlags where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for a: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public func setA(_ value: Bool) {
        if value {
            self.bin.setA(1);
        } else {
            self.bin.setA(0);
        }

    }
    /// Raw field setter for b: UInt16, little endian (bytes 1..3, bits 0..16).
    ///
    /// 
    public func setB(_ value: Bool) {
        if value {
            self.bin.setB(1);
        } else {
            self.bin.setB(0);
        }

    }
    /// Raw field setter for c: UInt32, little endian (bytes 3..7, bits 0..32).
    ///
    /// 
    public func setC(_ value: Bool) {
        if value {
            self.bin.setC(1);
        } else {
            self.bin.setC(0);
        }

    }
    /// Raw field setter for d: UInt64, little endian (bytes 7..15, bits 0..64).
    ///
    /// 
    public func setD(_ value: Bool) {
        if value {
            self.bin.setD(1);
        } else {
            self.bin.setD(0);
        }

    }

}

public struct StructsFloats<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: StructsRawFloats<T>
    public static var size: Int { 12 }
    public init(_ data: T) {
        self._raw = StructsRawFloats(bin: StructsBinFloats(data: data));
    }
    public var raw: StructsRawFloats<T> {
        return _raw;
    }
    public var bin: StructsBinFloats<T> {
        return _raw.bin;
    }
}
extension StructsFloats<BP3DProto.DataBuffer> {
    public init() {
        self._raw = StructsRawFloats(bin: StructsBinFloats(data: BP3DProto.DataBuffer(size: 12)));
    }
}
public let SIZE_STRUCTS_FLOATS: Int = 12;
extension StructsFloats: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = StructsFloats;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...12].toData());
    }
}
extension StructsFloats: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = StructsFloats;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 12 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 12, data: StructsFloats(bytes[...12]));
    }
}
extension StructsFloats where T: BP3DProto.Buffer {
    public var a: Float32 {
        self._raw.a
    }
    public var b: Float64 {
        self._raw.b
    }

}
extension StructsFloats where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setA(_ value: Float32) -> Self {
        self._raw.setA(value);
        return self;
    }
    @discardableResult
    public func setB(_ value: Float64) -> Self {
        self._raw.setB(value);
        return self;
    }

}
/// Definition of the bits layout for Floats structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct StructsBinFloats<T> {
    var data: T
}
extension StructsBinFloats where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field a: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public var a: UInt32 {
        BP3DProto.ByteCodecLE.readAligned(UInt32.self, self.data[0...4])

    }
    /// Bit pattern accessor for field b: UInt64, little endian (bytes 4..12, bits 0..64).
    ///
    /// 
    public var b: UInt64 {
        BP3DProto.ByteCodecLE.readAligned(UInt64.self, self.data[4...12])

    }

}
extension StructsBinFloats where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field a: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public func setA(_ value: UInt32) {
        var buffer = self.data[0...4];
        BP3DProto.ByteCodecLE.writeAligned(UInt32.self, &buffer, value: value);

    }
    /// Bit pattern setter for field b: UInt64, little endian (bytes 4..12, bits 0..64).
    ///
    /// 
    public func setB(_ value: UInt64) {
        var buffer = self.data[4...12];
        BP3DProto.ByteCodecLE.writeAligned(UInt64.self, &buffer, value: value);

    }

}
/// Definition of the raw layout for Floats structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct StructsRawFloats<T> {
    var bin: StructsBinFloats<T>
}
extension StructsRawFloats where T: BP3DProto.Buffer {
    /// Raw field accessor for a: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public var a: Float32 {
        BP3DProto.transmute(self.bin.a)

    }
    /// Raw field accessor for b: UInt64, little endian (bytes 4..12, bits 0..64).
    ///
    /// 
    public var b: Float64 {
        BP3DProto.transmute(self.bin.b)

    }

}
extension StructsRawFloats where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for a: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public func setA(_ value: Float32) {
        self.bin.setA(BP3DProto.transmute(value));

    }
    /// Raw field setter for b: UInt64, little endian (bytes 4..12, bits 0..64).
    ///
    /// 
    public func setB(_ value: Float64) {
        self.bin.setB(BP3DProto.transmute(value));

    }

}

public struct StructsMaster<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: StructsRawMaster<T>
    public static var size: Int { 41 }
    public init(_ data: T) {
        self._raw = StructsRawMaster(bin: StructsBinMaster(data: data));
    }
    public var raw: StructsRawMaster<T> {
        return _raw;
    }
    public var bin: StructsBinMaster<T> {
        return _raw.bin;
    }
}
extension StructsMaster<BP3DProto.DataBuffer> {
    public init() {
        self._raw = StructsRawMaster(bin: StructsBinMaster(data: BP3DProto.DataBuffer(size: 41)));
    }
}
public let SIZE_STRUCTS_MASTER: Int = 41;
extension StructsMaster: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = StructsMaster;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...41].toData());
    }
}
extension StructsMaster: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = StructsMaster;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 41 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 41, data: StructsMaster(bytes[...41]));
    }
}
extension StructsMaster where T: BP3DProto.Buffer {
    public var floats: StructsFloats<T> {
        StructsFloats(self._raw.bin.data[0...12])
    }
    public var nums: StructsNumbers<T> {
        StructsNumbers(self._raw.bin.data[12...26])
    }
    public var flags: StructsFlags<T> {
        StructsFlags(self._raw.bin.data[26...41])
    }

}
extension StructsMaster where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {

}
/// Definition of the bits layout for Master structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct StructsBinMaster<T> {
    var data: T
}
/// Definition of the raw layout for Master structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct StructsRawMaster<T> {
    var bin: StructsBinMaster<T>
}
