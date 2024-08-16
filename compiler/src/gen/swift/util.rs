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
use crate::compiler::structure::{FixedField, FixedFieldType};
use crate::gen::base::message::StringType;
use crate::model::protocol::Endianness;

macro_rules! gen_value_type {
    ($prefix: literal, $ty: expr, $suffix: literal) => {
        match $ty {
            FixedFieldType::Int8 => concat!($prefix, "Int8", $suffix),
            FixedFieldType::Int16 => concat!($prefix, "Int16", $suffix),
            FixedFieldType::Int32 => concat!($prefix, "Int32", $suffix),
            FixedFieldType::Int64 => concat!($prefix, "Int64", $suffix),
            FixedFieldType::UInt8 => concat!($prefix, "UInt8", $suffix),
            FixedFieldType::UInt16 => concat!($prefix, "UInt16", $suffix),
            FixedFieldType::UInt32 => concat!($prefix, "UInt32", $suffix),
            FixedFieldType::UInt64 => concat!($prefix, "UInt64", $suffix),
            FixedFieldType::Float32 => concat!($prefix, "Float32", $suffix),
            FixedFieldType::Float64 => concat!($prefix, "Float64", $suffix),
            FixedFieldType::Bool => concat!($prefix, "Bool", $suffix),
        }
    };
}

pub struct SwiftUtils;

impl crate::gen::base::structure::Utilities for SwiftUtils {
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
            Endianness::Little => "BP3DProto.BitCodecLE",
            Endianness::Big => "BP3DProto.BitCodecBE",
        }
    }

    fn get_byte_codec_inline(endianness: Endianness) -> &'static str {
        match endianness {
            Endianness::Little => "BP3DProto.ByteCodecLE",
            Endianness::Big => "BP3DProto.ByteCodecBE",
        }
    }

    fn get_byte_codec(endianness: Endianness) -> &'static str {
        match endianness {
            Endianness::Little => "BP3DProto.ByteCodecLE",
            Endianness::Big => "BP3DProto.ByteCodecBE",
        }
    }
}

impl crate::gen::base::message::Utilities for SwiftUtils {
    fn get_generics(_: &Message) -> &str {
        ""
    }

    fn get_value_type(endianness: Endianness, ty: FixedFieldType) -> &'static str {
        match endianness {
            Endianness::Little => gen_value_type!("BP3DProto.ValueLE<", ty, ">"),
            Endianness::Big => gen_value_type!("BP3DProto.ValueBE<", ty, ">")
        }
    }

    fn get_value_type_inline(endianness: Endianness, ty: FixedFieldType) -> &'static str {
        match endianness {
            Endianness::Little => gen_value_type!("BP3DProto.ValueLE<", ty, ">"),
            Endianness::Big => gen_value_type!("BP3DProto.ValueBE<", ty, ">")
        }
    }

    fn gen_option_type(ty: &str) -> String {
        format!("{}?", ty)
    }

    fn gen_option_type_inline(ty: &str) -> String {
        format!("BP3DProto.Optional<{}>", ty)
    }

    fn get_string_type(_: StringType) -> &'static str {
        "String"
    }

    fn get_string_type_inline(ty: StringType) -> &'static str {
        match ty {
            StringType::Varchar => "BP3DProto.VarcharString",
            StringType::NullTerminated => "BP3DProto.NullTerminatedString"
        }
    }

    fn get_payload_type() -> &'static str {
        "Data"
    }

    fn get_payload_type_inline() -> &'static str {
        "Data"
    }

    fn gen_struct_ref_type(type_name: &str) -> String {
        format!("{}<B>", type_name)
    }

    fn gen_message_ref_type(type_name: &str) -> String {
        format!("{}<B>", type_name)
    }

    fn gen_union_ref_type(type_name: &str) -> String {
        format!("{}<B>", type_name)
    }
}
