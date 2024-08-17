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

public struct EnumsHeader<T>: BP3DProto.FixedSize {
    var data: T
    public static var size: Int { 1 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension EnumsHeader<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 1)
    }
}
public let SIZE_ENUMS_HEADER: Int = 1;
extension EnumsHeader: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = EnumsHeader;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data.toData());
    }
}
extension EnumsHeader: BP3DProto.FromSlice where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = EnumsHeader;
    public static func from(slice: T) throws -> BP3DProto.Message<Output> {
        if slice.size < 1 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 1, data: EnumsHeader(slice[...1]));
    }
}
extension EnumsHeader where T: BP3DProto.Buffer {
    public var rawType: UInt8 {
        BP3DProto.ByteCodecLE.readAligned(UInt8.self, self.data[0...1])

    }
    public var type: EnumsType? {
        let rawValue = self.rawType;
        return EnumsType(rawValue: rawValue);
    }

}
extension EnumsHeader where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawType(_ value: UInt8) {
        var buffer = self.data[0...1];
        BP3DProto.ByteCodecLE.writeAligned(UInt8.self, &buffer, value: value);

    }
    @discardableResult
    public func setType(_ value: EnumsType) -> Self {
        self.setRawType(UInt8(value.rawValue));
        return self;
    }

}
