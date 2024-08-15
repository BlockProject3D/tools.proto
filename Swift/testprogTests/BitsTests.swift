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

final class BitsTests: XCTestCase {
    override func setUpWithError() throws {
    }
    
    override func tearDownWithError() throws {
    }
    
    func testNumbers() throws {
        let nums = BitsNumbers();
        XCTAssertEqual(SIZE_BITS_NUMBERS, 4);
        nums.setA(-8).setB(15).setC(-65536).setD(127);
        XCTAssertEqual(nums.a, -8);
        XCTAssertEqual(nums.b, 15);
        XCTAssertEqual(nums.c, -65536);
        XCTAssertEqual(nums.d, 127);
        nums.setC(65535);
        XCTAssertEqual(nums.c, 65535);
        nums.setA(-7);
        XCTAssertEqual(nums.a, -7);
        XCTAssertEqual(nums.rawA, 9);
        nums.setA(-6);
        XCTAssertEqual(nums.a, -6);
        XCTAssertEqual(nums.rawA, 10);
        nums.setA(-5);
        XCTAssertEqual(nums.a, -5);
        XCTAssertEqual(nums.rawA, 11);
        nums.setA(1);
        XCTAssertEqual(nums.a, 1);
        XCTAssertEqual(nums.rawA, 1);
        nums.setA(4);
        XCTAssertEqual(nums.a, 4);
        XCTAssertEqual(nums.rawA, 4);
    }
    
    func testNumbersRaw() throws {
        let nums = BitsNumbers();
        nums.setRawA(15);
        nums.setRawB(15);
        nums.setRawC(65536);
        nums.setRawD(127);
        nums.setRawC(65536);
        XCTAssertEqual(nums.rawA, 15);
        XCTAssertEqual(nums.a, -1);
        XCTAssertEqual(nums.rawB, 15);
        XCTAssertEqual(nums.rawC, 65536);
        XCTAssertEqual(nums.rawD, 127);
        var cur_a = 0;
        for i in 0...15 {
            nums.setRawA(UInt8(i));
            if i > 7 && cur_a > 0 {
                cur_a = -8
            }
            XCTAssertEqual(nums.a, Int8(cur_a));
            cur_a += 1;
        }
    }
    
    func testNumbers2() throws {
        let nums = Bits2Numbers();
        XCTAssertEqual(SIZE_BITS2_NUMBERS, 4);
        nums.setA(-8).setB(15).setC(-65536).setD(127);
        XCTAssertEqual(nums.a, -8);
        XCTAssertEqual(nums.b, 15);
        XCTAssertEqual(nums.c, -65536);
        XCTAssertEqual(nums.d, 127);
        nums.setC(65535);
        XCTAssertEqual(nums.c, 65535);
        nums.setA(-7);
        XCTAssertEqual(nums.a, -7);
        XCTAssertEqual(nums.rawA, 9);
        nums.setA(-6);
        XCTAssertEqual(nums.a, -6);
        XCTAssertEqual(nums.rawA, 10);
        nums.setA(-5);
        XCTAssertEqual(nums.a, -5);
        XCTAssertEqual(nums.rawA, 11);
        nums.setA(1);
        XCTAssertEqual(nums.a, 1);
        XCTAssertEqual(nums.rawA, 1);
        nums.setA(4);
        XCTAssertEqual(nums.a, 4);
        XCTAssertEqual(nums.rawA, 4);
    }

    func testNumbersRaw2() throws {
        let nums = Bits2Numbers();
        nums.setRawA(15);
        nums.setRawB(15);
        nums.setRawC(65536);
        nums.setRawD(127);
        nums.setRawC(65536);
        XCTAssertEqual(nums.rawA, 15);
        XCTAssertEqual(nums.a, -1);
        XCTAssertEqual(nums.rawB, 15);
        XCTAssertEqual(nums.rawC, 65536);
        XCTAssertEqual(nums.rawD, 127);
        var cur_a = 0;
        for i in 0...15 {
            nums.setRawA(UInt8(i));
            if i > 7 && cur_a > 0 {
                cur_a = -8
            }
            XCTAssertEqual(nums.a, Int8(cur_a));
            cur_a += 1;
        }
    }
}
