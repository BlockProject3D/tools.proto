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

use bp3d_proto::message::ShapeAndWrite;
use bp3d_proto::message::{FromBytes, WriteSelf};
use testprog::codec::ContainerHeader;
use testprog::custom_codec::{Header, Test, Test2};

#[test]
fn test_basic() {
    let mut buf = Vec::with_capacity(1024);
    {
        let msg = Test {
            non_optional: b"this is a test",
            optional: None,
        };
        msg.write_self(&mut buf).unwrap();
    }
    {
        let msg = Test::from_bytes(&buf).unwrap().into_inner();
        assert_eq!(msg.non_optional, b"this is a test");
        assert_eq!(msg.optional, None);
    }
}

#[test]
fn test_basic2() {
    let mut buf = Vec::with_capacity(1024);
    {
        let msg = Test {
            non_optional: b"this is a test",
            optional: Some(b"this is a test"),
        };
        msg.write_self(&mut buf).unwrap();
    }
    {
        let msg = Test::from_bytes(&buf).unwrap().into_inner();
        assert_eq!(msg.non_optional, b"this is a test");
        assert_eq!(msg.optional, Some(b"this is a test".as_ref()));
    }
}

#[test]
fn test_headers() {
    let mut buf = Vec::with_capacity(1024);
    {
        let mut hdr = Header::new();
        hdr.set_size(14);
        let msg = Test2 {
            hdr: hdr.to_ref(),
            data: ContainerHeader::new(b"this is a test"),
            data2: ContainerHeader::new(b"this is a test"),
        };
        assert_eq!(msg.size().unwrap(), 29);
        msg.write_self(&mut buf).unwrap();
    }
    {
        let msg = Test2::from_bytes(&buf).unwrap();
        assert_eq!(msg.size(), 29);
        let msg = msg.into_inner();
        assert_eq!(msg.hdr.get_size(), 14);
        assert_eq!(msg.data, ContainerHeader::new(b"this is a test"));
        assert_eq!(msg.data2, ContainerHeader::new(b"this is a test"));
    }
}

#[test]
fn test_headers2() {
    let mut buf = Vec::with_capacity(1024);
    {
        let mut hdr = Header::new();
        hdr.set_size(13);
        let msg = Test2 {
            hdr: hdr.to_ref(),
            data: ContainerHeader::new(b"this is a test"),
            data2: ContainerHeader::new(b"this is a test"),
        };
        assert_eq!(msg.size().unwrap(), 27);
        msg.write_self(&mut buf).unwrap();
    }
    {
        let msg = Test2::from_bytes(&buf).unwrap();
        assert_eq!(msg.size(), 27);
        let msg = msg.into_inner();
        assert_eq!(msg.hdr.get_size(), 13);
        assert_eq!(msg.data, ContainerHeader::new(b"this is a tes"));
        assert_eq!(msg.data2, ContainerHeader::new(b"this is a tes"));
    }
}

#[test]
fn test_headers_shape_write() {
    let mut buf = Vec::with_capacity(1024);
    {
        Test2 {
            hdr: Header::new().to_ref(),
            data: ContainerHeader::new(b"test"),
            data2: ContainerHeader::new(b"test"),
        }
        .shape_and_write(&mut buf)
        .unwrap();
    }
    {
        let msg = Test2::from_bytes(&buf).unwrap();
        assert_eq!(msg.size(), 9);
        let msg = msg.into_inner();
        assert_eq!(msg.hdr.get_size(), 4);
        assert_eq!(msg.data, ContainerHeader::new(b"test"));
        assert_eq!(msg.data2, ContainerHeader::new(b"test"));
    }
}
