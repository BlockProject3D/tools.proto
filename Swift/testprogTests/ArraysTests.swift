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

final class ArraysTests: XCTestCase {

    override func setUpWithError() throws {
    }

    override func tearDownWithError() throws {
    }

    func testMsg() throws {
        var msg_buffer = DataBuffer();
        do {
            let buffer = DataBuffer(size: 12);
            let arr = ArraysMsgItemsType(buffer);
            arr[0].setId(3).setCount(1024).setSlot(10);
            arr[1].setId(2).setCount(1023).setSlot(9);
            arr[2].setId(1).setCount(16).setSlot(8);
            arr[3].setId(0).setCount(4).setSlot(7);
            var slot = 0;
            for i in 0...arr.count - 1 {
                arr[i].setSlot(UInt8(slot));
                slot += 1;
            }
            let msg = ArraysMsg(items: arr);
            try ArraysMsg.write(input: msg, to: &msg_buffer);
        }
        do {
            let msg = try ArraysMsg.from(slice: msg_buffer);
            XCTAssertEqual(msg_buffer.size, msg.size);
            let msg1 = msg.data;
            XCTAssertEqual(msg1.items.count, 4);
            XCTAssertEqual(msg1.items[0].id, 3);
            XCTAssertEqual(msg1.items[1].id, 2);
            XCTAssertEqual(msg1.items[2].id, 1);
            XCTAssertEqual(msg1.items[3].id, 0);
            XCTAssertEqual(msg1.items[0].count, 1024);
            XCTAssertEqual(msg1.items[1].count, 1023);
            XCTAssertEqual(msg1.items[2].count, 16);
            XCTAssertEqual(msg1.items[3].count, 4);
            XCTAssertEqual(msg1.items[0].slot, 0);
            XCTAssertEqual(msg1.items[1].slot, 1);
            XCTAssertEqual(msg1.items[2].slot, 2);
            XCTAssertEqual(msg1.items[3].slot, 3);
        }
    }

}
