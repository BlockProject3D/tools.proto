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
use bp3d_proto_dynamic::field::reader::PrimitiveReader;
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

    let reader = PrimitiveReader::from_struct(proto.get_structure("bits.Numbers").unwrap());

    reader.set(&mut view["a"], -8).unwrap();
    reader.set(&mut view["b"], 15).unwrap();
    reader.set(&mut view["c"], -65536).unwrap();
    reader.set(&mut view["d"], 127).unwrap();

    assert_eq!(reader.get(&view["a"]).unwrap().to_signed(), -8);
    assert_eq!(reader.get(&view["b"]).unwrap().to_unsigned(), 15);
    assert_eq!(reader.get(&view["c"]).unwrap().to_signed(), -65536);
    assert_eq!(reader.get(&view["d"]).unwrap().to_unsigned(), 127);

    reader.set(&mut view["c"], 65535).unwrap();
    assert_eq!(reader.get(&view["c"]).unwrap().to_signed(), 65535);
    reader.set(&mut view["a"], -7).unwrap();
    assert_eq!(reader.get(&view["a"]).unwrap().to_signed(), -7);
    assert_eq!(reader.get_bin(&view["a"]).unwrap(), 9);
    reader.set(&mut view["a"], -6).unwrap();
    assert_eq!(reader.get(&view["a"]).unwrap().to_signed(), -6);
    assert_eq!(reader.get_bin(&view["a"]).unwrap(), 10);
    reader.set(&mut view["a"], -5).unwrap();
    assert_eq!(reader.get(&view["a"]).unwrap().to_signed(), -5);
    assert_eq!(reader.get_bin(&view["a"]).unwrap(), 11);
    reader.set(&mut view["a"], 1).unwrap();
    assert_eq!(reader.get(&view["a"]).unwrap().to_signed(), 1);
    assert_eq!(reader.get_bin(&view["a"]).unwrap(), 1);
    reader.set(&mut view["a"], 4).unwrap();
    assert_eq!(reader.get(&view["a"]).unwrap().to_signed(), 4);
    assert_eq!(reader.get_bin(&view["a"]).unwrap(), 4);
}
