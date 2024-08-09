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

public protocol Scalar {
    static var size: Int {get};

    init(fromBytesLE slice: Data);
    init(fromBytesBE slice: Data);
    init(fromUInt value: UInt);
    func toBytesLE(_ slice: inout Data);
    func toBytesBE(_ slice: inout Data);
    func toUInt() -> UInt;
}

extension UInt64: Scalar {
    public init(fromUInt value: UInt) {
        self = UInt64(value);
    }
    
    public func toUInt() -> UInt {
        return UInt(self);
    }

    public init(fromBytesBE slice: Data) {
        let motherfuckingswift = UInt64(UInt32(fromBytesBE: slice)) << 32 | UInt64(UInt32(fromBytesBE: Data(slice[4...])));
        self = motherfuckingswift;
    }

    public init(fromBytesLE slice: Data) {
        let motherfuckingswift = UInt64(UInt32(fromBytesBE: slice)) << 32 | UInt64(UInt32(fromBytesBE: Data(slice[4...])));
        self = motherfuckingswift.byteSwapped;
    }

    public static var size: Int {
        8
    }

    public func toBytesLE(_ slice: inout Data) {
        let value = self.littleEndian;
        slice.append(UInt8(truncatingIfNeeded: value));
        slice.append(UInt8(truncatingIfNeeded: value >> 8));
        slice.append(UInt8(truncatingIfNeeded: value >> 16));
        slice.append(UInt8(truncatingIfNeeded: value >> 24));
        slice.append(UInt8(truncatingIfNeeded: value >> 32));
        slice.append(UInt8(truncatingIfNeeded: value >> 40));
        slice.append(UInt8(truncatingIfNeeded: value >> 48));
        slice.append(UInt8(truncatingIfNeeded: value >> 56));
    }

    public func toBytesBE(_ slice: inout Data) {
        let value = self.bigEndian;
        slice.append(UInt8(truncatingIfNeeded: value));
        slice.append(UInt8(truncatingIfNeeded: value >> 8));
        slice.append(UInt8(truncatingIfNeeded: value >> 16));
        slice.append(UInt8(truncatingIfNeeded: value >> 24));
        slice.append(UInt8(truncatingIfNeeded: value >> 32));
        slice.append(UInt8(truncatingIfNeeded: value >> 40));
        slice.append(UInt8(truncatingIfNeeded: value >> 48));
        slice.append(UInt8(truncatingIfNeeded: value >> 56));
    }
}

extension UInt32: Scalar {
    public init(fromUInt value: UInt) {
        self = UInt32(value);
    }

    public func toUInt() -> UInt {
        return UInt(self);
    }

    public init(fromBytesBE slice: Data) {
        let value = UInt32(slice[0]) << 24 | UInt32(slice[1]) << 16 | UInt32(slice[2]) << 8 | UInt32(slice[3]);
        self = value;
    }

    public init(fromBytesLE slice: Data) {
        let value = UInt32(slice[3]) << 24 | UInt32(slice[2]) << 16 | UInt32(slice[1]) << 8 | UInt32(slice[0]);
        self = value;
    }

    public static var size: Int {
        4
    }

    public func toBytesLE(_ slice: inout Data) {
        let value = self.littleEndian;
        slice.append(UInt8(truncatingIfNeeded: value));
        slice.append(UInt8(truncatingIfNeeded: value >> 8));
        slice.append(UInt8(truncatingIfNeeded: value >> 16));
        slice.append(UInt8(truncatingIfNeeded: value >> 24));
    }

    public func toBytesBE(_ slice: inout Data) {
        let value = self.bigEndian;
        slice.append(UInt8(truncatingIfNeeded: value));
        slice.append(UInt8(truncatingIfNeeded: value >> 8));
        slice.append(UInt8(truncatingIfNeeded: value >> 16));
        slice.append(UInt8(truncatingIfNeeded: value >> 24));
    }
}

extension UInt16: Scalar {
    public init(fromUInt value: UInt) {
        self = UInt16(value);
    }

    public func toUInt() -> UInt {
        return UInt(self);
    }

    public init(fromBytesBE slice: Data) {
        let value = UInt16(slice[0]) << 8 | UInt16(slice[1]);
        self = value;
    }

    public init(fromBytesLE slice: Data) {
        let value = UInt16(slice[1]) << 8 | UInt16(slice[0]);
        self = value;
    }

    public static var size: Int {
        2
    }

    public func toBytesLE(_ slice: inout Data) {
        let value = self.littleEndian;
        slice.append(UInt8(truncatingIfNeeded: value));
        slice.append(UInt8(truncatingIfNeeded: value >> 8));
    }

    public func toBytesBE(_ slice: inout Data) {
        let value = self.bigEndian;
        slice.append(UInt8(truncatingIfNeeded: value));
        slice.append(UInt8(truncatingIfNeeded: value >> 8));
    }
}

extension UInt8: Scalar {
    public init(fromUInt value: UInt) {
        self = UInt8(value);
    }

    public func toUInt() -> UInt {
        return UInt(self);
    }

    public init(fromBytesBE slice: Data) {
        self = slice[0]
    }

    public init(fromBytesLE slice: Data) {
        self = slice[0]
    }

    public static var size: Int {
        1
    }

    public func toBytesLE(_ slice: inout Data) {
        slice.append(self);
    }

    public func toBytesBE(_ slice: inout Data) {
        slice.append(self);
    }
}
