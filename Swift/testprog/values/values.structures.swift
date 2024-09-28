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
    var data: T
    public static var size: Int { 1 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension ValuesValueInt8<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 1)
    }
}
public let SIZE_VALUES_VALUE_INT8: Int = 1;
extension ValuesValueInt8: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueInt8;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...1].toData());
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
    public var rawData: UInt8 {
        BP3DProto.ByteCodecLE.readAligned(UInt8.self, self.data[0...1])

    }
    public var data: Int8 {
        BP3DProto.transmute(self.rawData)

    }

}
extension ValuesValueInt8 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawData(_ value: UInt8) {
        var buffer = self.data[0...1];
        BP3DProto.ByteCodecLE.writeAligned(UInt8.self, &buffer, value: value);

    }
    @discardableResult
    public func setData(_ value: Int8) -> Self {
        self.setRawData(BP3DProto.transmute(value));

        return self;
    }

}

public struct ValuesValueInt16<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    var data: T
    public static var size: Int { 2 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension ValuesValueInt16<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 2)
    }
}
public let SIZE_VALUES_VALUE_INT16: Int = 2;
extension ValuesValueInt16: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueInt16;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...2].toData());
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
    public var rawData: UInt16 {
        BP3DProto.ByteCodecLE.readAligned(UInt16.self, self.data[0...2])

    }
    public var data: Int16 {
        BP3DProto.transmute(self.rawData)

    }

}
extension ValuesValueInt16 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawData(_ value: UInt16) {
        var buffer = self.data[0...2];
        BP3DProto.ByteCodecLE.writeAligned(UInt16.self, &buffer, value: value);

    }
    @discardableResult
    public func setData(_ value: Int16) -> Self {
        self.setRawData(BP3DProto.transmute(value));

        return self;
    }

}

public struct ValuesValueInt32<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    var data: T
    public static var size: Int { 4 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension ValuesValueInt32<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 4)
    }
}
public let SIZE_VALUES_VALUE_INT32: Int = 4;
extension ValuesValueInt32: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueInt32;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...4].toData());
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
    public var rawData: UInt32 {
        BP3DProto.ByteCodecLE.readAligned(UInt32.self, self.data[0...4])

    }
    public var data: Int32 {
        BP3DProto.transmute(self.rawData)

    }

}
extension ValuesValueInt32 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawData(_ value: UInt32) {
        var buffer = self.data[0...4];
        BP3DProto.ByteCodecLE.writeAligned(UInt32.self, &buffer, value: value);

    }
    @discardableResult
    public func setData(_ value: Int32) -> Self {
        self.setRawData(BP3DProto.transmute(value));

        return self;
    }

}

public struct ValuesValueInt64<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    var data: T
    public static var size: Int { 8 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension ValuesValueInt64<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 8)
    }
}
public let SIZE_VALUES_VALUE_INT64: Int = 8;
extension ValuesValueInt64: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueInt64;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...8].toData());
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
    public var rawData: UInt64 {
        BP3DProto.ByteCodecLE.readAligned(UInt64.self, self.data[0...8])

    }
    public var data: Int64 {
        BP3DProto.transmute(self.rawData)

    }

}
extension ValuesValueInt64 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawData(_ value: UInt64) {
        var buffer = self.data[0...8];
        BP3DProto.ByteCodecLE.writeAligned(UInt64.self, &buffer, value: value);

    }
    @discardableResult
    public func setData(_ value: Int64) -> Self {
        self.setRawData(BP3DProto.transmute(value));

        return self;
    }

}

public struct ValuesValueUInt8<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    var data: T
    public static var size: Int { 1 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension ValuesValueUInt8<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 1)
    }
}
public let SIZE_VALUES_VALUE_U_INT8: Int = 1;
extension ValuesValueUInt8: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueUInt8;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...1].toData());
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
    public var rawData: UInt8 {
        BP3DProto.ByteCodecLE.readAligned(UInt8.self, self.data[0...1])

    }
    public var data: UInt8 {
        self.rawData
    }

}
extension ValuesValueUInt8 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawData(_ value: UInt8) {
        var buffer = self.data[0...1];
        BP3DProto.ByteCodecLE.writeAligned(UInt8.self, &buffer, value: value);

    }
    @discardableResult
    public func setData(_ value: UInt8) -> Self {
        self.setRawData(value);
        return self;
    }

}

public struct ValuesValueUInt16<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    var data: T
    public static var size: Int { 2 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension ValuesValueUInt16<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 2)
    }
}
public let SIZE_VALUES_VALUE_U_INT16: Int = 2;
extension ValuesValueUInt16: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueUInt16;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...2].toData());
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
    public var rawData: UInt16 {
        BP3DProto.ByteCodecLE.readAligned(UInt16.self, self.data[0...2])

    }
    public var data: UInt16 {
        self.rawData
    }

}
extension ValuesValueUInt16 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawData(_ value: UInt16) {
        var buffer = self.data[0...2];
        BP3DProto.ByteCodecLE.writeAligned(UInt16.self, &buffer, value: value);

    }
    @discardableResult
    public func setData(_ value: UInt16) -> Self {
        self.setRawData(value);
        return self;
    }

}

public struct ValuesValueUInt32<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    var data: T
    public static var size: Int { 4 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension ValuesValueUInt32<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 4)
    }
}
public let SIZE_VALUES_VALUE_U_INT32: Int = 4;
extension ValuesValueUInt32: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueUInt32;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...4].toData());
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
    public var rawData: UInt32 {
        BP3DProto.ByteCodecLE.readAligned(UInt32.self, self.data[0...4])

    }
    public var data: UInt32 {
        self.rawData
    }

}
extension ValuesValueUInt32 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawData(_ value: UInt32) {
        var buffer = self.data[0...4];
        BP3DProto.ByteCodecLE.writeAligned(UInt32.self, &buffer, value: value);

    }
    @discardableResult
    public func setData(_ value: UInt32) -> Self {
        self.setRawData(value);
        return self;
    }

}

public struct ValuesValueUInt64<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    var data: T
    public static var size: Int { 8 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension ValuesValueUInt64<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 8)
    }
}
public let SIZE_VALUES_VALUE_U_INT64: Int = 8;
extension ValuesValueUInt64: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueUInt64;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...8].toData());
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
    public var rawData: UInt64 {
        BP3DProto.ByteCodecLE.readAligned(UInt64.self, self.data[0...8])

    }
    public var data: UInt64 {
        self.rawData
    }

}
extension ValuesValueUInt64 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawData(_ value: UInt64) {
        var buffer = self.data[0...8];
        BP3DProto.ByteCodecLE.writeAligned(UInt64.self, &buffer, value: value);

    }
    @discardableResult
    public func setData(_ value: UInt64) -> Self {
        self.setRawData(value);
        return self;
    }

}

public struct ValuesValueFloat<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    var data: T
    public static var size: Int { 4 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension ValuesValueFloat<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 4)
    }
}
public let SIZE_VALUES_VALUE_FLOAT: Int = 4;
extension ValuesValueFloat: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueFloat;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...4].toData());
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
    public var rawData: UInt32 {
        BP3DProto.ByteCodecLE.readAligned(UInt32.self, self.data[0...4])

    }
    public var data: Float32 {
        BP3DProto.transmute(self.rawData)

    }

}
extension ValuesValueFloat where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawData(_ value: UInt32) {
        var buffer = self.data[0...4];
        BP3DProto.ByteCodecLE.writeAligned(UInt32.self, &buffer, value: value);

    }
    @discardableResult
    public func setData(_ value: Float32) -> Self {
        self.setRawData(BP3DProto.transmute(value));

        return self;
    }

}

public struct ValuesValueDouble<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    var data: T
    public static var size: Int { 8 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension ValuesValueDouble<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 8)
    }
}
public let SIZE_VALUES_VALUE_DOUBLE: Int = 8;
extension ValuesValueDouble: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ValuesValueDouble;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...8].toData());
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
    public var rawData: UInt64 {
        BP3DProto.ByteCodecLE.readAligned(UInt64.self, self.data[0...8])

    }
    public var data: Float64 {
        BP3DProto.transmute(self.rawData)

    }

}
extension ValuesValueDouble where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawData(_ value: UInt64) {
        var buffer = self.data[0...8];
        BP3DProto.ByteCodecLE.writeAligned(UInt64.self, &buffer, value: value);

    }
    @discardableResult
    public func setData(_ value: Float64) -> Self {
        self.setRawData(BP3DProto.transmute(value));

        return self;
    }

}
