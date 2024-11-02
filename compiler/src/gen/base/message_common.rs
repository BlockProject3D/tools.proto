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

use crate::compiler::message::{Field, FieldType, Referenced};
use crate::compiler::util::types::TypeMapper;
use crate::gen::base::map::TypePathMapper;
use crate::gen::base::message::Utilities;
use crate::gen::base::Error;
use crate::gen::template::Template;
use std::borrow::Cow;

fn gen_optional<'a>(
    template: &'a Template,
    optional: bool,
    type_name: impl Into<Cow<'a, str>>,
) -> Result<Cow<'a, str>, Error> {
    if optional {
        template
            .scope()
            .var("msg_type", type_name)
            .render("", &["option"])
            .map(|v| v.into())
            .map_err(Error::Codec)
    } else {
        Ok(type_name.into())
    }
}

pub fn generate_field_type_inline<'a, U: Utilities, T: TypeMapper>(
    field: &'a Field,
    template: &'a Template,
    type_path_map: &'a TypePathMapper<T>,
) -> Result<Cow<'a, str>, Error> {
    let msg_type = match &field.ty {
        FieldType::Fixed(ty) => gen_optional(
            template,
            field.optional,
            U::get_value_type_inline(field.endianness, ty.ty),
        ),
        FieldType::Ref(v) => match v {
            Referenced::Struct(v) => gen_optional(template, field.optional, type_path_map.get(v)),
            Referenced::Message(v) => gen_optional(template, field.optional, type_path_map.get(v)),
        },
        FieldType::NullTerminatedString => gen_optional(
            template,
            field.optional,
            template.scope().render("", &["string"]).map_err(Error::Codec)?,
        ),
        FieldType::SizedString(v) => gen_optional(
            template,
            field.optional,
            template
                .scope()
                .var("codec", U::get_value_type(field.endianness, v.ty))
                .render("", &["sized_string"])
                .map_err(Error::Codec)?,
        ),
        FieldType::Array(v) => gen_optional(
            template,
            field.optional,
            template
                .scope()
                .var("codec", U::get_value_type(field.endianness, v.ty))
                .var("type_name", type_path_map.get(&v.item_type))
                .render("", &["array"])
                .map_err(Error::Codec)?,
        ),
        FieldType::Union(v) => gen_optional(template, field.optional, type_path_map.get(&v.r)),
        FieldType::List(v) => match v.nested {
            false => gen_optional(
                template,
                field.optional,
                template
                    .scope()
                    .var("codec", U::get_value_type(field.endianness, v.ty))
                    .var("type_name", type_path_map.get(&v.item_type))
                    .render("", &["unsized_list"])
                    .map_err(Error::Codec)?,
            ),
            true => gen_optional(
                template,
                field.optional,
                template
                    .scope()
                    .var("codec", U::get_value_type(field.endianness, v.ty))
                    .var("type_name", type_path_map.get(&v.item_type))
                    .render("", &["list"])
                    .map_err(Error::Codec)?,
            ),
        },
        FieldType::Payload => gen_optional(
            template,
            field.optional,
            template.scope().render("", &["payload"]).map_err(Error::Codec)?,
        ),
        FieldType::SizedList(v) => gen_optional(
            template,
            field.optional,
            template
                .scope()
                .var("codec", U::get_value_type(field.endianness, v.ty))
                .var("type_name", type_path_map.get(&v.item_type))
                .var("size_codec", U::get_value_type(field.endianness, v.size_ty))
                .render("", &["sized_list"])
                .map_err(Error::Codec)?,
        ),
    };
    msg_type
}
