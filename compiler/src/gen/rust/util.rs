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

use crate::compiler::message::{FieldType, Message};
use crate::compiler::structure::{FixedField, FixedFieldType};
use crate::gen::base::message::StringType;
use crate::model::protocol::Endianness;

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

pub struct RustUtils;

impl crate::gen::base::structure::Utilities for RustUtils {
    fn get_field_type(field_type: FixedFieldType) -> &'static str {
        gen_value_type!("", field_type, "")
    }

    fn get_fragment_name(field: &FixedField) -> &'static str {
        let raw_field_type = field.loc.get_unsigned_integer_type();
        let raw_field_byte_size = raw_field_type.get_byte_size();
        match raw_field_byte_size != field.loc.byte_size {
            true => "unaligned",
            false => "aligned",
        }
    }

    fn get_fragment_name_mut(field: &FixedField) -> &'static str {
        let raw_field_type = field.loc.get_unsigned_integer_type();
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
    fn get_generics(msg: &Message) -> &str {
        let has_lifetime = msg.fields.iter().any(|v| {
            matches!(
                v.ty,
                FieldType::Ref(_)
                    | FieldType::NullTerminatedString
                    | FieldType::VarcharString(_)
                    | FieldType::Array(_)
                    | FieldType::Union(_)
                    | FieldType::List(_)
                    | FieldType::Payload
            )
        });
        if has_lifetime {
            "<'a>"
        } else {
            ""
        }
    }

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

    fn gen_option_type(ty: &str) -> String {
        format!("Option<{}>", ty)
    }

    fn gen_option_type_inline(ty: &str) -> String {
        format!("bp3d_proto::message::util::Optional::<{}>", ty)
    }

    fn get_string_type(_: StringType) -> &'static str {
        "&'a str"
    }

    fn get_string_type_inline(ty: StringType) -> &'static str {
        match ty {
            StringType::Varchar => "bp3d_proto::message::util::VarcharString",
            StringType::NullTerminated => "bp3d_proto::message::util::NullTerminatedString",
        }
    }

    fn get_payload_type() -> &'static str {
        "&'a [u8]"
    }

    fn get_payload_type_inline() -> &'static str {
        "bp3d_proto::message::util::Buffer"
    }

    fn gen_struct_ref_type(type_name: &str) -> String {
        format!("{}<&'a [u8]>", type_name)
    }

    fn gen_message_ref_type(type_name: &str) -> String {
        format!("{}<'a>", type_name)
    }

    fn gen_union_ref_type(type_name: &str) -> String {
        format!("{}<'a>", type_name)
    }
}
