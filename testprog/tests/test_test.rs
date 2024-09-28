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

use bp3d_proto::message::{FromBytes, FromBytesWithOffsets, WriteTo};
use std::io::Write;
use testprog::test::{Test, Test1};

fn write_message<W: Write>(out: W) {
    let msg = Test {
        p1: Some(Test1 {
            s1: "this is a test",
            p1: 42,
        }),
        s1: "a test",
        s2: Some("hello world"),
    };
    Test::write_to(&msg, out).unwrap();
}

#[test]
fn test() {
    let mut v = Vec::new();
    write_message(&mut v);
    let msg = Test::from_bytes(&v).unwrap().into_inner();
    assert_eq!(msg.p1.unwrap().p1, 42);
    assert_eq!(msg.p1.unwrap().s1, "this is a test");
    assert_eq!(msg.s1, "a test");
    assert_eq!(msg.s2, Some("hello world"));
    println!("{:?}", msg);
}

#[test]
fn test_offsets() {
    let mut v = Vec::new();
    write_message(&mut v);
    let (msg, offsets) = Test::from_bytes_with_offsets(&v).unwrap().into_inner();
    assert_eq!(msg.p1.unwrap().p1, 42);
    assert_eq!(msg.p1.unwrap().s1, "this is a test");
    assert_eq!(msg.s1, "a test");
    assert_eq!(msg.s2, Some("hello world"));
    assert_eq!(offsets.s1.start, 0);
    assert_eq!(offsets.s1.size(), 7);
    assert_eq!(offsets.s2.start, 7);
    assert_eq!(offsets.s2.size(), 13);
    assert_eq!(offsets.p1.start, 20);
    assert_eq!(offsets.p1.size(), 20);
    assert_eq!(offsets.p1_offsets.unwrap().s1.start, 0);
    assert_eq!(offsets.p1_offsets.unwrap().s1.size(), 15);
    assert_eq!(offsets.p1_offsets.unwrap().p1.start, 15);
    assert_eq!(offsets.p1_offsets.unwrap().p1.size(), 4);
}
