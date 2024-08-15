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

final class StructArraysTests: XCTestCase {

    override func setUpWithError() throws {
    }

    override func tearDownWithError() throws {
    }

    func testBasic() throws {
        let basic = StructArraysBasic();
        XCTAssertEqual(SIZE_STRUCT_ARRAYS_BASIC, 58);
        basic.p3.set(0, 42.42).set(1, 42.42).set(2, 42.42).set(3, 42.42);
        XCTAssertEqual(basic.p3[0], 42.42);
        XCTAssertEqual(basic.p3[1], 42.42);
        XCTAssertEqual(basic.p3[2], 42.42);
        XCTAssertEqual(basic.p3[3], 42.42);
        basic.setP1(424242);
        XCTAssertEqual(basic.p1, 424242);
        basic.p2.fromData(Data("this is a test".utf8));
        XCTAssertEqual(basic.p2.toData()[...13], Data("this is a test".utf8));
        basic.p4.set(0, 0xABCDEF).set(1, 0xABCDEF);
        XCTAssertEqual(basic.p4[0], 0xABCDEF);
        XCTAssertEqual(basic.p4[1], 0xABCDEF);
    }

}
