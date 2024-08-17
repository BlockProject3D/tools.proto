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
import testprog

final class UnionsTests: XCTestCase {

    override func setUpWithError() throws {
    }

    override func tearDownWithError() throws {
    }

    func writeMessage<B: WritableBuffer>(_ value: UnionsValue<DataBuffer>, out: inout B) throws where B: Buffer {
        let header = EnumsHeader();
        value.setDiscriminant(header);
        let item = UnionsItem(
            header: header,
            name: "test",
            value: value
        );
        try UnionsItem.write(input: item, to: &out);
    }

    func readMessage<B: Buffer>(_ slice: B, type: EnumsType) throws -> UnionsValue<B> {
        let msg = try UnionsItem.from(slice: slice);
        XCTAssertEqual(slice.size, msg.size);
        let item = msg.data;
        XCTAssertEqual(item.header.type, type);
        XCTAssertEqual(item.name, "test");
        return item.value;
    }

    func testNumbers() throws {
        let value_buffer = DataBuffer(size: 8);
        var out = DataBuffer();

        try writeMessage(UnionsValue.int8(ValuesValueInt8(value_buffer).setData(-42)), out: &out);
        XCTAssertEqual(try readMessage(out, type: EnumsType.int8).asInt8()!.data, -42);
    }

}
