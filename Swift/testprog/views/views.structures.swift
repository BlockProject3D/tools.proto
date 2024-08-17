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

public struct ViewsFloats<T>: BP3DProto.FixedSize {
    var data: T
    public static var size: Int { 3 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension ViewsFloats<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 3)
    }
}
public let SIZE_VIEWS_FLOATS: Int = 3;
extension ViewsFloats: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = ViewsFloats;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...3].toData());
    }
}
extension ViewsFloats: BP3DProto.FromSlice where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = ViewsFloats;
    public static func from(slice: T) throws -> BP3DProto.Message<Output> {
        if slice.size < 3 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 3, data: ViewsFloats(slice[...3]));
    }
}
extension ViewsFloats where T: BP3DProto.Buffer {
    public var rawA: UInt32 {
        BP3DProto.BitCodecLE.readUnaligned(UInt32.self, self.data[0...3], bitOffset: 0, bitSize: 17)

    }
    public var a: Float64 {
        let rawValue = Float64(self.rawA);
        return rawValue * 7.629452739355007e-5 + 0.0;
    }
    public var rawB: UInt8 {
        BP3DProto.BitCodecLE.readAligned(UInt8.self, self.data[2...3], bitOffset: 1, bitSize: 7)

    }
    public var b: Float32 {
        let rawValue = Float32(self.rawB);
        return rawValue * 0.1 + 0.0;
    }

}
extension ViewsFloats where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawA(_ value: UInt32) {
        var buffer = self.data[0...3];
        BP3DProto.BitCodecLE.writeUnaligned(UInt32.self, &buffer, bitOffset: 0, bitSize: 17, value: value);

    }
    @discardableResult
    public func setA(_ value: Float64) -> Self {
        let rawValue = value * 13107.099999999999 + -0.0;
        self.setRawA(UInt32(rawValue));
        return self;
    }
    public func setRawB(_ value: UInt8) {
        var buffer = self.data[2...3];
        BP3DProto.BitCodecLE.writeAligned(UInt8.self, &buffer, bitOffset: 1, bitSize: 7, value: value);

    }
    @discardableResult
    public func setB(_ value: Float32) -> Self {
        let rawValue = value * 10.0 + 0.0;
        self.setRawB(UInt8(rawValue));
        return self;
    }

}
