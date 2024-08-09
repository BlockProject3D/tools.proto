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

public struct List<T: FromSlice, Item: FromSlice>: FromSlice where T.Output: Scalar {
    public typealias Output = [Item.Output];

    public static func from<B>(slice: B) throws -> Message<[Item.Output]> where B : Buffer {
        let msg = try T.from(slice: slice);
        var data = slice[msg.size...];
        var totalSize = msg.size;
        var items: [Item.Output] = [];
        items.reserveCapacity(Int(msg.data.toUInt()));
        for _ in 0...msg.data.toUInt() - 1 {
            let item = try Item.from(slice: data);
            totalSize += item.size;
            items.append(item.data);
            data = data[item.size...]
        }
        return Message(size: totalSize, data: items);
    }
}

extension List: WriteTo where T: WriteTo, T.Input: Scalar, Item: WriteTo {
    public typealias Input = [Item.Input]

    public static func write<B>(input: [Item.Input], to out: inout B) throws where B : WritableBuffer {
        try T.write(input: T.Input(fromUInt: UInt(input.count)), to: &out);
        for item in input {
            try Item.write(input: item, to: &out);
        }
    }
}

public struct Array<T: FromSlice, Item: FromSlice>: FromSlice where T.Output: Scalar, Item.Output: FixedSize {
    public typealias Output = [Item.Output];

    public static func from<B>(slice: B) throws -> Message<[Item.Output]> where B : Buffer {
        let msg = try T.from(slice: slice);
        var data = slice[msg.size...];
        let totalSize = msg.size + Int(msg.data.toUInt()) * Item.Output.size;
        if slice.size < totalSize {
            throw Error.truncated;
        }
        var items: [Item.Output] = [];
        items.reserveCapacity(Int(msg.data.toUInt()));
        for _ in 0...msg.data.toUInt() - 1 {
            let item = try Item.from(slice: data);
            items.append(item.data);
            data = data[item.size...]
        }
        return Message(size: totalSize, data: items);
    }
}

extension Array: WriteTo where T: WriteTo, T.Input: Scalar, Item: WriteTo {
    public typealias Input = [Item.Input]

    public static func write<B>(input: [Item.Input], to out: inout B) throws where B : WritableBuffer {
        try T.write(input: T.Input(fromUInt: UInt(input.count)), to: &out);
        for item in input {
            try Item.write(input: item, to: &out);
        }
    }
}
