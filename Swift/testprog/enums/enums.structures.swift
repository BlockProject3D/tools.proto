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

public struct EnumsHeader<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: EnumsRawHeader<T>
    public static var size: Int { 1 }
    public init(_ data: T) {
        self._raw = EnumsRawHeader(bin: EnumsBinHeader(data: data));
    }
    public var raw: EnumsRawHeader<T> {
        return _raw;
    }
    public var bin: EnumsBinHeader<T> {
        return _raw.bin;
    }
}
extension EnumsHeader<BP3DProto.DataBuffer> {
    public init() {
        self._raw = EnumsRawHeader(bin: EnumsBinHeader(data: BP3DProto.DataBuffer(size: 1)));
    }
}
public let SIZE_ENUMS_HEADER: Int = 1;
extension EnumsHeader: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = EnumsHeader;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...1].toData());
    }
}
extension EnumsHeader: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = EnumsHeader;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 1 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 1, data: EnumsHeader(bytes[...1]));
    }
}
extension EnumsHeader where T: BP3DProto.Buffer {
    public var type: EnumsType? {
        let rawValue = self._raw.type;
        return EnumsType(rawValue: rawValue);
    }

}
extension EnumsHeader where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setType(_ value: EnumsType) -> Self {
        self._raw.setType(value.rawValue);
        return self;
    }

}
/// Definition of the bits layout for Header structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct EnumsBinHeader<T> {
    var data: T
}
extension EnumsBinHeader where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field type: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public var type: UInt8 {
        BP3DProto.ByteCodecLE.readAligned(UInt8.self, self.data[0...1])

    }

}
extension EnumsBinHeader where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field type: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public func setType(_ value: UInt8) {
        var buffer = self.data[0...1];
        BP3DProto.ByteCodecLE.writeAligned(UInt8.self, &buffer, value: value);

    }

}
/// Definition of the raw layout for Header structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct EnumsRawHeader<T> {
    var bin: EnumsBinHeader<T>
}
extension EnumsRawHeader where T: BP3DProto.Buffer {
    /// Raw field accessor for type: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public var type: UInt8 {
        self.bin.type

    }

}
extension EnumsRawHeader where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for type: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public func setType(_ value: UInt8) {
        self.bin.setType(value);

    }

}
