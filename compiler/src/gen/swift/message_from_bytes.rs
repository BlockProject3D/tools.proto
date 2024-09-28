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

use crate::compiler::message::Message;
use crate::compiler::Protocol;
use crate::gen::base::map::TypePathMapper;
use crate::gen::base::message_from_bytes::generate;
use crate::gen::swift::util::{SwiftTypeMapper, SwiftUtils};
use crate::gen::template::{Options, Template};
use std::borrow::Cow;

const TEMPLATE: &[u8] = include_bytes!("message.from_bytes.template");

pub fn gen_message_from_slice_impl(proto: &Protocol, msg: &Message) -> String {
    let mut options = Options::default();
    options.functions_mut().add("remove_leading_coma", |v| Cow::Borrowed(&v[..v.len() - 2]));
    let mut template = Template::compile_with_options(TEMPLATE, &options).unwrap();
    template.var("proto_name", proto.name());
    let type_path_map = TypePathMapper::new(&proto.type_path_map, SwiftTypeMapper::from_protocol(proto));
    generate::<SwiftUtils, _>(template, msg, &type_path_map, "impl")
}
