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

use bp3d_proto::message::{FromBytes, WriteTo};
use bp3d_proto::util::Wrap;
use testprog::enums::{Header, Type};
use testprog::lists::{Dataset, DatasetRuns, SpanRun, SpanRunVars, Times, SIZE_TIMES};
use testprog::unions::{Item, Value};
use testprog::values::{ValueInt16, ValueString, SIZE_VALUE_U_INT64};

fn write_span_run<F: FnOnce(SpanRun) -> bp3d_proto::message::Result<()>>(f: F) {
    let data: Vec<u8> = Vec::new();
    let mut value: [u8; SIZE_VALUE_U_INT64] = [0; SIZE_VALUE_U_INT64];
    let mut header = Header::new();
    let mut times = Times::new();
    times.set_start(42424242).set_end(42424242);
    let mut list = SpanRunVars::new(data);
    list.write_item(&Item {
        header: header.set_type(Type::String).to_ref(),
        name: "test",
        value: Value::String(ValueString { data: "this is a test" }),
    })
    .unwrap();
    list.write_item(&Item {
        header: header.set_type(Type::Int16).to_ref(),
        name: "test1",
        value: Value::Int16(ValueInt16::wrap(&mut value).set_data(-4242).to_ref()),
    })
    .unwrap();
    let msg = SpanRun {
        times: times.to_ref(),
        vars: list.to_ref(),
    };
    f(msg).unwrap()
}

fn assert_span_run(msg: SpanRun) {
    assert_eq!(msg.vars.len(), 2);
    assert_eq!(msg.times.get_start(), 42424242);
    assert_eq!(msg.times.get_end(), 42424242);
    let mut vars = msg.vars.iter();
    let var = vars.next().unwrap().unwrap();
    assert_eq!(var.name, "test");
    assert_eq!(var.value.as_string().unwrap().data, "this is a test");
    let var = vars.next().unwrap().unwrap();
    assert_eq!(var.name, "test1");
    assert_eq!(var.value.as_int16().unwrap().get_data(), -4242);
}

#[test]
fn run() {
    assert_eq!(size_of::<Times<[u8; SIZE_TIMES]>>(), SIZE_TIMES);
    let mut msg_buffer: Vec<u8> = Vec::new();
    {
        write_span_run(|msg| SpanRun::write_to(&msg, &mut msg_buffer));
    }
    {
        let msg = SpanRun::from_bytes(&msg_buffer).unwrap();
        assert_eq!(msg_buffer.len(), msg.size());
        assert_span_run(msg.into_inner());
    }
}

#[test]
fn dataset() {
    let mut msg_buffer: Vec<u8> = Vec::new();
    {
        let data: Vec<u8> = Vec::new();
        let mut list = DatasetRuns::new(data);
        write_span_run(|msg| list.write_item(&msg));
        write_span_run(|msg| list.write_item(&msg));
        write_span_run(|msg| list.write_item(&msg));
        let msg = Dataset { runs: list.to_ref() };
        Dataset::write_to(&msg, &mut msg_buffer).unwrap();
    }
    {
        let msg = Dataset::from_bytes(&msg_buffer).unwrap().into_inner();
        assert_eq!(msg.runs.len(), 3);
        let mut runs = msg.runs.iter();
        assert_span_run(runs.next().unwrap().unwrap());
        assert_span_run(runs.next().unwrap().unwrap());
        assert_span_run(runs.next().unwrap().unwrap());
    }
}
