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

public struct Optional<T: FromSlice>: FromSlice {
    public typealias Output = T.Output?;

    public static func from(slice: Data) throws -> Message<T.Output?> {
        if slice.isEmpty {
            throw Error.truncated;
        }
        let b = slice[0] > 0;
        if b {
            let msg = try T.from(slice: slice[1...]);
            return Message(size: msg.size, data: msg.data);
        } else {
            return Message(size: 1, data: nil);
        }
    }
}

extension Optional: WriteTo where T: WriteTo {
    public typealias Input = T.Input?

    public static func write(input: T.Input?, to out: inout Data) throws {
        let b = input != nil ? 1 : 0;
        out.append(UInt8(b));
        if let input = input {
            try T.write(input: input, to: &out);
        }
    }
}

public struct ValueLE<T: Scalar>: FromSlice, WriteTo {
    public typealias Input = T;
    public typealias Output = T;

    public static func from(slice: Data) throws -> Message<T> {
        if slice.count < T.size {
            throw Error.truncated;
        }
        return Message(size: T.size, data: T(fromBytesLE: slice));
    }

    public static func write(input: T, to out: inout Data) throws {
        input.toBytesLE(&out);
    }
}

public struct ValueBE<T: Scalar>: FromSlice, WriteTo {
    public typealias Input = T;
    public typealias Output = T;

    public static func from(slice: Data) throws -> Message<T> {
        if slice.count < T.size {
            throw Error.truncated;
        }
        return Message(size: T.size, data: T(fromBytesBE: slice));
    }

    public static func write(input: T, to out: inout Data) throws {
        input.toBytesBE(&out);
    }
}
