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
use crate::compiler::util::types::TypePathMap;
use crate::gen::base::map::{DefaultTypeMapper, TypePathMapper};
use crate::gen::base::message::Templates;
use crate::gen::base::message_write::generate;
use crate::gen::base::Error;
use crate::gen::rust::util::{gen_where_clause, RustUtils};
use crate::gen::template::Template;
use crate::gen::{codec::CodecMap, RustParams};
use itertools::Itertools;

const TEMPLATE: &[u8] = include_bytes!("write.template");

fn _gen_message_write_impl(
    msg: &Message,
    codec_map: &CodecMap,
    type_path_map: &TypePathMapper<DefaultTypeMapper>,
    generics: &str,
    function: &str,
) -> Result<String, Error> {
    let mut templates = Templates {
        template: Template::compile(TEMPLATE).unwrap(),
        codec_map,
    };
    let where_clauses = msg
        .fields
        .iter()
        .map(|field| gen_where_clause(&templates.template, field, type_path_map, function))
        .join("");
    templates.template.var("where_clauses", where_clauses);
    if generics.is_empty() {
        templates.template.var("impl_generics", "").var("generics", "<'_>");
    } else {
        templates.template.var("generics", generics).var("impl_generics", generics);
    }
    generate::<RustUtils, _>(templates, msg, type_path_map, function)
}

pub fn gen_message_write_impl(
    msg: &Message,
    codec_map: &CodecMap,
    type_path_map: &TypePathMap,
    params: &RustParams,
) -> Result<String, Error> {
    let type_path_map = TypePathMapper::new(type_path_map, DefaultTypeMapper);
    let generics = RustUtils::get_generics_for_write(msg, &type_path_map).to_string();
    let mut code = _gen_message_write_impl(msg, codec_map, &type_path_map, &generics, "impl")?;
    if msg.fields.iter().any(|v| v.ty.is_union()) {
        code += &_gen_message_write_impl(msg, codec_map, &type_path_map, &generics, "impl_shape_write")?;
    }
    if params.enable_write_async {
        code += &_gen_message_write_impl(msg, codec_map, &type_path_map, &generics, "impl_async")?;
    }
    Ok(code)
}
