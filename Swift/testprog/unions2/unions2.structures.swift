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

public struct Unions2Header<T>: BP3DProto.FixedSize {
    var data: T
    public static var size: Int { 1 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension Unions2Header<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 1)
    }
}
public let SIZE_UNIONS2_HEADER: Int = 1;
extension Unions2Header: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = Unions2Header;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...1].toData());
    }
}
extension Unions2Header: BP3DProto.FromSlice where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = Unions2Header;
    public static func from(slice: T) throws -> BP3DProto.Message<Output> {
        if slice.size < 1 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 1, data: Unions2Header(slice[...1]));
    }
}
extension Unions2Header where T: BP3DProto.Buffer {
    public var rawTest: UInt8 {
        BP3DProto.BitCodecLE.readAligned(UInt8.self, self.data[0...1], bitOffset: 0, bitSize: 1)

    }
    public var test: Unions2Test? {
        let rawValue = self.rawTest;
        return Unions2Test(rawValue: rawValue);
    }
    public var rawTest2: UInt8 {
        BP3DProto.BitCodecLE.readAligned(UInt8.self, self.data[0...1], bitOffset: 1, bitSize: 7)

    }
    public var test2: Int8 {
        let rawValue = self.rawTest2;
        if rawValue > 63 {
            return -(Int8(((~rawValue) & 63) + 1));
        } else {
            return Int8(rawValue & 63);
        }
    }

}
extension Unions2Header where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawTest(_ value: UInt8) {
        var buffer = self.data[0...1];
        BP3DProto.BitCodecLE.writeAligned(UInt8.self, &buffer, bitOffset: 0, bitSize: 1, value: value);

    }
    @discardableResult
    public func setTest(_ value: Unions2Test) -> Self {
        self.setRawTest(UInt8(value.rawValue));
        return self;
    }
    public func setRawTest2(_ value: UInt8) {
        var buffer = self.data[0...1];
        BP3DProto.BitCodecLE.writeAligned(UInt8.self, &buffer, bitOffset: 1, bitSize: 7, value: value);

    }
    @discardableResult
    public func setTest2(_ value: Int8) -> Self {
        self.setRawTest2(BP3DProto.transmute(value));

        return self;
    }

}

import Foundation;
import BP3DProto;

public struct Unions2Header2<T>: BP3DProto.FixedSize {
    var data: T
    public static var size: Int { 1 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension Unions2Header2<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 1)
    }
}
public let SIZE_UNIONS2_HEADER2: Int = 1;
extension Unions2Header2: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = Unions2Header2;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...1].toData());
    }
}
extension Unions2Header2: BP3DProto.FromSlice where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = Unions2Header2;
    public static func from(slice: T) throws -> BP3DProto.Message<Output> {
        if slice.size < 1 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 1, data: Unions2Header2(slice[...1]));
    }
}
extension Unions2Header2 where T: BP3DProto.Buffer {
    public var inner: Unions2Header<T> {
        Unions2Header(self.data[0...1])
    }

}
extension Unions2Header2 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {

}
