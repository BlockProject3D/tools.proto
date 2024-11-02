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

public struct Lists2Times<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: Lists2RawTimes<T>
    public static var size: Int { 16 }
    public init(_ data: T) {
        self._raw = Lists2RawTimes(bin: Lists2BinTimes(data: data));
    }
    public var raw: Lists2RawTimes<T> {
        return _raw;
    }
    public var bin: Lists2BinTimes<T> {
        return _raw.bin;
    }
}
extension Lists2Times<BP3DProto.DataBuffer> {
    public init() {
        self._raw = Lists2RawTimes(bin: Lists2BinTimes(data: BP3DProto.DataBuffer(size: 16)));
    }
}
public let SIZE_LISTS2_TIMES: Int = 16;
extension Lists2Times: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = Lists2Times;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...16].toData());
    }
}
extension Lists2Times: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = Lists2Times;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 16 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 16, data: Lists2Times(bytes[...16]));
    }
}
extension Lists2Times where T: BP3DProto.Buffer {
    public var start: UInt64 {
        self._raw.start
    }
    public var end: UInt64 {
        self._raw.end
    }

}
extension Lists2Times where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setStart(_ value: UInt64) -> Self {
        self._raw.setStart(value);
        return self;
    }
    @discardableResult
    public func setEnd(_ value: UInt64) -> Self {
        self._raw.setEnd(value);
        return self;
    }

}
/// Definition of the bits layout for Times structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct Lists2BinTimes<T> {
    var data: T
}
extension Lists2BinTimes where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field start: UInt64, little endian (bytes 0..8, bits 0..64).
    ///
    /// 
    public var start: UInt64 {
        BP3DProto.ByteCodecLE.readAligned(UInt64.self, self.data[0...8])

    }
    /// Bit pattern accessor for field end: UInt64, little endian (bytes 8..16, bits 0..64).
    ///
    /// 
    public var end: UInt64 {
        BP3DProto.ByteCodecLE.readAligned(UInt64.self, self.data[8...16])

    }

}
extension Lists2BinTimes where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field start: UInt64, little endian (bytes 0..8, bits 0..64).
    ///
    /// 
    public func setStart(_ value: UInt64) {
        var buffer = self.data[0...8];
        BP3DProto.ByteCodecLE.writeAligned(UInt64.self, &buffer, value: value);

    }
    /// Bit pattern setter for field end: UInt64, little endian (bytes 8..16, bits 0..64).
    ///
    /// 
    public func setEnd(_ value: UInt64) {
        var buffer = self.data[8...16];
        BP3DProto.ByteCodecLE.writeAligned(UInt64.self, &buffer, value: value);

    }

}
/// Definition of the raw layout for Times structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct Lists2RawTimes<T> {
    var bin: Lists2BinTimes<T>
}
extension Lists2RawTimes where T: BP3DProto.Buffer {
    /// Raw field accessor for start: UInt64, little endian (bytes 0..8, bits 0..64).
    ///
    /// 
    public var start: UInt64 {
        self.bin.start

    }
    /// Raw field accessor for end: UInt64, little endian (bytes 8..16, bits 0..64).
    ///
    /// 
    public var end: UInt64 {
        self.bin.end

    }

}
extension Lists2RawTimes where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for start: UInt64, little endian (bytes 0..8, bits 0..64).
    ///
    /// 
    public func setStart(_ value: UInt64) {
        self.bin.setStart(value);

    }
    /// Raw field setter for end: UInt64, little endian (bytes 8..16, bits 0..64).
    ///
    /// 
    public func setEnd(_ value: UInt64) {
        self.bin.setEnd(value);

    }

}
