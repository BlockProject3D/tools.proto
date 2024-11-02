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

public struct ValuesValueInt8<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: ValuesRawValueInt8<T>
    public static var size: Int { 1 }
    public init(_ data: T) {
        self._raw = ValuesRawValueInt8(bin: ValuesBinValueInt8(data: data));
    }
    public var raw: ValuesRawValueInt8<T> {
        return _raw;
    }
    public var bin: ValuesBinValueInt8<T> {
        return _raw.bin;
    }
}
extension ValuesValueInt8<BP3DProto.DataBuffer> {
    public init() {
        self._raw = ValuesRawValueInt8(bin: ValuesBinValueInt8(data: BP3DProto.DataBuffer(size: 1)));
    }
}
public let SIZE_VALUES_VALUE_INT8: Int = 1;
extension ValuesValueInt8: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueInt8;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...1].toData());
    }
}
extension ValuesValueInt8: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = ValuesValueInt8;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 1 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 1, data: ValuesValueInt8(bytes[...1]));
    }
}
extension ValuesValueInt8 where T: BP3DProto.Buffer {
    public var data: Int8 {
        self._raw.data
    }

}
extension ValuesValueInt8 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setData(_ value: Int8) -> Self {
        self._raw.setData(value);
        return self;
    }

}
/// Definition of the bits layout for ValueInt8 structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct ValuesBinValueInt8<T> {
    var data: T
}
extension ValuesBinValueInt8 where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field data: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public var data: UInt8 {
        BP3DProto.ByteCodecLE.readAligned(UInt8.self, self.data[0...1])

    }

}
extension ValuesBinValueInt8 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field data: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public func setData(_ value: UInt8) {
        var buffer = self.data[0...1];
        BP3DProto.ByteCodecLE.writeAligned(UInt8.self, &buffer, value: value);

    }

}
/// Definition of the raw layout for ValueInt8 structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct ValuesRawValueInt8<T> {
    var bin: ValuesBinValueInt8<T>
}
extension ValuesRawValueInt8 where T: BP3DProto.Buffer {
    /// Raw field accessor for data: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public var data: Int8 {
        BP3DProto.transmute(self.bin.data)

    }

}
extension ValuesRawValueInt8 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for data: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public func setData(_ value: Int8) {
        self.bin.setData(BP3DProto.transmute(value));

    }

}

public struct ValuesValueInt16<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: ValuesRawValueInt16<T>
    public static var size: Int { 2 }
    public init(_ data: T) {
        self._raw = ValuesRawValueInt16(bin: ValuesBinValueInt16(data: data));
    }
    public var raw: ValuesRawValueInt16<T> {
        return _raw;
    }
    public var bin: ValuesBinValueInt16<T> {
        return _raw.bin;
    }
}
extension ValuesValueInt16<BP3DProto.DataBuffer> {
    public init() {
        self._raw = ValuesRawValueInt16(bin: ValuesBinValueInt16(data: BP3DProto.DataBuffer(size: 2)));
    }
}
public let SIZE_VALUES_VALUE_INT16: Int = 2;
extension ValuesValueInt16: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueInt16;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...2].toData());
    }
}
extension ValuesValueInt16: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = ValuesValueInt16;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 2 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 2, data: ValuesValueInt16(bytes[...2]));
    }
}
extension ValuesValueInt16 where T: BP3DProto.Buffer {
    public var data: Int16 {
        self._raw.data
    }

}
extension ValuesValueInt16 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setData(_ value: Int16) -> Self {
        self._raw.setData(value);
        return self;
    }

}
/// Definition of the bits layout for ValueInt16 structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct ValuesBinValueInt16<T> {
    var data: T
}
extension ValuesBinValueInt16 where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field data: UInt16, little endian (bytes 0..2, bits 0..16).
    ///
    /// 
    public var data: UInt16 {
        BP3DProto.ByteCodecLE.readAligned(UInt16.self, self.data[0...2])

    }

}
extension ValuesBinValueInt16 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field data: UInt16, little endian (bytes 0..2, bits 0..16).
    ///
    /// 
    public func setData(_ value: UInt16) {
        var buffer = self.data[0...2];
        BP3DProto.ByteCodecLE.writeAligned(UInt16.self, &buffer, value: value);

    }

}
/// Definition of the raw layout for ValueInt16 structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct ValuesRawValueInt16<T> {
    var bin: ValuesBinValueInt16<T>
}
extension ValuesRawValueInt16 where T: BP3DProto.Buffer {
    /// Raw field accessor for data: UInt16, little endian (bytes 0..2, bits 0..16).
    ///
    /// 
    public var data: Int16 {
        BP3DProto.transmute(self.bin.data)

    }

}
extension ValuesRawValueInt16 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for data: UInt16, little endian (bytes 0..2, bits 0..16).
    ///
    /// 
    public func setData(_ value: Int16) {
        self.bin.setData(BP3DProto.transmute(value));

    }

}

public struct ValuesValueInt32<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: ValuesRawValueInt32<T>
    public static var size: Int { 4 }
    public init(_ data: T) {
        self._raw = ValuesRawValueInt32(bin: ValuesBinValueInt32(data: data));
    }
    public var raw: ValuesRawValueInt32<T> {
        return _raw;
    }
    public var bin: ValuesBinValueInt32<T> {
        return _raw.bin;
    }
}
extension ValuesValueInt32<BP3DProto.DataBuffer> {
    public init() {
        self._raw = ValuesRawValueInt32(bin: ValuesBinValueInt32(data: BP3DProto.DataBuffer(size: 4)));
    }
}
public let SIZE_VALUES_VALUE_INT32: Int = 4;
extension ValuesValueInt32: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueInt32;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...4].toData());
    }
}
extension ValuesValueInt32: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = ValuesValueInt32;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 4 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 4, data: ValuesValueInt32(bytes[...4]));
    }
}
extension ValuesValueInt32 where T: BP3DProto.Buffer {
    public var data: Int32 {
        self._raw.data
    }

}
extension ValuesValueInt32 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setData(_ value: Int32) -> Self {
        self._raw.setData(value);
        return self;
    }

}
/// Definition of the bits layout for ValueInt32 structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct ValuesBinValueInt32<T> {
    var data: T
}
extension ValuesBinValueInt32 where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field data: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public var data: UInt32 {
        BP3DProto.ByteCodecLE.readAligned(UInt32.self, self.data[0...4])

    }

}
extension ValuesBinValueInt32 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field data: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public func setData(_ value: UInt32) {
        var buffer = self.data[0...4];
        BP3DProto.ByteCodecLE.writeAligned(UInt32.self, &buffer, value: value);

    }

}
/// Definition of the raw layout for ValueInt32 structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct ValuesRawValueInt32<T> {
    var bin: ValuesBinValueInt32<T>
}
extension ValuesRawValueInt32 where T: BP3DProto.Buffer {
    /// Raw field accessor for data: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public var data: Int32 {
        BP3DProto.transmute(self.bin.data)

    }

}
extension ValuesRawValueInt32 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for data: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public func setData(_ value: Int32) {
        self.bin.setData(BP3DProto.transmute(value));

    }

}

public struct ValuesValueInt64<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: ValuesRawValueInt64<T>
    public static var size: Int { 8 }
    public init(_ data: T) {
        self._raw = ValuesRawValueInt64(bin: ValuesBinValueInt64(data: data));
    }
    public var raw: ValuesRawValueInt64<T> {
        return _raw;
    }
    public var bin: ValuesBinValueInt64<T> {
        return _raw.bin;
    }
}
extension ValuesValueInt64<BP3DProto.DataBuffer> {
    public init() {
        self._raw = ValuesRawValueInt64(bin: ValuesBinValueInt64(data: BP3DProto.DataBuffer(size: 8)));
    }
}
public let SIZE_VALUES_VALUE_INT64: Int = 8;
extension ValuesValueInt64: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueInt64;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...8].toData());
    }
}
extension ValuesValueInt64: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = ValuesValueInt64;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 8 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 8, data: ValuesValueInt64(bytes[...8]));
    }
}
extension ValuesValueInt64 where T: BP3DProto.Buffer {
    public var data: Int64 {
        self._raw.data
    }

}
extension ValuesValueInt64 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setData(_ value: Int64) -> Self {
        self._raw.setData(value);
        return self;
    }

}
/// Definition of the bits layout for ValueInt64 structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct ValuesBinValueInt64<T> {
    var data: T
}
extension ValuesBinValueInt64 where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field data: UInt64, little endian (bytes 0..8, bits 0..64).
    ///
    /// 
    public var data: UInt64 {
        BP3DProto.ByteCodecLE.readAligned(UInt64.self, self.data[0...8])

    }

}
extension ValuesBinValueInt64 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field data: UInt64, little endian (bytes 0..8, bits 0..64).
    ///
    /// 
    public func setData(_ value: UInt64) {
        var buffer = self.data[0...8];
        BP3DProto.ByteCodecLE.writeAligned(UInt64.self, &buffer, value: value);

    }

}
/// Definition of the raw layout for ValueInt64 structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct ValuesRawValueInt64<T> {
    var bin: ValuesBinValueInt64<T>
}
extension ValuesRawValueInt64 where T: BP3DProto.Buffer {
    /// Raw field accessor for data: UInt64, little endian (bytes 0..8, bits 0..64).
    ///
    /// 
    public var data: Int64 {
        BP3DProto.transmute(self.bin.data)

    }

}
extension ValuesRawValueInt64 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for data: UInt64, little endian (bytes 0..8, bits 0..64).
    ///
    /// 
    public func setData(_ value: Int64) {
        self.bin.setData(BP3DProto.transmute(value));

    }

}

public struct ValuesValueUInt8<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: ValuesRawValueUInt8<T>
    public static var size: Int { 1 }
    public init(_ data: T) {
        self._raw = ValuesRawValueUInt8(bin: ValuesBinValueUInt8(data: data));
    }
    public var raw: ValuesRawValueUInt8<T> {
        return _raw;
    }
    public var bin: ValuesBinValueUInt8<T> {
        return _raw.bin;
    }
}
extension ValuesValueUInt8<BP3DProto.DataBuffer> {
    public init() {
        self._raw = ValuesRawValueUInt8(bin: ValuesBinValueUInt8(data: BP3DProto.DataBuffer(size: 1)));
    }
}
public let SIZE_VALUES_VALUE_U_INT8: Int = 1;
extension ValuesValueUInt8: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueUInt8;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...1].toData());
    }
}
extension ValuesValueUInt8: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = ValuesValueUInt8;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 1 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 1, data: ValuesValueUInt8(bytes[...1]));
    }
}
extension ValuesValueUInt8 where T: BP3DProto.Buffer {
    public var data: UInt8 {
        self._raw.data
    }

}
extension ValuesValueUInt8 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setData(_ value: UInt8) -> Self {
        self._raw.setData(value);
        return self;
    }

}
/// Definition of the bits layout for ValueUInt8 structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct ValuesBinValueUInt8<T> {
    var data: T
}
extension ValuesBinValueUInt8 where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field data: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public var data: UInt8 {
        BP3DProto.ByteCodecLE.readAligned(UInt8.self, self.data[0...1])

    }

}
extension ValuesBinValueUInt8 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field data: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public func setData(_ value: UInt8) {
        var buffer = self.data[0...1];
        BP3DProto.ByteCodecLE.writeAligned(UInt8.self, &buffer, value: value);

    }

}
/// Definition of the raw layout for ValueUInt8 structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct ValuesRawValueUInt8<T> {
    var bin: ValuesBinValueUInt8<T>
}
extension ValuesRawValueUInt8 where T: BP3DProto.Buffer {
    /// Raw field accessor for data: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public var data: UInt8 {
        self.bin.data

    }

}
extension ValuesRawValueUInt8 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for data: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public func setData(_ value: UInt8) {
        self.bin.setData(value);

    }

}

public struct ValuesValueUInt16<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: ValuesRawValueUInt16<T>
    public static var size: Int { 2 }
    public init(_ data: T) {
        self._raw = ValuesRawValueUInt16(bin: ValuesBinValueUInt16(data: data));
    }
    public var raw: ValuesRawValueUInt16<T> {
        return _raw;
    }
    public var bin: ValuesBinValueUInt16<T> {
        return _raw.bin;
    }
}
extension ValuesValueUInt16<BP3DProto.DataBuffer> {
    public init() {
        self._raw = ValuesRawValueUInt16(bin: ValuesBinValueUInt16(data: BP3DProto.DataBuffer(size: 2)));
    }
}
public let SIZE_VALUES_VALUE_U_INT16: Int = 2;
extension ValuesValueUInt16: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueUInt16;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...2].toData());
    }
}
extension ValuesValueUInt16: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = ValuesValueUInt16;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 2 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 2, data: ValuesValueUInt16(bytes[...2]));
    }
}
extension ValuesValueUInt16 where T: BP3DProto.Buffer {
    public var data: UInt16 {
        self._raw.data
    }

}
extension ValuesValueUInt16 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setData(_ value: UInt16) -> Self {
        self._raw.setData(value);
        return self;
    }

}
/// Definition of the bits layout for ValueUInt16 structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct ValuesBinValueUInt16<T> {
    var data: T
}
extension ValuesBinValueUInt16 where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field data: UInt16, little endian (bytes 0..2, bits 0..16).
    ///
    /// 
    public var data: UInt16 {
        BP3DProto.ByteCodecLE.readAligned(UInt16.self, self.data[0...2])

    }

}
extension ValuesBinValueUInt16 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field data: UInt16, little endian (bytes 0..2, bits 0..16).
    ///
    /// 
    public func setData(_ value: UInt16) {
        var buffer = self.data[0...2];
        BP3DProto.ByteCodecLE.writeAligned(UInt16.self, &buffer, value: value);

    }

}
/// Definition of the raw layout for ValueUInt16 structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct ValuesRawValueUInt16<T> {
    var bin: ValuesBinValueUInt16<T>
}
extension ValuesRawValueUInt16 where T: BP3DProto.Buffer {
    /// Raw field accessor for data: UInt16, little endian (bytes 0..2, bits 0..16).
    ///
    /// 
    public var data: UInt16 {
        self.bin.data

    }

}
extension ValuesRawValueUInt16 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for data: UInt16, little endian (bytes 0..2, bits 0..16).
    ///
    /// 
    public func setData(_ value: UInt16) {
        self.bin.setData(value);

    }

}

public struct ValuesValueUInt32<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: ValuesRawValueUInt32<T>
    public static var size: Int { 4 }
    public init(_ data: T) {
        self._raw = ValuesRawValueUInt32(bin: ValuesBinValueUInt32(data: data));
    }
    public var raw: ValuesRawValueUInt32<T> {
        return _raw;
    }
    public var bin: ValuesBinValueUInt32<T> {
        return _raw.bin;
    }
}
extension ValuesValueUInt32<BP3DProto.DataBuffer> {
    public init() {
        self._raw = ValuesRawValueUInt32(bin: ValuesBinValueUInt32(data: BP3DProto.DataBuffer(size: 4)));
    }
}
public let SIZE_VALUES_VALUE_U_INT32: Int = 4;
extension ValuesValueUInt32: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueUInt32;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...4].toData());
    }
}
extension ValuesValueUInt32: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = ValuesValueUInt32;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 4 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 4, data: ValuesValueUInt32(bytes[...4]));
    }
}
extension ValuesValueUInt32 where T: BP3DProto.Buffer {
    public var data: UInt32 {
        self._raw.data
    }

}
extension ValuesValueUInt32 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setData(_ value: UInt32) -> Self {
        self._raw.setData(value);
        return self;
    }

}
/// Definition of the bits layout for ValueUInt32 structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct ValuesBinValueUInt32<T> {
    var data: T
}
extension ValuesBinValueUInt32 where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field data: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public var data: UInt32 {
        BP3DProto.ByteCodecLE.readAligned(UInt32.self, self.data[0...4])

    }

}
extension ValuesBinValueUInt32 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field data: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public func setData(_ value: UInt32) {
        var buffer = self.data[0...4];
        BP3DProto.ByteCodecLE.writeAligned(UInt32.self, &buffer, value: value);

    }

}
/// Definition of the raw layout for ValueUInt32 structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct ValuesRawValueUInt32<T> {
    var bin: ValuesBinValueUInt32<T>
}
extension ValuesRawValueUInt32 where T: BP3DProto.Buffer {
    /// Raw field accessor for data: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public var data: UInt32 {
        self.bin.data

    }

}
extension ValuesRawValueUInt32 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for data: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public func setData(_ value: UInt32) {
        self.bin.setData(value);

    }

}

public struct ValuesValueUInt64<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: ValuesRawValueUInt64<T>
    public static var size: Int { 8 }
    public init(_ data: T) {
        self._raw = ValuesRawValueUInt64(bin: ValuesBinValueUInt64(data: data));
    }
    public var raw: ValuesRawValueUInt64<T> {
        return _raw;
    }
    public var bin: ValuesBinValueUInt64<T> {
        return _raw.bin;
    }
}
extension ValuesValueUInt64<BP3DProto.DataBuffer> {
    public init() {
        self._raw = ValuesRawValueUInt64(bin: ValuesBinValueUInt64(data: BP3DProto.DataBuffer(size: 8)));
    }
}
public let SIZE_VALUES_VALUE_U_INT64: Int = 8;
extension ValuesValueUInt64: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueUInt64;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...8].toData());
    }
}
extension ValuesValueUInt64: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = ValuesValueUInt64;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 8 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 8, data: ValuesValueUInt64(bytes[...8]));
    }
}
extension ValuesValueUInt64 where T: BP3DProto.Buffer {
    public var data: UInt64 {
        self._raw.data
    }

}
extension ValuesValueUInt64 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setData(_ value: UInt64) -> Self {
        self._raw.setData(value);
        return self;
    }

}
/// Definition of the bits layout for ValueUInt64 structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct ValuesBinValueUInt64<T> {
    var data: T
}
extension ValuesBinValueUInt64 where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field data: UInt64, little endian (bytes 0..8, bits 0..64).
    ///
    /// 
    public var data: UInt64 {
        BP3DProto.ByteCodecLE.readAligned(UInt64.self, self.data[0...8])

    }

}
extension ValuesBinValueUInt64 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field data: UInt64, little endian (bytes 0..8, bits 0..64).
    ///
    /// 
    public func setData(_ value: UInt64) {
        var buffer = self.data[0...8];
        BP3DProto.ByteCodecLE.writeAligned(UInt64.self, &buffer, value: value);

    }

}
/// Definition of the raw layout for ValueUInt64 structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct ValuesRawValueUInt64<T> {
    var bin: ValuesBinValueUInt64<T>
}
extension ValuesRawValueUInt64 where T: BP3DProto.Buffer {
    /// Raw field accessor for data: UInt64, little endian (bytes 0..8, bits 0..64).
    ///
    /// 
    public var data: UInt64 {
        self.bin.data

    }

}
extension ValuesRawValueUInt64 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for data: UInt64, little endian (bytes 0..8, bits 0..64).
    ///
    /// 
    public func setData(_ value: UInt64) {
        self.bin.setData(value);

    }

}

public struct ValuesValueFloat<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: ValuesRawValueFloat<T>
    public static var size: Int { 4 }
    public init(_ data: T) {
        self._raw = ValuesRawValueFloat(bin: ValuesBinValueFloat(data: data));
    }
    public var raw: ValuesRawValueFloat<T> {
        return _raw;
    }
    public var bin: ValuesBinValueFloat<T> {
        return _raw.bin;
    }
}
extension ValuesValueFloat<BP3DProto.DataBuffer> {
    public init() {
        self._raw = ValuesRawValueFloat(bin: ValuesBinValueFloat(data: BP3DProto.DataBuffer(size: 4)));
    }
}
public let SIZE_VALUES_VALUE_FLOAT: Int = 4;
extension ValuesValueFloat: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueFloat;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...4].toData());
    }
}
extension ValuesValueFloat: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = ValuesValueFloat;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 4 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 4, data: ValuesValueFloat(bytes[...4]));
    }
}
extension ValuesValueFloat where T: BP3DProto.Buffer {
    public var data: Float32 {
        self._raw.data
    }

}
extension ValuesValueFloat where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setData(_ value: Float32) -> Self {
        self._raw.setData(value);
        return self;
    }

}
/// Definition of the bits layout for ValueFloat structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct ValuesBinValueFloat<T> {
    var data: T
}
extension ValuesBinValueFloat where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field data: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public var data: UInt32 {
        BP3DProto.ByteCodecLE.readAligned(UInt32.self, self.data[0...4])

    }

}
extension ValuesBinValueFloat where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field data: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public func setData(_ value: UInt32) {
        var buffer = self.data[0...4];
        BP3DProto.ByteCodecLE.writeAligned(UInt32.self, &buffer, value: value);

    }

}
/// Definition of the raw layout for ValueFloat structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct ValuesRawValueFloat<T> {
    var bin: ValuesBinValueFloat<T>
}
extension ValuesRawValueFloat where T: BP3DProto.Buffer {
    /// Raw field accessor for data: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public var data: Float32 {
        BP3DProto.transmute(self.bin.data)

    }

}
extension ValuesRawValueFloat where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for data: UInt32, little endian (bytes 0..4, bits 0..32).
    ///
    /// 
    public func setData(_ value: Float32) {
        self.bin.setData(BP3DProto.transmute(value));

    }

}

public struct ValuesValueDouble<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: ValuesRawValueDouble<T>
    public static var size: Int { 8 }
    public init(_ data: T) {
        self._raw = ValuesRawValueDouble(bin: ValuesBinValueDouble(data: data));
    }
    public var raw: ValuesRawValueDouble<T> {
        return _raw;
    }
    public var bin: ValuesBinValueDouble<T> {
        return _raw.bin;
    }
}
extension ValuesValueDouble<BP3DProto.DataBuffer> {
    public init() {
        self._raw = ValuesRawValueDouble(bin: ValuesBinValueDouble(data: BP3DProto.DataBuffer(size: 8)));
    }
}
public let SIZE_VALUES_VALUE_DOUBLE: Int = 8;
extension ValuesValueDouble: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueDouble;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...8].toData());
    }
}
extension ValuesValueDouble: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = ValuesValueDouble;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 8 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 8, data: ValuesValueDouble(bytes[...8]));
    }
}
extension ValuesValueDouble where T: BP3DProto.Buffer {
    public var data: Float64 {
        self._raw.data
    }

}
extension ValuesValueDouble where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setData(_ value: Float64) -> Self {
        self._raw.setData(value);
        return self;
    }

}
/// Definition of the bits layout for ValueDouble structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct ValuesBinValueDouble<T> {
    var data: T
}
extension ValuesBinValueDouble where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field data: UInt64, little endian (bytes 0..8, bits 0..64).
    ///
    /// 
    public var data: UInt64 {
        BP3DProto.ByteCodecLE.readAligned(UInt64.self, self.data[0...8])

    }

}
extension ValuesBinValueDouble where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field data: UInt64, little endian (bytes 0..8, bits 0..64).
    ///
    /// 
    public func setData(_ value: UInt64) {
        var buffer = self.data[0...8];
        BP3DProto.ByteCodecLE.writeAligned(UInt64.self, &buffer, value: value);

    }

}
/// Definition of the raw layout for ValueDouble structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct ValuesRawValueDouble<T> {
    var bin: ValuesBinValueDouble<T>
}
extension ValuesRawValueDouble where T: BP3DProto.Buffer {
    /// Raw field accessor for data: UInt64, little endian (bytes 0..8, bits 0..64).
    ///
    /// 
    public var data: Float64 {
        BP3DProto.transmute(self.bin.data)

    }

}
extension ValuesRawValueDouble where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for data: UInt64, little endian (bytes 0..8, bits 0..64).
    ///
    /// 
    public func setData(_ value: Float64) {
        self.bin.setData(BP3DProto.transmute(value));

    }

}
