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

pub struct MaybeOptional<'b, 'a, 'fragment, 'variable> {
    template: &'a Template<'fragment, 'variable>,
    function: &'b str,
    optional: bool
}

impl<'b, 'a, 'fragment, 'variable> MaybeOptional<'b, 'a, 'fragment, 'variable> {
    pub fn new(field: &Field, function: &'b str, template: &'a Template<'fragment, 'variable>) -> Self {
        Self {
            template,
            function,
            optional: field.optional
        }
    }

    pub fn gen<'c>(self, type_name: impl Into<Cow<'c, str>>) -> Result<Cow<'c, str>, Error> where 'a: 'c {
        if self.optional {
            self.template
                .scope()
                .var("msg_type", type_name)
                .render(self.function, &["option"])
                .map(|v| v.into())
                .map_err(Error::Codec)
        } else {
            Ok(type_name.into())
        }
    }
}

pub fn generate_field_type_inline<'a, U: Utilities, T: TypeMapper>(
    field: &'a Field,
    template: &'a Template,
    type_path_map: &'a TypePathMapper<T>,
    function: &str
) -> Result<Cow<'a, str>, Error> {
    let optional = MaybeOptional::new(field, function, template);
    let msg_type = match &field.ty {
        FieldType::Fixed(ty) => optional.gen(U::get_value_type_inline(field.endianness, ty.ty)),
        FieldType::Ref(v) => match v {
            Referenced::Struct(v) => optional.gen(type_path_map.get(v)),
            Referenced::Message(v) => optional.gen(type_path_map.get(v)),
        },
        FieldType::Buffer => optional.gen(template.scope().render(function, &["buffer"]).map_err(Error::Codec)?),
        FieldType::SizedBuffer(v) => optional.gen(
            template
                .scope()
                .var("codec", U::get_value_type(field.endianness, v.ty))
                .render(function, &["sized_buffer"])
                .map_err(Error::Codec)?
        ),
        FieldType::FixedContainer(v) => optional.gen(
            template
                .scope()
                .var("codec", U::get_value_type(field.endianness, v.ty))
                .var("type_name", type_path_map.get(&v.item_type))
                .render(function, &["fixed_container"])
                .map_err(Error::Codec)?
        ),
        FieldType::Union(v) => optional.gen(type_path_map.get(&v.r)),
        FieldType::Container(v) => match v.nested {
            false => optional.gen(
                template
                    .scope()
                    .var("codec", U::get_value_type(field.endianness, v.ty))
                    .var("type_name", type_path_map.get(&v.item_type))
                    .render(function, &["unsized_container"])
                    .map_err(Error::Codec)?
            ),
            true => optional.gen(
                template
                    .scope()
                    .var("codec", U::get_value_type(field.endianness, v.ty))
                    .var("type_name", type_path_map.get(&v.item_type))
                    .render(function, &["container"])
                    .map_err(Error::Codec)?
            ),
        },
        FieldType::Payload => optional.gen(template.scope().render(function, &["payload"]).map_err(Error::Codec)?),
        FieldType::SizedContainer(v) => optional.gen(
            template
                .scope()
                .var("codec", U::get_value_type(field.endianness, v.ty))
                .var("type_name", type_path_map.get(&v.item_type))
                .var("size_codec", U::get_value_type(field.endianness, v.size_ty))
                .render(function, &["sized_container"])
                .map_err(Error::Codec)?
        ),
    };
    msg_type
}
