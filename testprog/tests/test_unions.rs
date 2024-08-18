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

use bp3d_proto::message::{FromSlice, WriteTo};
use std::io::Write;
use testprog::enums::{Header, Type};
use testprog::unions::{Item, Value};
use testprog::values::{
    ValueDouble, ValueFloat, ValueInt16, ValueInt32, ValueInt64, ValueInt8, ValueString, ValueUInt16, ValueUInt32,
    ValueUInt64, ValueUInt8,
};

fn write_message(value: Value, out: &mut impl Write) {
    let mut header = Header::new_on_stack();
    value.set_discriminant(&mut header);
    let item = Item {
        header: header.to_ref(),
        name: "test",
        value,
    };
    Item::write_to(&item, out).unwrap();
}

fn read_message(slice: &[u8], ty: Type) -> Value {
    let msg = Item::from_slice(slice).unwrap();
    assert_eq!(slice.len(), msg.size());
    let item = msg.into_inner();
    assert_eq!(item.header.get_type().unwrap(), ty);
    assert_eq!(item.name, "test");
    item.value
}

#[test]
fn item_numbers() {
    let mut buf = Vec::with_capacity(256);
    let mut value_buffer: [u8; 8] = [0; 8];

    write_message(
        Value::Int8(ValueInt8::from(&mut value_buffer).set_data(-42).to_ref()),
        &mut buf,
    );
    assert_eq!(read_message(&buf, Type::Int8).as_int8().unwrap().get_data(), -42);

    buf.clear();
    write_message(
        Value::Int16(ValueInt16::from(&mut value_buffer).set_data(-4242).to_ref()),
        &mut buf,
    );
    assert_eq!(read_message(&buf, Type::Int16).as_int16().unwrap().get_data(), -4242);

    buf.clear();
    write_message(
        Value::Int32(ValueInt32::from(&mut value_buffer).set_data(-424242).to_ref()),
        &mut buf,
    );
    assert_eq!(read_message(&buf, Type::Int32).as_int32().unwrap().get_data(), -424242);

    buf.clear();
    write_message(
        Value::Int64(ValueInt64::from(&mut value_buffer).set_data(-42424242).to_ref()),
        &mut buf,
    );
    assert_eq!(
        read_message(&buf, Type::Int64).as_int64().unwrap().get_data(),
        -42424242
    );

    buf.clear();
    write_message(
        Value::UInt8(ValueUInt8::from(&mut value_buffer).set_data(42).to_ref()),
        &mut buf,
    );
    assert_eq!(read_message(&buf, Type::UInt8).as_u_int8().unwrap().get_data(), 42);

    buf.clear();
    write_message(
        Value::UInt16(ValueUInt16::from(&mut value_buffer).set_data(4242).to_ref()),
        &mut buf,
    );
    assert_eq!(read_message(&buf, Type::UInt16).as_u_int16().unwrap().get_data(), 4242);

    buf.clear();
    write_message(
        Value::UInt32(ValueUInt32::from(&mut value_buffer).set_data(424242).to_ref()),
        &mut buf,
    );
    assert_eq!(
        read_message(&buf, Type::UInt32).as_u_int32().unwrap().get_data(),
        424242
    );

    buf.clear();
    write_message(
        Value::UInt64(ValueUInt64::from(&mut value_buffer).set_data(42424242).to_ref()),
        &mut buf,
    );
    assert_eq!(
        read_message(&buf, Type::UInt64).as_u_int64().unwrap().get_data(),
        42424242
    );
}

#[test]
fn item_null() {
    let mut buf = Vec::with_capacity(256);

    write_message(Value::Null, &mut buf);
    assert!(read_message(&buf, Type::Null).is_null());
}

#[test]
fn item_float() {
    let mut buf = Vec::with_capacity(256);
    let mut value_buffer: [u8; 8] = [0; 8];

    write_message(
        Value::Float(ValueFloat::from(&mut value_buffer).set_data(42.42).to_ref()),
        &mut buf,
    );
    assert_eq!(read_message(&buf, Type::Float).as_float().unwrap().get_data(), 42.42);

    buf.clear();
    write_message(
        Value::Double(ValueDouble::from(&mut value_buffer).set_data(42.4242).to_ref()),
        &mut buf,
    );
    assert_eq!(
        read_message(&buf, Type::Double).as_double().unwrap().get_data(),
        42.4242
    );
}

#[test]
fn item_string() {
    let mut buf = Vec::with_capacity(256);

    write_message(Value::String(ValueString { data: "this is a test" }), &mut buf);
    assert_eq!(
        read_message(&buf, Type::String).as_string().unwrap().data,
        "this is a test"
    );
}
