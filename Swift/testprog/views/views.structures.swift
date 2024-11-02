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

public struct ViewsFloats<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: ViewsRawFloats<T>
    public static var size: Int { 3 }
    public init(_ data: T) {
        self._raw = ViewsRawFloats(bin: ViewsBinFloats(data: data));
    }
    public var raw: ViewsRawFloats<T> {
        return _raw;
    }
    public var bin: ViewsBinFloats<T> {
        return _raw.bin;
    }
}
extension ViewsFloats<BP3DProto.DataBuffer> {
    public init() {
        self._raw = ViewsRawFloats(bin: ViewsBinFloats(data: BP3DProto.DataBuffer(size: 3)));
    }
}
public let SIZE_VIEWS_FLOATS: Int = 3;
extension ViewsFloats: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ViewsFloats;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...3].toData());
    }
}
extension ViewsFloats: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = ViewsFloats;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 3 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 3, data: ViewsFloats(bytes[...3]));
    }
}
extension ViewsFloats where T: BP3DProto.Buffer {
    public var a: Float64 {
        let rawValue = Float64(self._raw.a);
        return rawValue * 7.629452739355007e-5 + 0.0;
    }
    public var b: Float32 {
        let rawValue = Float32(self._raw.b);
        return rawValue * 0.1 + 0.0;
    }

}
extension ViewsFloats where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setA(_ value: Float64) -> Self {
        let rawValue = value * 13107.099999999999 + -0.0;
        self._raw.setA(UInt32(rawValue));
        return self;
    }
    @discardableResult
    public func setB(_ value: Float32) -> Self {
        let rawValue = value * 10.0 + 0.0;
        self._raw.setB(UInt8(rawValue));
        return self;
    }

}
/// Definition of the bits layout for Floats structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct ViewsBinFloats<T> {
    var data: T
}
extension ViewsBinFloats where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field a: UInt32, little endian (bytes 0..3, bits 0..17).
    ///
    /// 
    public var a: UInt32 {
        BP3DProto.BitCodecLE.readUnaligned(UInt32.self, self.data[0...3], bitOffset: 0, bitSize: 17)

    }
    /// Bit pattern accessor for field b: UInt8, little endian (bytes 2..3, bits 1..8).
    ///
    /// 
    public var b: UInt8 {
        BP3DProto.BitCodecLE.readAligned(UInt8.self, self.data[2...3], bitOffset: 1, bitSize: 7)

    }

}
extension ViewsBinFloats where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field a: UInt32, little endian (bytes 0..3, bits 0..17).
    ///
    /// 
    public func setA(_ value: UInt32) {
        var buffer = self.data[0...3];
        BP3DProto.BitCodecLE.writeUnaligned(UInt32.self, &buffer, bitOffset: 0, bitSize: 17, value: value);

    }
    /// Bit pattern setter for field b: UInt8, little endian (bytes 2..3, bits 1..8).
    ///
    /// 
    public func setB(_ value: UInt8) {
        var buffer = self.data[2...3];
        BP3DProto.BitCodecLE.writeAligned(UInt8.self, &buffer, bitOffset: 1, bitSize: 7, value: value);

    }

}
/// Definition of the raw layout for Floats structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct ViewsRawFloats<T> {
    var bin: ViewsBinFloats<T>
}
extension ViewsRawFloats where T: BP3DProto.Buffer {
    /// Raw field accessor for a: UInt32, little endian (bytes 0..3, bits 0..17).
    ///
    /// 
    public var a: UInt32 {
        self.bin.a

    }
    /// Raw field accessor for b: UInt8, little endian (bytes 2..3, bits 1..8).
    ///
    /// 
    public var b: UInt8 {
        self.bin.b

    }

}
extension ViewsRawFloats where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for a: UInt32, little endian (bytes 0..3, bits 0..17).
    ///
    /// 
    public func setA(_ value: UInt32) {
        self.bin.setA(value);

    }
    /// Raw field setter for b: UInt8, little endian (bytes 2..3, bits 1..8).
    ///
    /// 
    public func setB(_ value: UInt8) {
        self.bin.setB(value);

    }

}
