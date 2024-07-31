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

use crate::compiler::structure::{Field, Structure};
use crate::compiler::util::TypePathMap;
use crate::gen::rust::util::gen_field_type;

fn gen_structure_impl_new(s: &Structure) -> String {
    let mut code = format!("impl<T> {}<T> {{\n", s.name);
    code += "    pub fn new(data: T) -> Self {\n";
    code += "        Self { data }\n";
    code += "    }\n";
    code += "}\n";
    code
}

fn gen_structure_impl_fixed_size(s: &Structure) -> String {
    let mut code = format!("impl<T> bp3d_proto::FixedSize for {}<T> {{\n", s.name);
    code += &format!("    const SIZE: usize = {};\n", s.byte_size);
    code += "}\n";
    code
}

fn gen_structure_impl_from_slice(s: &Structure) -> String {
    let mut code = format!("impl<'a> bp3d_proto::message::FromSlice<'a> for {}<&'a [u8]> {{\n", s.name);
    code += "    type Output = Self;\n\n";
    code += "    fn from_slice(slice: &'a [u8]) -> Result<bp3d_proto::message::Message<Self>, bp3d_proto::message::Error> {\n";
    code += "        if slice.len() < <Self as bp3d_proto::FixedSize>::SIZE {\n";
    code += "            Err(bp3d_proto::message::Error::Truncated)\n";
    code += "        } else {\n";
    code += "            Ok(bp3d_proto::message::Message::new(<Self as bp3d_proto::FixedSize>::SIZE, Self::new(&slice[..<Self as bp3d_proto::FixedSize>::SIZE])))\n";
    code += "        }\n";
    code += "    }\n";
    code += "}\n";
    code
}

fn gen_field_getter(field: &Field, type_path_by_name: &TypePathMap) -> String {
    match field {
        Field::Fixed(v) => {
            let raw_field_type = gen_field_type(v.loc.get_unsigned_integer_type());
            let mut code = format!("    pub fn get_raw_{}(&self) -> {} {{\n", v.name, raw_field_type);
            code += &format!("        bp3d_proto::util::Codec::new(&self.data.as_ref()[{}..{}]).read::<{}, {}, {}>()\n", v.loc.byte_offset, v.loc.byte_size, raw_field_type, v.loc.bit_offset, v.loc.bit_size);
            code += "    }\n";
            code
        }
        Field::Array(v) => {
            let raw_field_type = gen_field_type(v.loc.get_unsigned_integer_type());
            let mut code = format!("    pub fn get_{}(&self) -> bp3d_proto::util::ArrayCodec<&[u8], {}, {}> {{\n", v.name, raw_field_type, v.item_bit_size());
            code += &format!("        bp3d_proto::util::ArrayCodec::new(&self.data.as_ref()[{}..{}])\n", v.loc.byte_offset, v.loc.byte_size);
            code += "    }\n";
            code
        },
        Field::Struct(v) => {
            let mut code = format!("    pub fn get_{}(&self) -> {} {{\n", v.name, type_path_by_name.get(&v.r.name));
            code += &format!("        {}::new(&self.data.as_ref()[{}..{}])\n", v.r.name, v.loc.byte_offset, v.loc.byte_size);
            code += "    }\n";
            code
        }
    }
}

fn gen_field_setter(field: &Field, type_path_by_name: &TypePathMap) -> String {
    match field {
        Field::Fixed(v) => {
            let raw_field_type = gen_field_type(v.loc.get_unsigned_integer_type());
            let mut code = format!("    pub fn set_raw_{}(&mut self, value: {}) {{\n", v.name, raw_field_type);
            code += &format!("        bp3d_proto::util::Codec::new(&mut self.data.as_mut()[{}..{}]).write::<{}, {}, {}>(value)\n", v.loc.byte_offset, v.loc.byte_size, raw_field_type, v.loc.bit_offset, v.loc.bit_size);
            code += "    }\n";
            code
        }
        Field::Array(v) => {
            let raw_field_type = gen_field_type(v.loc.get_unsigned_integer_type());
            let mut code = format!("    pub fn get_{}_mut(&mut self) -> bp3d_proto::util::ArrayCodec<&mut [u8], {}, {}> {{\n", v.name, raw_field_type, v.item_bit_size());
            code += &format!("        bp3d_proto::util::ArrayCodec::new(&mut self.data.as_mut()[{}..{}])\n", v.loc.byte_offset, v.loc.byte_size);
            code += "    }\n";
            code
        }
        Field::Struct(v) => {
            let mut code = format!("    pub fn get_{}_mut(&self) -> {} {{\n", v.name, type_path_by_name.get(&v.r.name));
            code += &format!("        {}::new(&mut self.data.as_mut()[{}..{}])\n", v.r.name, v.loc.byte_offset, v.loc.byte_size);
            code += "    }\n";
            code
        }
    }
}

fn gen_structure_getters(s: &Structure, type_path_by_name: &TypePathMap) -> String {
    let mut code = format!("impl<T: AsRef<[u8]>> {}<T> {{\n", s.name);
    for v in &s.fields {
        code += &gen_field_getter(v, type_path_by_name);
    }
    code += "}\n";
    code
}

fn gen_structure_setters(s: &Structure, type_path_by_name: &TypePathMap) -> String {
    let mut code = format!("impl<T: AsMut<[u8]>> {}<T> {{\n", s.name);
    for v in &s.fields {
        code += &gen_field_setter(v, type_path_by_name);
    }
    code += "}\n";
    code
}

pub fn gen_structure_decl(s: &Structure, type_path_by_name: &TypePathMap) -> String {
    let mut code = format!("#[derive(Copy, Clone)]\npub struct {}<T> {{\n", s.name);
    code += "    data: T\n";
    code += "}\n";
    code += &gen_structure_impl_new(s);
    code += &gen_structure_impl_fixed_size(s);
    code += &gen_structure_impl_from_slice(s);
    code += &gen_structure_getters(s, type_path_by_name);
    code += &gen_structure_setters(s, type_path_by_name);
    code
}