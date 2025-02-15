// Copyright (c) 2025, BlockProject 3D
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

use bp3d_proto_dynamic::field::primitive::Value;
use bp3d_proto_dynamic::proto::{Proto, StructureExt};
use bp3d_protoc::api::core::loader::{Loader, Options};

#[test]
fn test_basic() {
    let mut loader = Loader::new(16);
    loader.load_from_folder("../testprog/src", &Options::from_package("testprog")).unwrap();
    loader.exclude("custom_codec_broken");
    let proto = Proto::build(loader).unwrap();
    let mut view = proto.get_structure("bits.Numbers").unwrap().new_instance();
    view.read_copy(b"abcd").unwrap();
    view.copy_from(b"1234");
    println!("{:?}", view["a"].as_bytes());
    println!("{:?}", view["b"].as_bytes());
    println!("{:?}", view["c"].as_bytes());
    println!("{:?}", view["d"].as_bytes());
    let value_a = bp3d_proto_dynamic::field::primitive::from_field(proto.get_structure("bits.Numbers").unwrap().get_field("a").unwrap()).unwrap();
    let value_b = bp3d_proto_dynamic::field::primitive::from_field(proto.get_structure("bits.Numbers").unwrap().get_field("b").unwrap()).unwrap();
    let value_c = bp3d_proto_dynamic::field::primitive::from_field(proto.get_structure("bits.Numbers").unwrap().get_field("c").unwrap()).unwrap();
    let value_d = bp3d_proto_dynamic::field::primitive::from_field(proto.get_structure("bits.Numbers").unwrap().get_field("d").unwrap()).unwrap();

    value_a.set(&mut view["a"], Value::Signed(-8));
    value_b.set(&mut view["b"], Value::Unsigned(15));
    value_c.set(&mut view["c"], Value::Signed(-65536));
    value_d.set(&mut view["d"], Value::Unsigned(127));

    assert_eq!(value_a.get(&view["a"]).to_signed(), -8);
    assert_eq!(value_b.get(&view["b"]).to_unsigned(), 15);
    assert_eq!(value_c.get(&view["c"]).to_signed(), -65536);
    assert_eq!(value_d.get(&view["d"]).to_unsigned(), 127);

    value_c.set(&mut view["c"], Value::Signed(65535));
    assert_eq!(value_c.get(&view["c"]).to_signed(), 65535);
    value_a.set(&mut view["a"], Value::Signed(-7));
    assert_eq!(value_a.get(&view["a"]).to_signed(), -7);
    assert_eq!(value_a.get_bits(&view["a"]), 9);
    value_a.set(&mut view["a"], Value::Signed(-6));
    assert_eq!(value_a.get(&view["a"]).to_signed(), -6);
    assert_eq!(value_a.get_bits(&view["a"]), 10);
    value_a.set(&mut view["a"], Value::Signed(-5));
    assert_eq!(value_a.get(&view["a"]).to_signed(), -5);
    assert_eq!(value_a.get_bits(&view["a"]), 11);
    value_a.set(&mut view["a"], Value::Signed(1));
    assert_eq!(value_a.get(&view["a"]).to_signed(), 1);
    assert_eq!(value_a.get_bits(&view["a"]), 1);
    value_a.set(&mut view["a"], Value::Signed(4));
    assert_eq!(value_a.get(&view["a"]).to_signed(), 4);
    assert_eq!(value_a.get_bits(&view["a"]), 4);
}
