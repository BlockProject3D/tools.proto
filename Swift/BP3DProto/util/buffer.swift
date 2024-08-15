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
    func copyTo(ptr: UnsafeMutableRawBufferPointer, size: Int);
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

    public func copyTo(ptr: UnsafeMutableRawBufferPointer, size: Int) {
        for i in 0...size - 1 {
            ptr.storeBytes(of: self[i], toByteOffset: i, as: UInt8.self);
        }
    }
}

public protocol WritableBuffer {
    mutating func write(bytes: Data);
    mutating func write<S: Sequence<UInt8>>(bytes: S);
    mutating func write(byte: UInt8);
}

public struct DataBuffer: Buffer, WritableBuffer {
    private let data: NSData;
    private var dataMut: NSMutableData?;
    private let start: Int;
    private var end: Int;
    private var cursor: Int;

    private func get() -> NSData {
        if let dataMut = self.dataMut {
            return dataMut;
        }
        return self.data;
    }

    private mutating func getMut() -> NSMutableData {
        if self.dataMut == nil {
            self.dataMut = self.data.mutableCopy() as? NSMutableData;
        }
        return self.dataMut!;
    }

    private init(from: DataBuffer, start: Int, end: Int) {
        self.start = start;
        self.end = end;
        self.cursor = start;
        self.data = from.data;
        self.dataMut = from.dataMut;
    }

    public init() {
        self.start = 0;
        self.end = 0;
        self.cursor = 0;
        self.dataMut = NSMutableData();
        self.data = self.dataMut!;
    }

    public init(size: Int) {
        self.start = 0;
        self.end = size;
        self.cursor = 0;
        self.dataMut = NSMutableData(length: size);
        self.data = self.dataMut!;
    }

    public init<S: Sequence<UInt8>>(_ bytes: S) {
        self.data = Data(bytes) as NSData;
        self.start = 0;
        self.end = self.data.count;
        self.cursor = 0;
        self.dataMut = nil;
    }

    public init<S: Sequence<UInt8>>(mut bytes: S) {
        self.data = Data(bytes) as NSData;
        self.start = 0;
        self.end = self.data.count;
        self.cursor = 0;
        self.dataMut = self.data.mutableCopy() as? NSMutableData;
    }

    public init(mut bytes: NSMutableData) {
        self.data = bytes;
        self.dataMut = bytes;
        self.start = 0;
        self.end = bytes.length;
        self.cursor = start;
    }

    public init(mut ptr: UnsafeMutableRawBufferPointer) {
        self.data = NSData(bytes: ptr.baseAddress!, length: ptr.count);
        self.dataMut = NSMutableData(bytes: ptr.baseAddress!, length: ptr.count);
        self.start = 0;
        self.end = ptr.count;
        self.cursor = start;
    }

    public init(_ ptr: UnsafeRawBufferPointer) {
        self.data = NSData(bytes: ptr.baseAddress!, length: ptr.count);
        self.dataMut = nil;
        self.start = 0;
        self.end = ptr.count;
        self.cursor = start;
    }

    public init(_ bytes: NSData) {
        self.data = bytes;
        self.dataMut = nil;
        self.start = 0;
        self.end = data.length;
        self.cursor = start;
    }

    public var size: Int {
        return end - start;
    }

    public mutating func clear() {
        self.cursor = self.start;
        self.end = self.start;
    }

    public func copyTo(ptr: UnsafeMutableRawBufferPointer, size: Int) {
        assert(size <= self.size);
        self.get().copyBytes(to: ptr, from: self.start...self.start + size)
    }

    public func findFirst(_ value: UInt8) -> Int? {
        return self.get()[self.start...].firstIndex(of: value);
    }

    public subscript(index: ClosedRange<Int>) -> DataBuffer {
        let start = index.lowerBound;
        let end = index.upperBound;
        assert(end - start >= 0);
        assert(start <= size);
        assert(end <= size);
        assert(start >= 0);
        assert(end >= 0);
        return DataBuffer(from: self, start: self.start + start, end: self.start + end);
    }

    public subscript(index: PartialRangeFrom<Int>) -> DataBuffer {
        assert(index.lowerBound <= size);
        assert(index.lowerBound > 0);
        return DataBuffer(from: self, start: self.start + index.lowerBound, end: self.end);
    }

    public subscript(index: PartialRangeThrough<Int>) -> DataBuffer {
        assert(index.upperBound <= size);
        assert(index.upperBound > 0);
        return DataBuffer(from: self, start: self.start, end: self.start + index.upperBound);
    }

    public subscript(index: Int) -> UInt8 {
        assert(index < size);
        return self.get()[self.start + index];
    }

    public mutating func write(bytes: Data) {
        let data = self.getMut();
        if self.cursor + bytes.count < data.count {
            bytes.withUnsafeBytes { ptr in
                data.replaceBytes(in: NSRange(location: self.cursor, length: bytes.count), withBytes: ptr);
            }
            //data.replaceSubrange(self.cursor...self.cursor + bytes.count - 1, with: bytes);
        } else {
            let maxLen = data.count - self.cursor;
            if maxLen > 0 {
                bytes.withUnsafeBytes { ptr in
                    data.replaceBytes(in: NSRange(location: self.cursor, length: maxLen), withBytes: ptr);
                }
                //data.replaceSubrange(self.cursor...self.cursor + maxLen - 1, with: bytes[0...maxLen - 1]);
            }
            let remaining = bytes.count - maxLen;
            if remaining > 0 {
                bytes[maxLen...].withUnsafeBytes { ptr in
                    data.append(ptr, length: remaining);
                }
                //data.append(contentsOf: bytes[maxLen...maxLen + remaining - 1]);
            }
        }
        self.cursor += bytes.count;
        self.end = self.cursor;
    }

    public mutating func write<S: Sequence<UInt8>>(bytes: S) {
        self.write(bytes: Data(bytes));
    }

    public mutating func write(byte: UInt8) {
        let data = self.getMut();
        if self.cursor + 1 < data.count {
            withUnsafeBytes(of: byte, { ptr in
                data.replaceBytes(in: NSRange(location: self.cursor, length: 1), withBytes: ptr.baseAddress!);
            });
            //data[self.cursor] = byte;
        } else {
            //data.append(byte);
            withUnsafeBytes(of: byte, { ptr in
                data.append(ptr.baseAddress!, length: 1);
            });
        }
        self.cursor += 1;
        self.end = self.cursor;
    }

    public func toData() -> Data {
        return Data(self.get()[self.start...self.end - 1]);
    }
}
