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

import Foundation

public struct List<Buffer: BP3DProto.Buffer, T: FromSlice, Item: FromSlice>: FromSlice where T.Output: Scalar, T.Buffer == Buffer, Item.Buffer == Buffer {
    var buffer: Buffer;
    var _count: Int;

    public var count: Int {
        _count
    }

    public init(_ buffer: Buffer, count: Int) {
        self._count = count;
        self.buffer = buffer;
    }

    public typealias Output = List<Buffer, T, Item>;

    public static func from(slice: Buffer) throws -> Message<List<Buffer, T, Item>> {
        let msg = try T.from(slice: slice);
        var data = slice[msg.size...];
        var totalSize = msg.size;
        for _ in 0...msg.data.toUInt() - 1 {
            let item = try Item.from(slice: data);
            totalSize += item.size;
            data = data[item.size...];
        }
        return Message(size: totalSize, data: List(slice[msg.size...], count: Int(msg.data.toUInt())));
    }

    public func toArray() throws -> [Item.Output] {
        var data = buffer;
        var items: [Item.Output] = [];
        items.reserveCapacity(_count);
        for _ in 0..._count - 1 {
            let item = try Item.from(slice: data);
            items.append(item.data);
            data = data[item.size...];
        }
        return items;
    }
}

extension List: WriteTo where T: WriteTo, T.Input: Scalar {
    public init(_ buffer: Buffer) {
        self.buffer = buffer;
        self._count = 0;
    }

    public typealias Input = List<Buffer, T, Item>;

    public static func write<B>(input: List<Buffer, T, Item>, to out: inout B) throws where B : WritableBuffer {
        try T.write(input: T.Input(fromUInt: UInt(input._count)), to: &out);
        out.write(bytes: input.buffer.toData());
    }
}

extension List where Item: WriteTo, Buffer: WritableBuffer {
    public mutating func writeItem(_ item: Item.Input) throws {
        try Item.write(input: item, to: &buffer);
        self._count += 1;
    }

    public mutating func writeItems(_ items: [Item.Input]) throws {
        for item in items {
            try self.writeItem(item);
        }
    }
}

public struct UnsizedList<Buffer: BP3DProto.Buffer, T: FromSlice, Item: FromSlice>: FromSlice where T.Output: Scalar, T.Buffer == Buffer, Item.Buffer == Buffer {
    public typealias Output = List<Buffer, T, Item>;

    public static func from(slice: Buffer) throws -> Message<List<Buffer, T, Item>> {
        let msg = try T.from(slice: slice);
        let data = slice[msg.size...];
        return Message(size: slice.size, data: List(data, count: Int(msg.data.toUInt())));
    }
}
