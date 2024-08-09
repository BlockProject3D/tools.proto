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

public protocol Buffer {
    subscript(index: ClosedRange<Int>) -> Self { get }
    subscript(index: PartialRangeFrom<Int>) -> Self { get }
    subscript(index: PartialRangeThrough<Int>) -> Self { get }
    subscript(index: Int) -> UInt8 { get }
    func findFirst(_ value: UInt8) -> Int?;
    var size: Int { get }
    var isEmpty: Bool { get }
    func toData() -> Data;
}

extension Buffer {
    public var isEmpty: Bool {
        return size == 0;
    }

    public func findFirst(_ value: UInt8) -> Int? {
        var i = 0;
        while i < self.size {
            if self[i] == value {
                return i;
            }
            i += 1;
        }
        return nil;
    }
}

public protocol WritableBuffer {
    mutating func write(bytes: Data) throws;
    mutating func write<S: Sequence<UInt8>>(bytes: S) throws;
    mutating func write(byte: UInt8) throws;
}

public struct DataBuffer: Buffer, WritableBuffer {
    private var data: Data;
    private let start: Int;
    private var end: Int;
    private var cursor: Int;

    public init() {
        self.init(bytes: Data());
    }

    public func findFirst(_ value: UInt8) -> Int? {
        return self.data[self.start...].firstIndex(of: value);
    }

    public init(bytes: any Sequence<UInt8>) {
        self.init(bytes: Data(bytes));
    }

    public init(bytes data: Data) {
        self.init(bytes: data, start: 0, end: data.count);
    }

    public init(bytes data: Data, start: Int, end: Int) {
        self.data = data;
        self.start = start;
        self.end = end;
        self.cursor = start;
    }

    public var size: Int {
        return end - start;
    }

    public mutating func clear() {
        self.cursor = self.start;
        self.end = self.start;
    }

    public subscript(index: ClosedRange<Int>) -> DataBuffer {
        let start = index.lowerBound;
        let end = index.upperBound;
        assert(end - start > 0);
        assert(start < size);
        assert(end <= size);
        assert(start > 0);
        assert(end > 0);
        return DataBuffer(bytes: self.data, start: self.start + start, end: self.start + end);
    }

    public subscript(index: PartialRangeFrom<Int>) -> DataBuffer {
        assert(index.lowerBound < size);
        assert(index.lowerBound > 0);
        return DataBuffer(bytes: self.data, start: self.start + index.lowerBound, end: self.end);
    }

    public subscript(index: PartialRangeThrough<Int>) -> DataBuffer {
        assert(index.upperBound < size);
        assert(index.upperBound > 0);
        return DataBuffer(bytes: self.data, start: self.start, end: self.start + index.upperBound);
    }

    public subscript(index: Int) -> UInt8 {
        assert(index < size);
        return self.data[self.start + index];
    }

    public mutating func write(bytes: Data) throws {
        if self.cursor + bytes.count < self.data.count {
            self.data.replaceSubrange(self.cursor...self.cursor + bytes.count - 1, with: bytes);
        } else {
            let maxLen = self.data.count - self.cursor;
            if maxLen > 0 {
                self.data.replaceSubrange(self.cursor...self.cursor + maxLen - 1, with: bytes[0...maxLen - 1]);
            }
            let remaining = bytes.count - maxLen;
            if remaining > 0 {
                self.data.append(contentsOf: bytes[maxLen...maxLen + remaining - 1]);
            }
        }
        self.cursor += bytes.count;
        self.end = self.cursor;
    }

    public mutating func write<S: Sequence<UInt8>>(bytes: S) throws {
        try self.write(bytes: Data(bytes));
    }

    public mutating func write(byte: UInt8) throws {
        if self.cursor + 1 < self.data.count {
            self.data[self.cursor] = byte;
        } else {
            self.data.append(byte);
        }
        self.cursor += 1;
        self.end = self.cursor;
    }

    public func toData() -> Data {
        return Data(self.data[self.start...self.end - 1]);
    }
}
