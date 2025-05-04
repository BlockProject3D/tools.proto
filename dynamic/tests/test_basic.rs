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

use bp3d_proto_dynamic::component::ComponentType;
use bp3d_proto_dynamic::component::factory::Factory;
use bp3d_proto_dynamic::proto::Proto;
use bp3d_protoc::api::core::loader::{Loader, Options};

#[test]
fn test_basic2() {
    let bytes: [u8; 4] = [0xFF; 4];
    let mut loader = Loader::new(16);
    loader.load_from_folder("../testprog/src", &Options::from_package("testprog")).unwrap();
    loader.exclude("custom_codec_broken");
    let proto = Proto::build(loader, Factory::new()).unwrap();
    let mut view = proto.get_structure("bits.Numbers").unwrap().new_instance(false);
    view.read_view(&bytes).unwrap();
    println!("{}", view);
}

#[test]
fn test_basic() {
    let mut loader = Loader::new(16);
    loader.load_from_folder("../testprog/src", &Options::from_package("testprog")).unwrap();
    loader.exclude("custom_codec_broken");
    let proto = Proto::build(loader, Factory::new()).unwrap();
    let mut view = proto.get_structure("bits.Numbers").unwrap().new_instance(false);
    view.read_copy(b"abcd").unwrap();
    view.buffer_mut().copy_from(b"1234");

    view["a"].get_primitive_mut().unwrap().set(-8);
    view["b"].get_primitive_mut().unwrap().set(15);
    view["c"].get_primitive_mut().unwrap().set(-65536);
    view["d"].get_primitive_mut().unwrap().set(127);

    assert_eq!(view["a"].get_primitive().unwrap().get().to_signed(), -8);
    assert_eq!(view["b"].get_primitive().unwrap().get().to_unsigned(), 15);
    assert_eq!(view["c"].get_primitive().unwrap().get().to_signed(), -65536);
    assert_eq!(view["d"].get_primitive().unwrap().get().to_unsigned(), 127);

    view["c"].get_primitive_mut().unwrap().set(65535);
    assert_eq!(view["c"].get_primitive().unwrap().get().to_signed(), 65535);

    let mut value_a = view["a"].get_primitive_mut().unwrap();
    value_a.set(-7);
    assert_eq!(value_a.get().to_signed(), -7);
    assert_eq!(value_a.get_bin(), 9);
    value_a.set(-6);
    assert_eq!(value_a.get().to_signed(), -6);
    assert_eq!(value_a.get_bin(), 10);
    value_a.set(-5);
    assert_eq!(value_a.get().to_signed(), -5);
    assert_eq!(value_a.get_bin(), 11);
    value_a.set(1);
    assert_eq!(value_a.get().to_signed(), 1);
    assert_eq!(value_a.get_bin(), 1);
    value_a.set(4);
    assert_eq!(value_a.get().to_signed(), 4);
    assert_eq!(value_a.get_bin(), 4);
    println!("{}", view);
}
