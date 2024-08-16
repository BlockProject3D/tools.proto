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

import XCTest
import BP3DProto

final class BP3DProtoTests: XCTestCase {
    override func setUpWithError() throws {
    }

    override func tearDownWithError() throws {
    }

    func testValueLE() throws {
        let data = DataBuffer([0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x67, 0x89]);
        let value8 = try ValueLE<DataBuffer, UInt8>.from(slice: data).data;
        let value16 = try ValueLE<DataBuffer, UInt16>.from(slice: data).data;
        let value32 = try ValueLE<DataBuffer, UInt32>.from(slice: data).data;
        let value64 = try ValueLE<DataBuffer, UInt64>.from(slice: data).data;
        XCTAssertEqual(value8, 0xAB);
        XCTAssertEqual(value16, 0xCDAB);
        XCTAssertEqual(value32, 0x12EFCDAB);
        XCTAssertEqual(value64, 0x8967563412EFCDAB);
    }

    func testValueBE() throws {
        let data = DataBuffer([0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x67, 0x89]);
        let value8 = try ValueBE<DataBuffer, UInt8>.from(slice: data).data;
        let value16 = try ValueBE<DataBuffer, UInt16>.from(slice: data).data;
        let value32 = try ValueBE<DataBuffer, UInt32>.from(slice: data).data;
        let value64 = try ValueBE<DataBuffer, UInt64>.from(slice: data).data;
        XCTAssertEqual(value8, 0xAB);
        XCTAssertEqual(value16, 0xABCD);
        XCTAssertEqual(value32, 0xABCDEF12);
        XCTAssertEqual(value64, 0xABCDEF1234566789);
    }

    func testValueLEWrite() throws {
        var data = DataBuffer();
        try ValueLE<DataBuffer, UInt64>.write(input: 0xABCDEF1234566789, to: &data);
        XCTAssertEqual(data.toData(), Data([0x89, 0x67, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB]));
        let value64 = try ValueLE<DataBuffer, UInt64>.from(slice: data).data;
        XCTAssertEqual(value64, 0xABCDEF1234566789);
    }

    func testValueBEWrite() throws {
        var data = DataBuffer();
        try ValueBE<DataBuffer, UInt64>.write(input: 0xABCDEF1234566789, to: &data);
        XCTAssertEqual(data.toData(), Data([0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x67, 0x89]));
        let value64 = try ValueBE<DataBuffer, UInt64>.from(slice: data).data;
        XCTAssertEqual(value64, 0xABCDEF1234566789);
    }

    func testNullTerminatedString() throws {
        var data = DataBuffer();
        try NullTerminatedString<DataBuffer>.write(input: "test", to: &data);
        let str = try NullTerminatedString.from(slice: data).data;
        XCTAssertEqual(str, "test");
        data.clear();
        try NullTerminatedString<DataBuffer>.write(input: "toto", to: &data);
        let str1 = try NullTerminatedString.from(slice: data).data;
        XCTAssertEqual(str1, "toto");
    }

    func testVarcharString() throws {
        var data = DataBuffer();
        try VarcharString<DataBuffer, ValueLE<DataBuffer, UInt8>>.write(input: "test", to: &data);
        let str = try VarcharString<DataBuffer, ValueLE<DataBuffer, UInt8>>.from(slice: data).data;
        XCTAssertEqual(str, "test");
        data.clear();
        try VarcharString<DataBuffer, ValueBE<DataBuffer, UInt32>>.write(input: "toto", to: &data);
        let str1 = try VarcharString<DataBuffer, ValueBE<DataBuffer, UInt32>>.from(slice: data).data;
        XCTAssertEqual(str1, "toto");
    }

    func testOptionals() throws {
        var data = DataBuffer();
        try Optional<DataBuffer, VarcharString<DataBuffer, ValueBE<DataBuffer, UInt32>>>.write(input: "toto", to: &data);
        let str = try Optional<DataBuffer, VarcharString<DataBuffer, ValueBE<DataBuffer, UInt32>>>.from(slice: data).data;
        XCTAssertEqual(str, "toto");
        data.clear();
        try Optional<DataBuffer, VarcharString<DataBuffer, ValueBE<DataBuffer, UInt32>>>.write(input: "test", to: &data);
        let str1 = try Optional<DataBuffer, VarcharString<DataBuffer, ValueBE<DataBuffer, UInt32>>>.from(slice: data).data;
        XCTAssertEqual(str1, "test");
        data.clear();
        try Optional<DataBuffer, VarcharString<DataBuffer, ValueBE<DataBuffer, UInt32>>>.write(input: nil, to: &data);
        let str2 = try Optional<DataBuffer, VarcharString<DataBuffer, ValueBE<DataBuffer, UInt32>>>.from(slice: data).data;
        XCTAssertNil(str2);
    }

    func testBitCodecLE() throws {
        let buffer = DataBuffer([0xFF, 0xFF, 0xFF, 0xFF]);
        XCTAssertEqual(BitCodecLE.read(UInt32.self, buffer[...4], bitOffset: 0, bitSize: 32), 0xFFFFFFFF);
        XCTAssertEqual(BitCodecLE.read(UInt8.self, buffer[0...1], bitOffset: 0, bitSize: 1), 1);
        XCTAssertEqual(BitCodecLE.read(UInt8.self, buffer[0...1], bitOffset: 0, bitSize: 4), 0xF);
        XCTAssertEqual(BitCodecLE.read(UInt8.self, buffer[0...1], bitOffset: 4, bitSize: 4), 0xF);
    }

    func testBitCodecBE() throws {
        let buffer = DataBuffer([0xAB, 0xF0]);
        XCTAssertEqual(BitCodecBE.read(UInt16.self, buffer[0...2], bitOffset: 0, bitSize: 12), 0xABF);
        let buffer1 = DataBuffer([0x0, 0x0]);
        var b = buffer1[...2]
        BitCodecBE.write(UInt16.self, &b, bitOffset: 0, bitSize: 12, value: 0xABF);
        XCTAssertEqual(BitCodecBE.read(UInt16.self, buffer1[0...2], bitOffset: 0, bitSize: 12), 0xABF);
    }

    func testArrayCodec() throws {
        let buffer = DataBuffer(size: UInt32.size * 4);
        let codec = ArrayCodec<DataBuffer, ByteCodecBE, UInt32>(buffer: buffer, itemBitSize: 32);
        XCTAssertEqual(codec.count, 4);
        codec.set(0, 0xAB).set(1, 0xCD).set(2, 0xEF).set(3, 0x12);
        XCTAssertEqual(codec[0], 0xAB);
        XCTAssertEqual(codec[1], 0xCD);
        XCTAssertEqual(codec[2], 0xEF);
        XCTAssertEqual(codec[3], 0x12);
        codec.set(3, 0x42424242);
        XCTAssertEqual(codec[3], 0x42424242);
    }
}
