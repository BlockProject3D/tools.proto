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
import testprog
import BP3DProto

final class StructsTests: XCTestCase {
    
    override func setUpWithError() throws {
    }
    
    override func tearDownWithError() throws {
    }
    
    func checkNumbers<B: Buffer>(_ nums: StructsNumbers<B>) {
        XCTAssertEqual(nums.uA, 0x123456AB);
        XCTAssertEqual(nums.a, -424242);
        XCTAssertEqual(nums.uB, 0x1234);
        XCTAssertEqual(nums.b, -4242);
        XCTAssertEqual(nums.uC, 0x12);
        XCTAssertEqual(nums.c, -42);
    }
    
    func checkFlags<B: Buffer>(_ flags: StructsFlags<B>) {
        XCTAssert(flags.a);
        XCTAssert(flags.b);
        XCTAssert(flags.c);
        XCTAssert(flags.d);
    }
    
    func checkFloats<B: Buffer>(_ floats: StructsFloats<B>) {
        XCTAssertEqual(floats.a, 4242.0);
        XCTAssertEqual(floats.b, 4242.4242);
    }
    
    func testNumbers() throws {
        let nums = StructsNumbers();
        XCTAssertEqual(SIZE_STRUCTS_NUMBERS, 14);
        nums.setUA(0x123456AB)
            .setA(-424242)
            .setUB(0x1234)
            .setB(-4242)
            .setUC(0x12)
            .setC(-42);
        checkNumbers(nums);
    }
    
    func testFlags() throws {
        let flags = StructsFlags();
        XCTAssertEqual(SIZE_STRUCTS_FLAGS, 15);
        flags.setA(true).setB(true).setC(true).setD(true);
        checkFlags(flags);
    }
    
    func testFloats() throws {
        let floats = StructsFloats();
        XCTAssertEqual(SIZE_STRUCTS_FLOATS, 12);
        floats.setA(4242.0).setB(4242.4242);
        checkFloats(floats);
    }

    func testMaster() throws {
        let master = StructsMaster();
        XCTAssertEqual(SIZE_STRUCTS_MASTER, 14 + 15 + 12);
        master.nums
            .setUA(0x123456AB)
            .setA(-424242)
            .setUB(0x1234)
            .setB(-4242)
            .setUC(0x12)
            .setC(-42);
        master.flags.setA(true).setB(true).setC(true).setD(true);
        master.floats.setA(4242.0).setB(4242.4242);
        checkNumbers(master.nums);
        checkFlags(master.flags);
        checkFloats(master.floats);
    }
}
