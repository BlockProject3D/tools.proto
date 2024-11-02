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

public struct ArraysItem<T>: BP3DProto.FixedSize, FromBuffer {
    public typealias Buffer = T
    private var _raw: ArraysRawItem<T>
    public static var size: Int { 3 }
    public init(_ data: T) {
        self._raw = ArraysRawItem(bin: ArraysBinItem(data: data));
    }
    public var raw: ArraysRawItem<T> {
        return _raw;
    }
    public var bin: ArraysBinItem<T> {
        return _raw.bin;
    }
}
extension ArraysItem<BP3DProto.DataBuffer> {
    public init() {
        self._raw = ArraysRawItem(bin: ArraysBinItem(data: BP3DProto.DataBuffer(size: 3)));
    }
}
public let SIZE_ARRAYS_ITEM: Int = 3;
extension ArraysItem: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ArraysItem;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input._raw.bin.data[...3].toData());
    }
}
extension ArraysItem: BP3DProto.FromBytes where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = ArraysItem;
    public static func from(bytes: T) throws -> BP3DProto.Message<Output> {
        if bytes.size < 3 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 3, data: ArraysItem(bytes[...3]));
    }
}
extension ArraysItem where T: BP3DProto.Buffer {
    public var id: UInt8 {
        self._raw.id
    }
    public var count: UInt16 {
        self._raw.count
    }
    public var slot: UInt8 {
        self._raw.slot
    }

}
extension ArraysItem where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    @discardableResult
    public func setId(_ value: UInt8) -> Self {
        self._raw.setId(value);
        return self;
    }
    @discardableResult
    public func setCount(_ value: UInt16) -> Self {
        self._raw.setCount(value);
        return self;
    }
    @discardableResult
    public func setSlot(_ value: UInt8) -> Self {
        self._raw.setSlot(value);
        return self;
    }

}
/// Definition of the bits layout for Item structure.
///
/// The bits layout wraps a byte buffer and offers access
/// to the raw bit patterns of each field in a structure.
public struct ArraysBinItem<T> {
    var data: T
}
extension ArraysBinItem where T: BP3DProto.Buffer {
    /// Bit pattern accessor for field id: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public var id: UInt8 {
        BP3DProto.ByteCodecLE.readAligned(UInt8.self, self.data[0...1])

    }
    /// Bit pattern accessor for field count: UInt16, little endian (bytes 1..3, bits 0..11).
    ///
    /// 
    public var count: UInt16 {
        BP3DProto.BitCodecLE.readAligned(UInt16.self, self.data[1...3], bitOffset: 0, bitSize: 11)

    }
    /// Bit pattern accessor for field slot: UInt8, little endian (bytes 2..3, bits 3..8).
    ///
    /// 
    public var slot: UInt8 {
        BP3DProto.BitCodecLE.readAligned(UInt8.self, self.data[2...3], bitOffset: 3, bitSize: 5)

    }

}
extension ArraysBinItem where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Bit pattern setter for field id: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public func setId(_ value: UInt8) {
        var buffer = self.data[0...1];
        BP3DProto.ByteCodecLE.writeAligned(UInt8.self, &buffer, value: value);

    }
    /// Bit pattern setter for field count: UInt16, little endian (bytes 1..3, bits 0..11).
    ///
    /// 
    public func setCount(_ value: UInt16) {
        var buffer = self.data[1...3];
        BP3DProto.BitCodecLE.writeAligned(UInt16.self, &buffer, bitOffset: 0, bitSize: 11, value: value);

    }
    /// Bit pattern setter for field slot: UInt8, little endian (bytes 2..3, bits 3..8).
    ///
    /// 
    public func setSlot(_ value: UInt8) {
        var buffer = self.data[2...3];
        BP3DProto.BitCodecLE.writeAligned(UInt8.self, &buffer, bitOffset: 3, bitSize: 5, value: value);

    }

}
/// Definition of the raw layout for Item structure.
///
/// The raw layout wraps a bits layout and offers access
/// to the raw values (as specified in the model) of
/// each field in a structure.
public struct ArraysRawItem<T> {
    var bin: ArraysBinItem<T>
}
extension ArraysRawItem where T: BP3DProto.Buffer {
    /// Raw field accessor for id: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public var id: UInt8 {
        self.bin.id

    }
    /// Raw field accessor for count: UInt16, little endian (bytes 1..3, bits 0..11).
    ///
    /// 
    public var count: UInt16 {
        self.bin.count

    }
    /// Raw field accessor for slot: UInt8, little endian (bytes 2..3, bits 3..8).
    ///
    /// 
    public var slot: UInt8 {
        self.bin.slot

    }

}
extension ArraysRawItem where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    /// Raw field setter for id: UInt8, little endian (bytes 0..1, bits 0..8).
    ///
    /// 
    public func setId(_ value: UInt8) {
        self.bin.setId(value);

    }
    /// Raw field setter for count: UInt16, little endian (bytes 1..3, bits 0..11).
    ///
    /// 
    public func setCount(_ value: UInt16) {
        self.bin.setCount(value);

    }
    /// Raw field setter for slot: UInt8, little endian (bytes 2..3, bits 3..8).
    ///
    /// 
    public func setSlot(_ value: UInt8) {
        self.bin.setSlot(value);

    }

}
