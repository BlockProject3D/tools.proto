//
//  BP3DProtoTests.swift
//  BP3DProtoTests
//
//  Created by Yuri Edward on 8/9/24.
//

import XCTest
import BP3DProto

final class BP3DProtoTests: XCTestCase {
    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testValueLE() throws {
        let data = DataBuffer(bytes: [0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x67, 0x89]);
        let value8 = try ValueLE<UInt8>.from(slice: data).data;
        let value16 = try ValueLE<UInt16>.from(slice: data).data;
        let value32 = try ValueLE<UInt32>.from(slice: data).data;
        let value64 = try ValueLE<UInt64>.from(slice: data).data;
        XCTAssertEqual(value8, 0xAB);
        XCTAssertEqual(value16, 0xCDAB);
        XCTAssertEqual(value32, 0x12EFCDAB);
        XCTAssertEqual(value64, 0x8967563412EFCDAB);
    }

    func testValueBE() throws {
        let data = DataBuffer(bytes: [0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x67, 0x89]);
        let value8 = try ValueBE<UInt8>.from(slice: data).data;
        let value16 = try ValueBE<UInt16>.from(slice: data).data;
        let value32 = try ValueBE<UInt32>.from(slice: data).data;
        let value64 = try ValueBE<UInt64>.from(slice: data).data;
        XCTAssertEqual(value8, 0xAB);
        XCTAssertEqual(value16, 0xABCD);
        XCTAssertEqual(value32, 0xABCDEF12);
        XCTAssertEqual(value64, 0xABCDEF1234566789);
    }

    func testValueLEWrite() throws {
        var data = DataBuffer();
        try ValueLE<UInt64>.write(input: 0xABCDEF1234566789, to: &data);
        XCTAssertEqual(data.toData(), Data([0x89, 0x67, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB]));
        let value64 = try ValueLE<UInt64>.from(slice: data).data;
        XCTAssertEqual(value64, 0xABCDEF1234566789);
    }

    func testValueBEWrite() throws {
        var data = DataBuffer();
        try ValueBE<UInt64>.write(input: 0xABCDEF1234566789, to: &data);
        XCTAssertEqual(data.toData(), Data([0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x67, 0x89]));
        let value64 = try ValueBE<UInt64>.from(slice: data).data;
        XCTAssertEqual(value64, 0xABCDEF1234566789);
    }

    func testNullTerminatedString() throws {
        var data = DataBuffer();
        try NullTerminatedString.write(input: "test", to: &data);
        let str = try NullTerminatedString.from(slice: data).data;
        XCTAssertEqual(str, "test");
        data.clear();
        try NullTerminatedString.write(input: "toto", to: &data);
        let str1 = try NullTerminatedString.from(slice: data).data;
        XCTAssertEqual(str1, "toto");
    }

    func testVarcharString() throws {
        var data = DataBuffer();
        try VarcharString<ValueLE<UInt8>>.write(input: "test", to: &data);
        let str = try VarcharString<ValueLE<UInt8>>.from(slice: data).data;
        XCTAssertEqual(str, "test");
        data.clear();
        try VarcharString<ValueBE<UInt32>>.write(input: "toto", to: &data);
        let str1 = try VarcharString<ValueBE<UInt32>>.from(slice: data).data;
        XCTAssertEqual(str1, "toto");
    }

    func testOptionals() throws {
        var data = DataBuffer();
        try Optional<VarcharString<ValueBE<UInt32>>>.write(input: "toto", to: &data);
        let str = try Optional<VarcharString<ValueBE<UInt32>>>.from(slice: data).data;
        XCTAssertEqual(str, "toto");
        data.clear();
        try Optional<VarcharString<ValueBE<UInt32>>>.write(input: "test", to: &data);
        let str1 = try Optional<VarcharString<ValueBE<UInt32>>>.from(slice: data).data;
        XCTAssertEqual(str1, "test");
        data.clear();
        try Optional<VarcharString<ValueBE<UInt32>>>.write(input: nil, to: &data);
        let str2 = try Optional<VarcharString<ValueBE<UInt32>>>.from(slice: data).data;
        XCTAssertNil(str2);
    }
}
