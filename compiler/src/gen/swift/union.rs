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

use crate::compiler::union::{DiscriminantField, Union};
use crate::gen::base::union::{generate, Utilities};
use itertools::Itertools;
use crate::compiler::Protocol;
use crate::gen::base::TypePathMapper;
use crate::gen::swift::util::{SwiftTypeMapper, SwiftUtils};
use crate::gen::template::Template;
use crate::gen::template::util::CaseConversion;

const TEMPLATE: &[u8] = include_bytes!("./union.template");

impl Utilities for SwiftUtils {
    fn gen_discriminant_path(discriminant: &DiscriminantField) -> String {
        discriminant
            .iter()
            .map(|(f, is_leaf)| {
                if is_leaf {
                    format!("raw{}", f.name().to_pascal_case())
                } else {
                    format!("{}", f.name().to_camel_case())
                }
            })
            .join(".")
    }

    fn gen_discriminant_path_mut(discriminant: &DiscriminantField) -> String {
        discriminant
            .iter()
            .map(|(f, is_leaf)| {
                if is_leaf {
                    format!("setRaw{}", f.name().to_pascal_case())
                } else {
                    format!("{}", f.name().to_camel_case())
                }
            })
            .join(".")
    }

    fn get_generics(_: &Union) -> &str {
        ""
    }
}

pub fn gen_union_decl(proto: &Protocol, u: &Union) -> String {
    let type_path_by_name = TypePathMapper::new(&proto.type_path_by_name, SwiftTypeMapper::from_protocol(proto));
    let mut template = Template::compile(TEMPLATE).unwrap();
    template.var("proto_name", &proto.name);
    generate::<SwiftUtils, _>(template, u, &type_path_by_name)
}