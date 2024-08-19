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

public struct Array<Buffer: BP3DProto.Buffer, T: FromSlice, Item: FromSlice>: FromSlice where T.Output: Scalar, Item.Output: FixedSize, T.Buffer == Buffer, Item.Buffer == Buffer {
    let buffer: Buffer;
    public let count: Int;

    public typealias Output = Self;

    public static func from(slice: Buffer) throws -> Message<Self> {
        let msg = try T.from(slice: slice);
        let data = slice[msg.size...];
        let totalSize = msg.size + Int(msg.data.toUInt()) * Item.Output.size;
        if slice.size < totalSize {
            throw Error.truncated;
        }
        let len = Int(msg.data.toUInt());
        return Message(size: totalSize, data: Array(data[...(len * Item.Output.size)], count: len));
    }

    private init(_ buffer: Buffer, count: Int) {
        self.count = count;
        self.buffer = buffer;
    }

    public init(_ buffer: Buffer) {
        self.buffer = buffer
        self.count = self.buffer.size / Item.Output.size
    }

    public subscript(index: Int) -> Item.Output where Item.Output: FromBuffer, Item.Output.Buffer == Buffer {
        let pos = index * Item.Output.size;
        let end = pos + Item.Output.size;
        return Item.Output(self.buffer[pos...end]);
    }

    public func get(_ index: Int) -> Item.Output? where Item.Output: FromBuffer, Item.Output.Buffer == Buffer {
        if index >= count {
            return nil;
        }
        let pos = index * Item.Output.size;
        let end = pos + Item.Output.size;
        return Item.Output(self.buffer[pos...end]);
    }

    public func toArray() -> [Item.Output] where Item.Output: FromBuffer, Item.Output.Buffer == Buffer {
        var arr: [Item.Output] = [];
        arr.reserveCapacity(count);
        for i in 0...count - 1 {
            arr.append(self[i])
        }
        return arr;
    }
}

extension Array: WriteTo where T: WriteTo, T.Input: Scalar, Item: WriteTo {
    public typealias Input = Self;

    public static func write<B>(input: Self, to out: inout B) throws where B : WritableBuffer {
        try T.write(input: T.Input(fromUInt: UInt(input.count)), to: &out);
        out.write(bytes: input.buffer.toData());
    }
}
