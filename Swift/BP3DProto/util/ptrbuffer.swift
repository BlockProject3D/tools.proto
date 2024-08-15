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

public struct PtrBuffer: Buffer {
    var data: UnsafeRawBufferPointer
    let start: Int;
    let end: Int;
    var cursor: Int;

    public init(_ ptr: UnsafeRawBufferPointer) {
        self.start = 0;
        self.end = ptr.count;
        self.cursor = 0;
        self.data = ptr;
    }

    public init(_ ptr: UnsafeMutableRawBufferPointer) {
        self.start = 0;
        self.end = ptr.count;
        self.cursor = 0;
        self.data = UnsafeRawBufferPointer(ptr);
    }

    private init(from: Self, start: Int, end: Int) {
        self.data = from.data;
        self.start = start;
        self.end = end;
        self.cursor = from.cursor;
    }

    public subscript(index: ClosedRange<Int>) -> PtrBuffer {
        let start = index.lowerBound;
        let end = index.upperBound;
        assert(end - start >= 0);
        assert(start <= size);
        assert(end <= size);
        assert(start >= 0);
        assert(end >= 0);
        return PtrBuffer(from: self, start: self.start + start, end: self.start + end);
    }

    public subscript(index: PartialRangeFrom<Int>) -> PtrBuffer {
        assert(index.lowerBound <= size);
        assert(index.lowerBound > 0);
        return PtrBuffer(from: self, start: self.start + index.lowerBound, end: self.end);
    }

    public subscript(index: PartialRangeThrough<Int>) -> PtrBuffer {
        assert(index.upperBound <= size);
        assert(index.upperBound > 0);
        return PtrBuffer(from: self, start: self.start, end: self.start + index.upperBound);
    }

    public subscript(index: Int) -> UInt8 {
        assert(index < size);
        return self.data[self.start + index];
    }

    public var size: Int {
        end - start
    }

    public func toData() -> Data {
        return Data(self.data[self.start...self.end - 1]);
    }
}

public struct MutPtrBuffer: Buffer, WritableBuffer {
    var data: UnsafeMutableRawBufferPointer
    let start: Int;
    let end: Int;
    var cursor: Int;

    public init(_ ptr: UnsafeMutableRawBufferPointer) {
        self.start = 0;
        self.end = ptr.count;
        self.cursor = 0;
        self.data = ptr;
    }

    private init(from: Self, start: Int, end: Int) {
        self.data = from.data;
        self.start = start;
        self.end = end;
        self.cursor = from.cursor;
    }

    public subscript(index: ClosedRange<Int>) -> MutPtrBuffer {
        let start = index.lowerBound;
        let end = index.upperBound;
        assert(end - start >= 0);
        assert(start <= size);
        assert(end <= size);
        assert(start >= 0);
        assert(end >= 0);
        return MutPtrBuffer(from: self, start: self.start + start, end: self.start + end);
    }

    public subscript(index: PartialRangeFrom<Int>) -> MutPtrBuffer {
        assert(index.lowerBound <= size);
        assert(index.lowerBound > 0);
        return MutPtrBuffer(from: self, start: self.start + index.lowerBound, end: self.end);
    }

    public subscript(index: PartialRangeThrough<Int>) -> MutPtrBuffer {
        assert(index.upperBound <= size);
        assert(index.upperBound > 0);
        return MutPtrBuffer(from: self, start: self.start, end: self.start + index.upperBound);
    }

    public subscript(index: Int) -> UInt8 {
        assert(index < size);
        return self.data[self.start + index];
    }

    public var size: Int {
        end - start
    }

    public func toData() -> Data {
        return Data(self.data[self.start...self.end - 1]);
    }

    public mutating func write(bytes: Data) {
        bytes.copyBytes(to: UnsafeMutableRawBufferPointer(rebasing: self.data[self.cursor...]), count: bytes.count);
    }

    public mutating func write<S>(bytes: S) where S : Sequence, S.Element == UInt8 {
        self.write(bytes: Data(bytes))
    }

    public mutating func write(byte: UInt8) {
        self.data[self.cursor] = byte;
        self.cursor += 1;
    }
}
