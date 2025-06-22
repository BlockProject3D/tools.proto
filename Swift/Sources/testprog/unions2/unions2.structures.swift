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

public struct Unions2Header<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: Unions2RawHeader<T>
    public static var size: Int { 1 }
    public init(_ data: T) {
        self._raw = Unions2RawHeader(bin: Unions2BinHeader(data: data));
    }
    public var raw: Unions2RawHeader<T> {
        return _raw;
    }
    public var bin: Unions2BinHeader<T> {
        return _raw.bin;
    }
}
extension Unions2Header<BP3DProto.DataBuffer> {
    public init() {
        self._raw = Unions2RawHeader(bin: Unions2BinHeader(data: BP3DProto.DataBuffer(size: 1)));
    }
}
public let SIZE_UNIONS2_HEADER: Int = 1;
extension Unions2Header: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = Unions2Header;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...1].toData());
    }
}
extension Unions2Header: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = Unions2Header;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 1 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 1, data: Unions2Header(bytes[...1]));
    }
}
extension Unions2Header where T: BP3DProto.Buffer {
    public var test: Unions2Test? {
        let rawValue = self._raw.test;
        return Unions2Test(rawValue: UInt8(rawValue));
    }
    public var test2: Int8 {
        self._raw.test2
    }

}
extension Unions2Header where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setTest(_ value: Unions2Test) -> Self {
        self._raw.setTest(UInt8(value.rawValue));
        return self;
    }
    @discardableResult
    public func setTest2(_ value: Int8) -> Self {
        self._raw.setTest2(value);
        return self;
    }

}
/// Definition of the bits layout for Header structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct Unions2BinHeader<T> {
    var data: T
}
extension Unions2BinHeader where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field test: UInt8, little endian (bytes 0..1, bits 0..1).
    ///
    /// 
    public var test: UInt8 {
        BP3DProto.BitCodecLE.readAligned(UInt8.self, self.data[0...1], bitOffset: 0, bitSize: 1)

    }
    /// Bit pattern accessor for field test2: UInt8, little endian (bytes 0..1, bits 1..8).
    ///
    /// 
    public var test2: UInt8 {
        BP3DProto.BitCodecLE.readAligned(UInt8.self, self.data[0...1], bitOffset: 1, bitSize: 7)

    }

}
extension Unions2BinHeader where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field test: UInt8, little endian (bytes 0..1, bits 0..1).
    ///
    /// 
    public func setTest(_ value: UInt8) {
        var buffer = self.data[0...1];
        BP3DProto.BitCodecLE.writeAligned(UInt8.self, &buffer, bitOffset: 0, bitSize: 1, value: value);

    }
    /// Bit pattern setter for field test2: UInt8, little endian (bytes 0..1, bits 1..8).
    ///
    /// 
    public func setTest2(_ value: UInt8) {
        var buffer = self.data[0...1];
        BP3DProto.BitCodecLE.writeAligned(UInt8.self, &buffer, bitOffset: 1, bitSize: 7, value: value);

    }

}
/// Definition of the raw layout for Header structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct Unions2RawHeader<T> {
    var bin: Unions2BinHeader<T>
}
extension Unions2RawHeader where T: BP3DProto.Buffer {
    /// Raw field accessor for test: UInt8, little endian (bytes 0..1, bits 0..1).
    ///
    /// 
    public var test: UInt8 {
        self.bin.test

    }
    /// Raw field accessor for test2: UInt8, little endian (bytes 0..1, bits 1..8).
    ///
    /// 
    public var test2: Int8 {
        let rawValue = self.bin.test2;
        if rawValue > 63 {
            return -(Int8(((~rawValue) & 63) + 1));
        } else {
            return Int8(rawValue & 63);
        }

    }

}
extension Unions2RawHeader where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for test: UInt8, little endian (bytes 0..1, bits 0..1).
    ///
    /// 
    public func setTest(_ value: UInt8) {
        self.bin.setTest(value);

    }
    /// Raw field setter for test2: UInt8, little endian (bytes 0..1, bits 1..8).
    ///
    /// 
    public func setTest2(_ value: Int8) {
        self.bin.setTest2(BP3DProto.transmute(value));

    }

}

public struct Unions2Header2<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: Unions2RawHeader2<T>
    public static var size: Int { 1 }
    public init(_ data: T) {
        self._raw = Unions2RawHeader2(bin: Unions2BinHeader2(data: data));
    }
    public var raw: Unions2RawHeader2<T> {
        return _raw;
    }
    public var bin: Unions2BinHeader2<T> {
        return _raw.bin;
    }
}
extension Unions2Header2<BP3DProto.DataBuffer> {
    public init() {
        self._raw = Unions2RawHeader2(bin: Unions2BinHeader2(data: BP3DProto.DataBuffer(size: 1)));
    }
}
public let SIZE_UNIONS2_HEADER2: Int = 1;
extension Unions2Header2: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = Unions2Header2;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...1].toData());
    }
}
extension Unions2Header2: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = Unions2Header2;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 1 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 1, data: Unions2Header2(bytes[...1]));
    }
}
extension Unions2Header2 where T: BP3DProto.Buffer {
    public var inner: Unions2Header<T> {
        Unions2Header(self._raw.bin.data[0...1])
    }

}
extension Unions2Header2 where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {

}
/// Definition of the bits layout for Header2 structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct Unions2BinHeader2<T> {
    var data: T
}
/// Definition of the raw layout for Header2 structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct Unions2RawHeader2<T> {
    var bin: Unions2BinHeader2<T>
}
