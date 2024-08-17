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

public struct TestIntContainer<T>: BP3DProto.FixedSize {
    var data: T
    public static var size: Int { 4 }
    public init(_ data: T) {
        self.data = data;
    }
}
extension TestIntContainer<BP3DProto.DataBuffer> {
    public init() {
        self.data = BP3DProto.DataBuffer(size: 4)
    }
}
public let SIZE_TEST_INT_CONTAINER: Int = 4;
extension TestIntContainer: BP3DProto.WriteTo where T: BP3DProto.Buffer {
    public typealias Input = TestIntContainer;
    public static func write<B: BP3DProto.WritableBuffer>(input: Input, to out: inout B) throws {
        out.write(bytes: input.data[...4].toData());
    }
}
extension TestIntContainer: BP3DProto.FromSlice where T: BP3DProto.Buffer {
    public typealias Buffer = T;
    public typealias Output = TestIntContainer;
    public static func from(slice: T) throws -> BP3DProto.Message<Output> {
        if slice.size < 4 {
            throw BP3DProto.Error.truncated;
        }
        return BP3DProto.Message(size: 4, data: TestIntContainer(slice[...4]));
    }
}
extension TestIntContainer where T: BP3DProto.Buffer {
    public var rawTestInt: UInt32 {
        BP3DProto.ByteCodecLE.readAligned(UInt32.self, self.data[0...4])

    }
    public var testInt: UInt32 {
        self.rawTestInt
    }

}
extension TestIntContainer where T: BP3DProto.Buffer, T: BP3DProto.WritableBuffer {
    public func setRawTestInt(_ value: UInt32) {
        var buffer = self.data[0...4];
        BP3DProto.ByteCodecLE.writeAligned(UInt32.self, &buffer, value: value);

    }
    @discardableResult
    public func setTestInt(_ value: UInt32) -> Self {
        self.setRawTestInt(value);
        return self;
    }

}