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

use crate::compiler::message::{Field, FieldType, Message, Referenced};
use crate::compiler::structure::FixedFieldType;
use crate::compiler::util::TypeMapper;
use crate::gen::base::map::TypePathMapper;
use crate::gen::template::Template;
use crate::model::protocol::Endianness;
use itertools::Itertools;

pub enum StringType {
    Varchar,
    NullTerminated,
}

pub trait Utilities: crate::gen::base::structure::Utilities {
    fn get_value_type(endianness: Endianness, ty: FixedFieldType) -> &'static str;
    fn get_value_type_inline(endianness: Endianness, ty: FixedFieldType) -> &'static str;
    fn gen_option_type(ty: &str) -> String;
    fn gen_option_type_inline(ty: &str) -> String;
    fn get_string_type(ty: StringType) -> &'static str;
    fn get_string_type_inline(ty: StringType) -> &'static str;
    fn get_payload_type() -> &'static str;
    fn get_payload_type_inline() -> &'static str;
    fn gen_struct_ref_type(type_name: &str) -> String;
    fn gen_message_ref_type(type_name: &str) -> String;
}

pub fn gen_msg_field_decl<U: Utilities, T: TypeMapper>(
    field: &Field,
    template: &Template,
    type_path_map: &TypePathMapper<T>,
) -> String {
    let mut scope = template.scope();
    scope.var("name", &field.name);
    let msg_type = match &field.ty {
        FieldType::Fixed(ty) => U::get_field_type(ty.ty).into(),
        FieldType::Ref(v) => match v {
            Referenced::Struct(v) => U::gen_struct_ref_type(&type_path_map.get(v)),
            Referenced::Message(v) => U::gen_message_ref_type(&type_path_map.get(v)),
        },
        FieldType::NullTerminatedString => U::get_string_type(StringType::NullTerminated).into(),
        FieldType::VarcharString(_) => U::get_string_type(StringType::Varchar).into(),
        FieldType::Array(v) => scope
            .var("codec", U::get_value_type(field.endianness, v.ty))
            .var("type_name", type_path_map.get(&v.item_type))
            .render("", &["array"])
            .unwrap(),
        FieldType::Union(v) => scope.var("type_name", type_path_map.get(&v.r)).render("", &["union"]).unwrap(),
        FieldType::List(v) => scope
            .var("codec", U::get_value_type(field.endianness, v.ty))
            .var("type_name", type_path_map.get(&v.item_type))
            .render("", &["list"])
            .unwrap(),
        FieldType::Payload => U::get_payload_type().into(),
        FieldType::SizedList(v) => scope
            .var("codec", U::get_value_type(field.endianness, v.ty))
            .var("type_name", type_path_map.get(&v.item_type))
            .var("size_codec", U::get_value_type(field.endianness, v.size_ty))
            .render("", &["sized_list"])
            .unwrap(),
    };
    let msg_type = match field.optional {
        true => U::gen_option_type(&msg_type),
        false => msg_type,
    };
    scope.var("type", msg_type).render("decl", &["field"]).unwrap()
}

pub fn generate<'variable, U: Utilities, T: TypeMapper>(
    mut template: Template<'_, 'variable>,
    msg: &'variable Message,
    type_path_map: &TypePathMapper<T>,
) -> String {
    template.var("msg_name", &msg.name);
    let fields = msg.fields.iter().map(|v| gen_msg_field_decl::<U, T>(v, &template, type_path_map)).join("");
    template.scope().var("fields", fields).render("", &["decl"]).unwrap()
}
