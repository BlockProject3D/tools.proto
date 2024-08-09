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

public struct NullTerminatedString: FromSlice, WriteTo {
    public typealias Output = String;
    public typealias Input = String;

    public static func from<B: Buffer>(slice: B) throws -> Message<String> {
        guard let index = slice.findFirst(0x0) else { throw Error.truncated };
        let str = String(decoding: slice[...index].toData(), as: UTF8.self);
        return Message(size: index + 1, data: str);
    }

    public static func write<B: WritableBuffer>(input: String, to out: inout B) throws {
        let str = input.utf8;
        try out.write(bytes: str);
        try out.write(byte: UInt8(0x0));
    }
}

public struct VarcharString<T: FromSlice>: FromSlice where T.Output: Scalar {
    public typealias Output = String;

    public static func from<B: Buffer>(slice: B) throws -> Message<String> {
        let size = try T.from(slice: slice);
        let length = size.data.toUInt();
        let data = slice[size.size...size.size + Int(length)].toData();
        let str = String(decoding: data, as: UTF8.self);
        return Message(size: Int(length) + size.size, data: str);
    }
}

extension VarcharString: WriteTo where T: WriteTo, T.Input: Scalar {
    public typealias Input = String;

    public static func write<B: WritableBuffer>(input: String, to out: inout B) throws {
        let str = input.utf8;
        let length = T.Input(fromUInt: UInt(str.count));
        try T.write(input: length, to: &out);
        try out.write(bytes: str);
    }
}
