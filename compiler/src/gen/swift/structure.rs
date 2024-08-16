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

use crate::compiler::Protocol;
use crate::compiler::structure::Structure;
use crate::gen::base::structure::{generate, Templates};
use crate::gen::base::{DefaultTypeMapper, TypePathMapper};
use crate::gen::swift::util::SwiftUtils;
use crate::gen::template::Template;

const STRUCT_TEMPLATE: &[u8] = include_bytes!("./structure.template");
const STRUCT_FIELD_TEMPLATE: &[u8] = include_bytes!("./structure.field.template");

pub fn gen_structure_decl(proto: &Protocol, s: &Structure) -> String {
    let mut template = Template::compile(STRUCT_TEMPLATE).unwrap();
    let mut field_template = Template::compile(STRUCT_FIELD_TEMPLATE).unwrap();
    template.var("proto_name", &proto.name);
    field_template.var("proto_name", &proto.name);
    let templates = Templates {
        template,
        field_template,
    };
    generate::<SwiftUtils, _>(templates, s, &TypePathMapper::new(&proto.type_path_by_name, DefaultTypeMapper))
}
