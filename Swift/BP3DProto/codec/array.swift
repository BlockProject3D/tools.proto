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

public struct ArrayCodec<B: Buffer, C: ByteCodec, Item: FromBytes> {
    var buffer: B;
    let itemByteSize: Int;

    public var count: Int {
        buffer.size / itemByteSize
    }

    public init(buffer: B, itemBitSize: Int) {
        self.buffer = buffer
        self.itemByteSize = itemBitSize / 8;
    }

    public subscript(index: Int) -> Item {
        get {
            let pos = index * itemByteSize;
            let end = pos + itemByteSize;
            return C.read(Item.self, buffer[pos...end]);
        }
    }
}

extension ArrayCodec where B: WritableBuffer {
    @discardableResult
    public func set(_ index: Int, _ newvalue: Item) -> Self {
        let pos = index * itemByteSize;
        let end = pos + itemByteSize;
        var b = buffer[pos...end];
        C.write(Item.self, &b, value: newvalue);
        return self;
    }
}

extension ArrayCodec where Item == UInt8 {
    public func toData() -> Data {
        self.buffer.toData()
    }
}

extension ArrayCodec where Item == UInt8, B: WritableBuffer {
    public func fromData(_ bytes: Data) {
        var b = buffer[0...];
        b.write(bytes: bytes);
    }
}
