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
use testprog::arrays::{Msg, Msg1, msg, msg1};

#[test]
fn msg() {
    let mut msg_buffer = Vec::new();
    {
        let mut buffer: [u8; 3 * 4] = [0; 3 * 4];
        let mut arr = msg::ItemsType::from_parts_checked(&mut buffer, 4).unwrap();
        arr.get_mut::<msg::ItemsMut>(0).set_id(3).set_count(1024).set_slot(10);
        arr.get_mut::<msg::ItemsMut>(1).set_id(2).set_count(1023).set_slot(9);
        arr.get_mut::<msg::ItemsMut>(2).set_id(1).set_count(16).set_slot(8);
        arr.get_mut::<msg::ItemsMut>(3).set_id(0).set_count(4).set_slot(7);
        let msg = Msg {
            items: arr.as_ref()
        };
        Msg::write_to(&msg, &mut msg_buffer).unwrap();
    }
    {
        let msg = Msg::from_slice(&msg_buffer).unwrap().into_inner();
        assert_eq!(msg.items.len(), 4);
        assert_eq!(msg.items.get::<msg::ItemsRef>(0).get_id(), 3);
        assert_eq!(msg.items.get::<msg::ItemsRef>(1).get_id(), 2);
        assert_eq!(msg.items.get::<msg::ItemsRef>(2).get_id(), 1);
        assert_eq!(msg.items.get::<msg::ItemsRef>(3).get_id(), 0);
        assert_eq!(msg.items.get::<msg::ItemsRef>(0).get_count(), 1024);
        assert_eq!(msg.items.get::<msg::ItemsRef>(1).get_count(), 1023);
        assert_eq!(msg.items.get::<msg::ItemsRef>(2).get_count(), 16);
        assert_eq!(msg.items.get::<msg::ItemsRef>(3).get_count(), 4);
        assert_eq!(msg.items.get::<msg::ItemsRef>(0).get_slot(), 10);
        assert_eq!(msg.items.get::<msg::ItemsRef>(1).get_slot(), 9);
        assert_eq!(msg.items.get::<msg::ItemsRef>(2).get_slot(), 8);
        assert_eq!(msg.items.get::<msg::ItemsRef>(3).get_slot(), 7);
    }
}

#[test]
fn msg1_1() {
    let mut msg_buffer = Vec::new();
    {
        let mut buffer: [u8; 3 * 4] = [0; 3 * 4];
        let mut arr = msg1::ItemsType::from_parts_checked(&mut buffer, 4).unwrap();
        arr.get_mut::<msg1::ItemsMut>(0).set_id(3).set_count(1024).set_slot(10);
        arr.get_mut::<msg1::ItemsMut>(1).set_id(2).set_count(1023).set_slot(9);
        arr.get_mut::<msg1::ItemsMut>(2).set_id(1).set_count(16).set_slot(8);
        arr.get_mut::<msg1::ItemsMut>(3).set_id(0).set_count(4).set_slot(7);
        let msg = Msg1 {
            items: Some(arr.as_ref())
        };
        Msg1::write_to(&msg, &mut msg_buffer).unwrap();
    }
    {
        let msg = Msg1::from_slice(&msg_buffer).unwrap().into_inner();
        assert!(msg.items.is_some());
        let items = msg.items.unwrap();
        assert_eq!(items.len(), 4);
        assert_eq!(items.get::<msg::ItemsRef>(0).get_id(), 3);
        assert_eq!(items.get::<msg::ItemsRef>(1).get_id(), 2);
        assert_eq!(items.get::<msg::ItemsRef>(2).get_id(), 1);
        assert_eq!(items.get::<msg::ItemsRef>(3).get_id(), 0);
        assert_eq!(items.get::<msg::ItemsRef>(0).get_count(), 1024);
        assert_eq!(items.get::<msg::ItemsRef>(1).get_count(), 1023);
        assert_eq!(items.get::<msg::ItemsRef>(2).get_count(), 16);
        assert_eq!(items.get::<msg::ItemsRef>(3).get_count(), 4);
        assert_eq!(items.get::<msg::ItemsRef>(0).get_slot(), 10);
        assert_eq!(items.get::<msg::ItemsRef>(1).get_slot(), 9);
        assert_eq!(items.get::<msg::ItemsRef>(2).get_slot(), 8);
        assert_eq!(items.get::<msg::ItemsRef>(3).get_slot(), 7);
    }
}

#[test]
fn msg1_2() {
    let mut msg_buffer = Vec::new();
    {
        let msg = Msg1 {
            items: None
        };
        Msg1::write_to(&msg, &mut msg_buffer).unwrap();
    }
    {
        let msg = Msg1::from_slice(&msg_buffer).unwrap().into_inner();
        assert!(msg.items.is_none());
    }
}