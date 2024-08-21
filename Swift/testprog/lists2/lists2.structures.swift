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
    var data: T
    public static var size: Int { 16 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension Lists2Times<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 16)
    }
}
public let SIZE_LISTS2_TIMES: Int = 16;
extension Lists2Times: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = Lists2Times;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...16].toData());
    }
}
extension Lists2Times: BP3DProto.FromSlice where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = Lists2Times;
    public static func from(slice: T) throws -> BP3DProto.Message<Output> {
        if slice.size < 16 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 16, data: Lists2Times(slice[...16]));
    }
}
extension Lists2Times where T: BP3DProto.Buffer {
    public var rawStart: UInt64 {
        BP3DProto.ByteCodecLE.readAligned(UInt64.self, self.data[0...8])

    }
    public var start: UInt64 {
        self.rawStart
    }
    public var rawEnd: UInt64 {
        BP3DProto.ByteCodecLE.readAligned(UInt64.self, self.data[8...16])

    }
    public var end: UInt64 {
        self.rawEnd
    }

}
extension Lists2Times where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawStart(_ value: UInt64) {
        var buffer = self.data[0...8];
        BP3DProto.ByteCodecLE.writeAligned(UInt64.self, &buffer, value: value);

    }
    @discardableResult
    public func setStart(_ value: UInt64) -> Self {
        self.setRawStart(value);
        return self;
    }
    public func setRawEnd(_ value: UInt64) {
        var buffer = self.data[8...16];
        BP3DProto.ByteCodecLE.writeAligned(UInt64.self, &buffer, value: value);

    }
    @discardableResult
    public func setEnd(_ value: UInt64) -> Self {
        self.setRawEnd(value);
        return self;
    }

}
