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

public struct StructsNumbers<T>: BP3DProto.FixedSize {
    var data: T
    public static var size: Int { 14 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension StructsNumbers<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 14)
    }
}
public let SIZE_STRUCTS_NUMBERS: Int = 14;
extension StructsNumbers: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = StructsNumbers;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...14].toData());
    }
}
extension StructsNumbers: BP3DProto.FromSlice where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = StructsNumbers;
    public static func from(slice: T) throws -> BP3DProto.Message<Output> {
        if slice.size < 14 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 14, data: StructsNumbers(slice[...14]));
    }
}
extension StructsNumbers where T: BP3DProto.Buffer {
    public var rawUA: UInt32 {
        BP3DProto.ByteCodecLE.readAligned(UInt32.self, self.data[0...4])

    }
    public var uA: UInt32 {
        self.rawUA
    }
    public var rawA: UInt32 {
        BP3DProto.ByteCodecLE.readAligned(UInt32.self, self.data[4...8])

    }
    public var a: Int32 {
        BP3DProto.transmute(self.rawA)

    }
    public var rawUB: UInt16 {
        BP3DProto.ByteCodecLE.readAligned(UInt16.self, self.data[8...10])

    }
    public var uB: UInt16 {
        self.rawUB
    }
    public var rawB: UInt16 {
        BP3DProto.ByteCodecLE.readAligned(UInt16.self, self.data[10...12])

    }
    public var b: Int16 {
        BP3DProto.transmute(self.rawB)

    }
    public var rawUC: UInt8 {
        BP3DProto.ByteCodecLE.readAligned(UInt8.self, self.data[12...13])

    }
    public var uC: UInt8 {
        self.rawUC
    }
    public var rawC: UInt8 {
        BP3DProto.ByteCodecLE.readAligned(UInt8.self, self.data[13...14])

    }
    public var c: Int8 {
        BP3DProto.transmute(self.rawC)

    }

}
extension StructsNumbers where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawUA(_ value: UInt32) {
        var buffer = self.data[0...4];
        BP3DProto.ByteCodecLE.writeAligned(UInt32.self, &buffer, value: value);

    }
    @discardableResult
    public func setUA(_ value: UInt32) -> Self {
        self.setRawUA(value);
        return self;
    }
    public func setRawA(_ value: UInt32) {
        var buffer = self.data[4...8];
        BP3DProto.ByteCodecLE.writeAligned(UInt32.self, &buffer, value: value);

    }
    @discardableResult
    public func setA(_ value: Int32) -> Self {
        self.setRawA(BP3DProto.transmute(value));

        return self;
    }
    public func setRawUB(_ value: UInt16) {
        var buffer = self.data[8...10];
        BP3DProto.ByteCodecLE.writeAligned(UInt16.self, &buffer, value: value);

    }
    @discardableResult
    public func setUB(_ value: UInt16) -> Self {
        self.setRawUB(value);
        return self;
    }
    public func setRawB(_ value: UInt16) {
        var buffer = self.data[10...12];
        BP3DProto.ByteCodecLE.writeAligned(UInt16.self, &buffer, value: value);

    }
    @discardableResult
    public func setB(_ value: Int16) -> Self {
        self.setRawB(BP3DProto.transmute(value));

        return self;
    }
    public func setRawUC(_ value: UInt8) {
        var buffer = self.data[12...13];
        BP3DProto.ByteCodecLE.writeAligned(UInt8.self, &buffer, value: value);

    }
    @discardableResult
    public func setUC(_ value: UInt8) -> Self {
        self.setRawUC(value);
        return self;
    }
    public func setRawC(_ value: UInt8) {
        var buffer = self.data[13...14];
        BP3DProto.ByteCodecLE.writeAligned(UInt8.self, &buffer, value: value);

    }
    @discardableResult
    public func setC(_ value: Int8) -> Self {
        self.setRawC(BP3DProto.transmute(value));

        return self;
    }

}

public struct StructsFlags<T>: BP3DProto.FixedSize {
    var data: T
    public static var size: Int { 15 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension StructsFlags<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 15)
    }
}
public let SIZE_STRUCTS_FLAGS: Int = 15;
extension StructsFlags: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = StructsFlags;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...15].toData());
    }
}
extension StructsFlags: BP3DProto.FromSlice where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = StructsFlags;
    public static func from(slice: T) throws -> BP3DProto.Message<Output> {
        if slice.size < 15 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 15, data: StructsFlags(slice[...15]));
    }
}
extension StructsFlags where T: BP3DProto.Buffer {
    public var rawA: UInt8 {
        BP3DProto.ByteCodecLE.readAligned(UInt8.self, self.data[0...1])

    }
    public var a: Bool {
        if self.rawA != 0 {
            return true;
        } else {
            return false;
        }

    }
    public var rawB: UInt16 {
        BP3DProto.ByteCodecLE.readAligned(UInt16.self, self.data[1...3])

    }
    public var b: Bool {
        if self.rawB != 0 {
            return true;
        } else {
            return false;
        }

    }
    public var rawC: UInt32 {
        BP3DProto.ByteCodecLE.readAligned(UInt32.self, self.data[3...7])

    }
    public var c: Bool {
        if self.rawC != 0 {
            return true;
        } else {
            return false;
        }

    }
    public var rawD: UInt64 {
        BP3DProto.ByteCodecLE.readAligned(UInt64.self, self.data[7...15])

    }
    public var d: Bool {
        if self.rawD != 0 {
            return true;
        } else {
            return false;
        }

    }

}
extension StructsFlags where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawA(_ value: UInt8) {
        var buffer = self.data[0...1];
        BP3DProto.ByteCodecLE.writeAligned(UInt8.self, &buffer, value: value);

    }
    @discardableResult
    public func setA(_ value: Bool) -> Self {
        if value {
            self.setRawA(1);
        } else {
            self.setRawA(0);
        }

        return self;
    }
    public func setRawB(_ value: UInt16) {
        var buffer = self.data[1...3];
        BP3DProto.ByteCodecLE.writeAligned(UInt16.self, &buffer, value: value);

    }
    @discardableResult
    public func setB(_ value: Bool) -> Self {
        if value {
            self.setRawB(1);
        } else {
            self.setRawB(0);
        }

        return self;
    }
    public func setRawC(_ value: UInt32) {
        var buffer = self.data[3...7];
        BP3DProto.ByteCodecLE.writeAligned(UInt32.self, &buffer, value: value);

    }
    @discardableResult
    public func setC(_ value: Bool) -> Self {
        if value {
            self.setRawC(1);
        } else {
            self.setRawC(0);
        }

        return self;
    }
    public func setRawD(_ value: UInt64) {
        var buffer = self.data[7...15];
        BP3DProto.ByteCodecLE.writeAligned(UInt64.self, &buffer, value: value);

    }
    @discardableResult
    public func setD(_ value: Bool) -> Self {
        if value {
            self.setRawD(1);
        } else {
            self.setRawD(0);
        }

        return self;
    }

}

public struct StructsFloats<T>: BP3DProto.FixedSize {
    var data: T
    public static var size: Int { 12 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension StructsFloats<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 12)
    }
}
public let SIZE_STRUCTS_FLOATS: Int = 12;
extension StructsFloats: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = StructsFloats;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...12].toData());
    }
}
extension StructsFloats: BP3DProto.FromSlice where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = StructsFloats;
    public static func from(slice: T) throws -> BP3DProto.Message<Output> {
        if slice.size < 12 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 12, data: StructsFloats(slice[...12]));
    }
}
extension StructsFloats where T: BP3DProto.Buffer {
    public var rawA: UInt32 {
        BP3DProto.ByteCodecLE.readAligned(UInt32.self, self.data[0...4])

    }
    public var a: Float32 {
        BP3DProto.transmute(self.rawA)

    }
    public var rawB: UInt64 {
        BP3DProto.ByteCodecLE.readAligned(UInt64.self, self.data[4...12])

    }
    public var b: Float64 {
        BP3DProto.transmute(self.rawB)

    }

}
extension StructsFloats where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawA(_ value: UInt32) {
        var buffer = self.data[0...4];
        BP3DProto.ByteCodecLE.writeAligned(UInt32.self, &buffer, value: value);

    }
    @discardableResult
    public func setA(_ value: Float32) -> Self {
        self.setRawA(BP3DProto.transmute(value));

        return self;
    }
    public func setRawB(_ value: UInt64) {
        var buffer = self.data[4...12];
        BP3DProto.ByteCodecLE.writeAligned(UInt64.self, &buffer, value: value);

    }
    @discardableResult
    public func setB(_ value: Float64) -> Self {
        self.setRawB(BP3DProto.transmute(value));

        return self;
    }

}

public struct StructsMaster<T>: BP3DProto.FixedSize {
    var data: T
    public static var size: Int { 41 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension StructsMaster<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 41)
    }
}
public let SIZE_STRUCTS_MASTER: Int = 41;
extension StructsMaster: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = StructsMaster;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...41].toData());
    }
}
extension StructsMaster: BP3DProto.FromSlice where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = StructsMaster;
    public static func from(slice: T) throws -> BP3DProto.Message<Output> {
        if slice.size < 41 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 41, data: StructsMaster(slice[...41]));
    }
}
extension StructsMaster where T: BP3DProto.Buffer {
    public var floats: StructsFloats<T> {
        StructsFloats(self.data[0...12])
    }
    public var nums: StructsNumbers<T> {
        StructsNumbers(self.data[12...26])
    }
    public var flags: StructsFlags<T> {
        StructsFlags(self.data[26...41])
    }

}
extension StructsMaster where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {

}
