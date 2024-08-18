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
use crate::compiler::util::TypePathMap;
use crate::gen::base::union::{generate, Utilities};
use crate::gen::base::{DefaultTypeMapper, TypePathMapper};
use crate::gen::rust::util::RustUtils;
use crate::gen::template::Template;
use itertools::Itertools;

const TEMPLATE: &[u8] = include_bytes!("./union.template");

impl Utilities for RustUtils {
    fn gen_discriminant_path(discriminant: &DiscriminantField) -> String {
        discriminant
            .iter()
            .map(|(f, is_leaf)| {
                if is_leaf {
                    format!("get_raw_{}()", f.name())
                } else {
                    format!("get_{}()", f.name())
                }
            })
            .join(".")
    }

    fn gen_discriminant_path_mut(discriminant: &DiscriminantField) -> String {
        discriminant
            .iter()
            .map(|(f, is_leaf)| {
                if is_leaf {
                    format!("set_raw_{}", f.name())
                } else {
                    format!("get_{}_mut()", f.name())
                }
            })
            .join(".")
    }

    fn get_generics(u: &Union) -> &str {
        if u.cases.iter().any(|v| v.item_type.is_some()) {
            "<'a>"
        } else {
            ""
        }
    }
}

pub fn gen_union_decl(u: &Union, type_path_by_name: &TypePathMap) -> String {
    generate::<RustUtils, _>(
        Template::compile(TEMPLATE).unwrap(),
        u,
        &TypePathMapper::new(type_path_by_name, DefaultTypeMapper),
    )
}
