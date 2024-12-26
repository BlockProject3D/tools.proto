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

use crate::compiler::message::{Field, FieldType, Message};
use crate::compiler::structure::FixedFieldType;
use crate::compiler::util::types::TypeMapper;
use crate::gen::base::map::TypePathMapper;
use crate::gen::template::Template;
use crate::model::protocol::Endianness;
use itertools::Itertools;
use std::borrow::Cow;

macro_rules! gen_value_type {
    ($prefix: literal, $ty: expr, $suffix: literal) => {
        match $ty {
            FixedFieldType::Int8 => concat!($prefix, "i8", $suffix),
            FixedFieldType::Int16 => concat!($prefix, "i16", $suffix),
            FixedFieldType::Int32 => concat!($prefix, "i32", $suffix),
            FixedFieldType::Int64 => concat!($prefix, "i64", $suffix),
            FixedFieldType::UInt8 => concat!($prefix, "u8", $suffix),
            FixedFieldType::UInt16 => concat!($prefix, "u16", $suffix),
            FixedFieldType::UInt32 => concat!($prefix, "u32", $suffix),
            FixedFieldType::UInt64 => concat!($prefix, "u64", $suffix),
            FixedFieldType::Float32 => concat!($prefix, "f32", $suffix),
            FixedFieldType::Float64 => concat!($prefix, "f64", $suffix),
            FixedFieldType::Bool => concat!($prefix, "bool", $suffix),
        }
    };
}

pub struct Generic<'a> {
    pub name: Cow<'a, str>,
    pub default: Option<Cow<'a, str>>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Lifetime {
    None,
    Anonymous,
    Named,
}

pub struct Generics<T> {
    lifetime: Lifetime,
    data: T,
}

impl<T> Generics<T> {
    pub fn new(lifetime: Lifetime, data: T) -> Self {
        Self { lifetime, data }
    }
}

fn _to_string<'a>(mut generics: impl Iterator<Item = Cow<'a, str>>, lifetime: Lifetime) -> Cow<'a, str> {
    if let Some(value) = generics.next() {
        let str = generics.join(", ");
        match (str.is_empty(), lifetime) {
            (true, Lifetime::None) => format!("<{}>", value).into(),
            (false, Lifetime::None) => format!("<{}, {}>", value, str).into(),
            (true, Lifetime::Named) => format!("<'a, {}>", value).into(),
            (false, Lifetime::Named) => format!("<'a, {}, {}>", value, str).into(),
            (true, Lifetime::Anonymous) => format!("<'_, {}>", value).into(),
            (false, Lifetime::Anonymous) => format!("<'_, {}, {}>", value, str).into(),
        }
    } else if lifetime == Lifetime::Named {
        Cow::Borrowed("<'a>")
    } else if lifetime == Lifetime::Anonymous {
        Cow::Borrowed("<'_>")
    } else {
        Cow::Borrowed("")
    }
}

impl<'a, T: Iterator<Item = Generic<'a>>> Generics<T> {
    pub fn into_string(self) -> Cow<'a, str> {
        let generics = self.data.map(|v| match &v.default {
            None => v.name,
            Some(_) => v.name,
        });
        _to_string(generics, self.lifetime)
    }

    pub fn into_string_with_defaults(self) -> Cow<'a, str> {
        let generics = self.data.map(|v| match &v.default {
            None => v.name,
            Some(v1) => format!("{}={}", v.name, v1).into(),
        });
        _to_string(generics, self.lifetime)
    }
}

pub struct RustUtils;

impl RustUtils {
    fn _gen_generics<'a, T: TypeMapper>(
        msg: &'a Message,
        type_path_map: &'a TypePathMapper<T>,
        lifetime: Lifetime,
    ) -> Generics<impl Iterator<Item = Generic<'a>>> {
        let unions = msg.fields.iter().filter_map(|v| match &v.ty {
            FieldType::Union(u) => Some(Generic {
                name: format!("T{}", v.name).into(),
                default: Some(format!("{}<'a>", type_path_map.get(&u.r)).into()),
            }),
            _ => None,
        });
        Generics::new(lifetime, unions)
    }

    pub fn get_generics_for_write<'a, T: TypeMapper>(
        msg: &'a Message,
        type_path_map: &'a TypePathMapper<T>,
        lifetime: Lifetime,
    ) -> Generics<impl Iterator<Item = Generic<'a>>> {
        let has_lifetime = msg.fields.iter().any(|v| matches!(v.ty, FieldType::Union(_)));
        Self::_gen_generics(msg, type_path_map, if has_lifetime { lifetime } else { Lifetime::None })
    }

    pub fn get_generics<'a, T: TypeMapper>(
        msg: &'a Message,
        type_path_map: &'a TypePathMapper<T>,
    ) -> Generics<impl Iterator<Item = Generic<'a>>> {
        let has_lifetime = msg.fields.iter().any(|v| {
            matches!(
                v.ty,
                FieldType::Ref(_)
                    | FieldType::Buffer
                    | FieldType::SizedBuffer(_)
                    | FieldType::FixedContainer(_)
                    | FieldType::Union(_)
                    | FieldType::Container(_)
                    | FieldType::SizedContainer(_)
                    | FieldType::Payload
            )
        });
        Self::_gen_generics(
            msg,
            type_path_map,
            if has_lifetime { Lifetime::Named } else { Lifetime::None },
        )
    }
}

impl crate::gen::base::structure::Utilities for RustUtils {
    fn get_field_type(field_type: FixedFieldType) -> &'static str {
        gen_value_type!("", field_type, "")
    }

    fn get_fragment_name(field: &crate::compiler::structure::Field) -> &'static str {
        let raw_field_type = field.ty.as_fixed().unwrap().bits_type;
        let raw_field_byte_size = raw_field_type.get_byte_size();
        match raw_field_byte_size != field.loc.byte_size {
            true => "unaligned",
            false => "aligned",
        }
    }

    fn get_bit_codec_inline(endianness: Endianness) -> &'static str {
        match endianness {
            Endianness::Little => "<bp3d_proto::codec::BitCodecLE as bp3d_proto::codec::BitCodec>",
            Endianness::Big => "<bp3d_proto::codec::BitCodecBE as bp3d_proto::codec::BitCodec>",
        }
    }

    fn get_byte_codec_inline(endianness: Endianness) -> &'static str {
        match endianness {
            Endianness::Little => "<bp3d_proto::codec::ByteCodecLE as bp3d_proto::codec::ByteCodec>",
            Endianness::Big => "<bp3d_proto::codec::ByteCodecBE as bp3d_proto::codec::ByteCodec>",
        }
    }

    fn get_byte_codec(endianness: Endianness) -> &'static str {
        match endianness {
            Endianness::Little => "bp3d_proto::codec::ByteCodecLE",
            Endianness::Big => "bp3d_proto::codec::ByteCodecBE",
        }
    }
}

impl crate::gen::base::message::Utilities for RustUtils {
    fn get_value_type(endianness: Endianness, ty: FixedFieldType) -> &'static str {
        match endianness {
            Endianness::Little => gen_value_type!("bp3d_proto::message::util::ValueLE<", ty, ">"),
            Endianness::Big => gen_value_type!("bp3d_proto::message::util::ValueBE<", ty, ">"),
        }
    }

    fn get_value_type_inline(endianness: Endianness, ty: FixedFieldType) -> &'static str {
        match endianness {
            Endianness::Little => gen_value_type!("bp3d_proto::message::util::ValueLE::<", ty, ">"),
            Endianness::Big => gen_value_type!("bp3d_proto::message::util::ValueBE::<", ty, ">"),
        }
    }

    fn gen_struct_ref_type(type_name: &str) -> String {
        format!("{}<&'a [u8]>", type_name)
    }

    fn gen_message_ref_type(type_name: &str) -> String {
        format!("{}<'a>", type_name)
    }
}

pub fn gen_where_clause<T: TypeMapper>(
    template: &Template,
    field: &Field,
    type_path_map: &TypePathMapper<T>,
    function: &str,
) -> String {
    match &field.ty {
        FieldType::Union(v) => template
            .scope()
            .var("name", &field.name)
            .var("type_name", type_path_map.get(&v.r))
            .var("discriminant_type", type_path_map.get(&v.r.discriminant.root))
            .render(function, &["where"])
            .unwrap(),
        _ => "".into(),
    }
}
