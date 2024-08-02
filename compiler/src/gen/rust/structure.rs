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

use crate::compiler::structure::{Field, FieldView, FixedField, Structure};
use crate::compiler::util::TypePathMap;
use crate::gen::rust::util::gen_field_type;

fn gen_structure_impl_new(s: &Structure) -> String {
    let mut code = format!("impl<T> {}<T> {{\n", s.name);
    code += "    pub fn new(data: T) -> Self {\n";
    code += "        Self { data }\n";
    code += "    }\n";
    code += "}\n";
    code += &format!("impl {}<[u8; {}]> {{\n", s.name, s.byte_size);
    code += "    pub fn new_on_stack() -> Self {\n";
    code += &format!("        Self {{ data: [0; {}] }}\n", s.byte_size);
    code += "    }\n";
    code += "}\n";
    code
}

fn gen_structure_impl_fixed_size(s: &Structure) -> String {
    let mut code = format!("impl<T> bp3d_proto::util::FixedSize for {}<T> {{\n", s.name);
    code += &format!("    const SIZE: usize = {};\n", s.byte_size);
    code += "}\n";
    code
}

fn gen_structure_impl_from_slice(s: &Structure) -> String {
    let mut code = format!("impl<'a> bp3d_proto::message::FromSlice<'a> for {}<&'a [u8]> {{\n", s.name);
    code += "    type Output = Self;\n\n";
    code += "    fn from_slice(slice: &'a [u8]) -> bp3d_proto::message::Result<bp3d_proto::message::Message<Self>> {\n";
    code += "        if slice.len() < <Self as bp3d_proto::util::FixedSize>::SIZE {\n";
    code += "            Err(bp3d_proto::message::Error::Truncated)\n";
    code += "        } else {\n";
    code += "            Ok(bp3d_proto::message::Message::new(<Self as bp3d_proto::util::FixedSize>::SIZE, Self::new(&slice[..<Self as bp3d_proto::util::FixedSize>::SIZE])))\n";
    code += "        }\n";
    code += "    }\n";
    code += "}\n";
    code
}

fn gen_structure_impl_write_to(s: &Structure) -> String {
    let mut code = format!("impl<'a> bp3d_proto::message::WriteTo for {}<&'a [u8]> {{\n", s.name);
    code += "    type Input = Self;\n\n";
    code += "    fn write_to<W: std::io::Write>(input: &Self, mut out: W) -> bp3d_proto::message::Result<()> {\n";
    code += "        out.write_all(&input.data)?;\n";
    code += "        Ok(())\n";
    code += "    }\n";
    code += "}\n";
    code
}

fn gen_field_getter(field: &Field, type_path_by_name: &TypePathMap) -> String {
    match field {
        Field::Fixed(v) => {
            let raw_field_type = v.loc.get_unsigned_integer_type();
            let raw_field_byte_size = raw_field_type.get_byte_size();
            let raw_field_type = gen_field_type(raw_field_type);
            let function_name = match raw_field_byte_size != v.loc.byte_size {
                true => "read_unaligned",
                false => "read_aligned"
            };
            let mut code = format!("    pub fn get_raw_{}(&self) -> {} {{\n", v.name, raw_field_type);
            if v.loc.bit_size % 8 != 0 {
                code += &format!("        unsafe {{ bp3d_proto::codec::BitCodec::new(&self.data.as_ref()[{}..{}]).{}::<{}, {}, {}>() }}\n", v.loc.byte_offset, v.loc.byte_offset + v.loc.byte_size, function_name, raw_field_type, v.loc.bit_offset, v.loc.bit_size);
            } else {
                code += &format!("        unsafe {{ bp3d_proto::codec::ByteCodec::new(&self.data.as_ref()[{}..{}]).{}::<{}>() }}\n", v.loc.byte_offset, v.loc.byte_offset + v.loc.byte_size, function_name, raw_field_type);
            }
            code += "    }\n";
            code += &gen_field_view_getter(v, type_path_by_name);
            code
        }
        Field::Array(v) => {
            let field_type = gen_field_type(v.ty);
            let mut code = format!("    pub fn get_{}(&self) -> bp3d_proto::codec::ArrayCodec<&[u8], {}, {}> {{\n", v.name, field_type, v.item_bit_size());
            code += &format!("        bp3d_proto::codec::ArrayCodec::new(&self.data.as_ref()[{}..{}])\n", v.loc.byte_offset, v.loc.byte_offset + v.loc.byte_size);
            code += "    }\n";
            code
        },
        Field::Struct(v) => {
            let mut code = format!("    pub fn get_{}(&self) -> {}<&[u8]> {{\n", v.name, type_path_by_name.get(&v.r.name));
            code += &format!("        {}::new(&self.data.as_ref()[{}..{}])\n", v.r.name, v.loc.byte_offset, v.loc.byte_offset + v.loc.byte_size);
            code += "    }\n";
            code
        }
    }
}

fn gen_field_setter(field: &Field, type_path_by_name: &TypePathMap) -> String {
    match field {
        Field::Fixed(v) => {
            let raw_field_type = v.loc.get_unsigned_integer_type();
            let raw_field_byte_size = raw_field_type.get_byte_size();
            let raw_field_type = gen_field_type(raw_field_type);
            let mut code = format!("    pub fn set_raw_{}(&mut self, value: {}) {{\n", v.name, raw_field_type);
            let function_name = match raw_field_byte_size != v.loc.byte_size {
                true => "write_unaligned",
                false => "write_aligned"
            };
            if v.loc.bit_size % 8 != 0 {
                code += &format!("        unsafe {{ bp3d_proto::codec::BitCodec::new(&mut self.data.as_mut()[{}..{}]).{}::<{}, {}, {}>(value) }}\n", v.loc.byte_offset, v.loc.byte_offset + v.loc.byte_size, function_name, raw_field_type, v.loc.bit_offset, v.loc.bit_size);
            } else {
                code += &format!("        unsafe {{ bp3d_proto::codec::ByteCodec::new(&mut self.data.as_mut()[{}..{}]).{}::<{}>(value) }}\n", v.loc.byte_offset, v.loc.byte_offset + v.loc.byte_size, function_name, raw_field_type);
            }
            code += "    }\n";
            code += &gen_field_view_setter(v, type_path_by_name);
            code
        }
        Field::Array(v) => {
            let field_type = gen_field_type(v.ty);
            let mut code = format!("    pub fn get_{}_mut(&mut self) -> bp3d_proto::codec::ArrayCodec<&mut [u8], {}, {}> {{\n", v.name, field_type, v.item_bit_size());
            code += &format!("        bp3d_proto::codec::ArrayCodec::new(&mut self.data.as_mut()[{}..{}])\n", v.loc.byte_offset, v.loc.byte_offset + v.loc.byte_size);
            code += "    }\n";
            code
        }
        Field::Struct(v) => {
            let mut code = format!("    pub fn get_{}_mut(&mut self) -> {}<&mut [u8]> {{\n", v.name, type_path_by_name.get(&v.r.name));
            code += &format!("        {}::new(&mut self.data.as_mut()[{}..{}])\n", v.r.name, v.loc.byte_offset, v.loc.byte_offset + v.loc.byte_size);
            code += "    }\n";
            code
        }
    }
}

fn gen_field_view_getter(field: &FixedField, type_path_by_name: &TypePathMap) -> String {
    match &field.view {
        FieldView::Float { a, b, .. } => {
            let field_type = gen_field_type(field.ty);
            let mut code = format!("    pub fn get_{}(&self) -> {} {{\n", field.name, field_type);
            code += &format!("        let raw_value = self.get_raw_{}() as {};\n", field.name, field_type);
            code += &format!("        raw_value * {:?} + {:?}\n", a, b);
            code += "    }\n";
            code
        },
        FieldView::Enum(e) => {
            let item_type = type_path_by_name.get(&e.name);
            let raw_field_type = gen_field_type(field.loc.get_unsigned_integer_type());
            let mut code = format!("    pub fn get_{}(&self) -> Option<{}> {{\n", field.name, item_type);
            code += &format!("        let raw_value = self.get_raw_{}();\n", field.name);
            code += &format!("        if raw_value > {} {{\n", e.largest);
            code += "            None\n";
            code += "        } else {\n";
            code += &format!("        unsafe {{ std::mem::transmute::<{}, {}>(raw_value) }}\n", raw_field_type, item_type);
            code += "        }\n";
            code += "    }\n";
            code
        },
        FieldView::Transmute => {
            let field_type = gen_field_type(field.ty);
            let raw_field_type = gen_field_type(field.loc.get_unsigned_integer_type());
            let mut code = format!("    pub fn get_{}(&self) -> {} {{\n", field.name, field_type);
            if field_type == "bool" {
                code += &format!("        if self.get_raw_{}() != 0 {{ true }} else {{ false }}\n", field.name);
            } else {
                code += &format!("        unsafe {{ std::mem::transmute::<{}, {}>(self.get_raw_{}()) }}\n", raw_field_type, field_type, field.name);
            }
            code += "    }\n";
            code
        },
        FieldView::SignedCast { max_positive, max_value} => {
            let field_type = gen_field_type(field.ty);
            let mut code = format!("    pub fn get_{}(&self) -> {} {{\n", field.name, field_type);
            code += &format!("        let raw_value = self.get_raw_{}();\n", field.name);
            code += &format!("        if raw_value > {} {{\n", max_positive);
            code += &format!("            -((((!raw_value) & {max_positive}) + 1) as {})\n", field_type);
            code += "        } else {\n";
            code += &format!("            (raw_value & {}) as {}\n", max_positive, field_type);
            code += "        }\n";
            code += "    }\n";
            code
        },
        FieldView::None => {
            let field_type = gen_field_type(field.ty);
            let mut code = format!("    pub fn get_{}(&self) -> {} {{\n", field.name, field_type);
            code += &format!("        self.get_raw_{}()\n", field.name);
            code += "    }\n";
            code
        }
    }
}

fn gen_field_view_setter(field: &FixedField, type_path_by_name: &TypePathMap) -> String {
    match &field.view {
        FieldView::Float { a_inv, b_inv, .. } => {
            let field_type = gen_field_type(field.ty);
            let raw_field_type = gen_field_type(field.loc.get_unsigned_integer_type());
            let mut code = format!("    pub fn set_{}(&mut self, value: {}) -> &mut Self {{\n", field.name, field_type);
            code += &format!("        let raw_value = value * {:?} + {:?};\n", a_inv, b_inv);
            code += &format!("        self.set_raw_{}(raw_value as {});\n", field.name, raw_field_type);
            code += "        self\n";
            code += "    }\n";
            code
        },
        FieldView::Enum(e) => {
            let item_type = type_path_by_name.get(&e.name);
            let raw_field_type = gen_field_type(field.loc.get_unsigned_integer_type());
            let mut code = format!("    pub fn set_{}(&mut self, value: {}) -> &mut Self {{\n", field.name, item_type);
            code += &format!("        self.set_raw_{}(value as {});\n", field.name, raw_field_type);
            code += "        self\n";
            code += "    }\n";
            code
        },
        FieldView::Transmute | FieldView::SignedCast { .. } => {
            let field_type = gen_field_type(field.ty);
            let raw_field_type = gen_field_type(field.loc.get_unsigned_integer_type());
            let mut code = format!("    pub fn set_{}(&mut self, value: {}) -> &mut Self {{\n", field.name, field_type);
            if field_type == "bool" {
                code += &format!("        self.set_raw_{}(if value {{ 1 }} else {{ 0 }});\n", field.name);
            } else {
                code += &format!("        self.set_raw_{}(unsafe {{ std::mem::transmute::<{}, {}>(value) }});\n", field.name, field_type, raw_field_type);
            }
            code += "        self\n";
            code += "    }\n";
            code
        },
        FieldView::None => {
            let field_type = gen_field_type(field.ty);
            let mut code = format!("    pub fn set_{}(&mut self, value: {}) -> &mut Self {{\n", field.name, field_type);
            code += &format!("        self.set_raw_{}(value);\n", field.name);
            code += "        self\n";
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
    let mut code = format!("#[derive(Copy, Clone, Default, Debug)]\npub struct {}<T> {{\n", s.name);
    code += "    data: T\n";
    code += "}\n";
    code += &gen_structure_impl_new(s);
    code += &gen_structure_impl_fixed_size(s);
    code += &gen_structure_impl_write_to(s);
    code += &gen_structure_impl_from_slice(s);
    code += &gen_structure_getters(s, type_path_by_name);
    code += &gen_structure_setters(s, type_path_by_name);
    code
}
