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

final class ListsTests: XCTestCase {

    override func setUpWithError() throws {
    }

    override func tearDownWithError() throws {
    }

    func writeSpanRun(_ f: (ListsSpanRun<DataBuffer>) throws -> ()) throws {
        let header = EnumsHeader();
        let times = ListsTimes();
        times.setStart(42424242).setEnd(42424242);
        var list = ListsSpanRunVars(DataBuffer());
        try list.writeItem(UnionsItem(
            header: header.setType(EnumsType.string),
            name: "test",
            value: UnionsValue.string(ValuesValueString(data: "this is a test"))
        ));
        try list.writeItem(UnionsItem(
            header: header.setType(EnumsType.int16),
            name: "test1",
            value: UnionsValue.int16(ValuesValueInt16().setData(-4242))
        ));
        let msg = ListsSpanRun(times: times, vars: list);
        try f(msg);
    }

    func assertSpanRun<B: Buffer>(_ msg: ListsSpanRun<B>) throws {
        XCTAssertEqual(msg.vars.count, 2);
        XCTAssertEqual(msg.times.start, 42424242);
        XCTAssertEqual(msg.times.end, 42424242);
        let vars = try msg.vars.toArray();
        XCTAssertEqual(vars[0].name, "test");
        XCTAssertEqual(vars[0].value.asString()?.data, "this is a test");
        XCTAssertEqual(vars[1].name, "test1");
        XCTAssertEqual(vars[1].value.asInt16()?.data, -4242);
    }

    func testRun() throws {
        var buffer = DataBuffer();
        try writeSpanRun({ msg in try ListsSpanRun.write(input: msg, to: &buffer) });
        let msg = try ListsSpanRun.from(slice: buffer);
        XCTAssertEqual(msg.size, buffer.size);
        try assertSpanRun(msg.data);
    }

}
