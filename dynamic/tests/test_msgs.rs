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
use bp3d_proto_dynamic::field::string::{NullTerminatedString, VarcharString};
use bp3d_proto_dynamic::component::factory::{Factory, Key, SizeType};
use bp3d_proto_dynamic::proto::Proto;
use bp3d_protoc::api::core::loader::{Loader, Options};

#[test]
fn test_msgs() {
    let mut factory = Factory::new();
    factory.add_component(Key::for_buffer("string", Some(SizeType::U8)), VarcharString(SizeType::U8)).unwrap();
    factory.add_component(Key::for_buffer("string", None), NullTerminatedString).unwrap();
    let mut loader = Loader::new(16);
    loader.load_from_folder("../testprog/src", &Options::from_package("testprog")).unwrap();
    loader.exclude("custom_codec_broken");
    let proto = Proto::build(loader, factory).unwrap();
    let msg = proto.get_message("test.Test").unwrap().new_instance(true);
    println!("{:?}", msg)
}
