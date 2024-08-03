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

use itertools::Itertools;
use crate::compiler::message::Referenced;
use crate::compiler::union::Union;
use crate::compiler::util::TypePathMap;

fn get_generics(u: &Union) -> &str {
    if u.cases.iter().any(|v| v.item_type.is_some()) {
        "<'a>"
    } else {
        ""
    }
}

fn get_discriminant_path(u: &Union) -> String {
    u.discriminant.iter().map(|(f, is_leaf)| if is_leaf {
        format!("get_raw_{}()", f.name())
    } else {
        format!("get_{}()", f.name())
    }).join(".")
}

fn get_discriminant_path_mut(u: &Union) -> String {
    u.discriminant.iter().map(|(f, is_leaf)| if is_leaf {
        format!("set_raw_{}", f.name())
    } else {
        format!("get_{}_mut()", f.name())
    }).join(".")
}

fn gen_union_from_slice_impl(u: &Union, type_path_by_name: &TypePathMap) -> String {
    let generics = get_generics(u);
    let mut code = format!("impl<'a> {}{generics} {{\n", u.name);
    if generics != "" {
        code += &format!("    pub fn from_slice(slice: &'a [u8], discriminant: &{}<&'a [u8]>) -> bp3d_proto::message::Result<bp3d_proto::message::Message<Self>> {{\n", type_path_by_name.get(&u.discriminant.root.name));
        code += "        use bp3d_proto::message::FromSlice;\n";
    } else {
        code += &format!("    pub fn from_slice(_: &'a [u8], discriminant: &{}<&'a [u8]>) -> bp3d_proto::message::Result<bp3d_proto::message::Message<Self>> {{\n", type_path_by_name.get(&u.discriminant.root.name));
    }
    let discriminant_path = get_discriminant_path(u);
    code += &format!("        let discriminant = discriminant.{};\n", discriminant_path);
    code += "        match discriminant {\n";
    for case in &u.cases {
        match &case.item_type {
            Some(item_type) => code += &format!("            {} => {}::from_slice(slice).map(|v| v.map(Self::{})),\n", case.case, type_path_by_name.get(item_type.name()), case.name),
            None => code += &format!("            {} => Ok(bp3d_proto::message::Message::new(0, Self::{})),\n", case.case, case.name)
        }
    }
    code += "            _ => Err(bp3d_proto::message::Error::InvalidUnionDiscriminant(discriminant as _))\n";
    code += "        }\n";
    code += "    }\n";
    code += "}\n";
    code
}

fn gen_union_write_to_impl(u: &Union, type_path_by_name: &TypePathMap) -> String {
    let generics = get_generics(u);
    let mut code = format!("impl<'a> {}{generics} {{\n", u.name);
    if generics != "" {
        code += &format!("    pub fn write_to<W: std::io::Write>(input: &Self, discriminant: &{}<&'a [u8]>, mut out: W) -> bp3d_proto::message::Result<()> {{\n", type_path_by_name.get(&u.discriminant.root.name));
        code += "        use bp3d_proto::message::WriteTo;\n";
        let discriminant_path = get_discriminant_path(u);
        code += &format!("        let discriminant = discriminant.{};\n", discriminant_path);
    } else {
        code += &format!("    pub fn write_to<W: std::io::Write>(input: &Self, _: &{}<&'a [u8]>, _: W) -> bp3d_proto::message::Result<()> {{\n", type_path_by_name.get(&u.discriminant.root.name));
    }
    code += "        match input {\n";
    for case in &u.cases {
        if let Some(item_type) = &case.item_type {
            code += &format!("            Self::{}(v) => if discriminant == {} {{ {}::write_to(v, &mut out)? }} else {{ return Err(bp3d_proto::message::Error::InvalidUnionDiscriminant(discriminant as _)) }},\n", case.name, case.case, type_path_by_name.get(item_type.name()));
        }
    }
    code += "            _ => ()\n";
    code += "        };\n";
    code += "        Ok(())\n";
    code += "    }\n";
    code += "}\n";
    code
}

fn gen_union_set_discriminant(u: &Union, type_path_by_name: &TypePathMap) -> String {
    let generics = get_generics(u);
    let mut code = format!("impl{generics} {}{generics} {{\n", u.name);
    code += &format!("    pub fn set_discriminant<T: AsMut<[u8]>>(&self, discriminant: &mut {}<T>) {{\n", type_path_by_name.get(&u.discriminant.root.name));
    let discriminant_path = get_discriminant_path_mut(u);
    code += "        let discriminant_value = match self {\n";
    for case in &u.cases {
        match case.item_type {
            None => code += &format!("            Self::{} => {},\n", case.name, case.case),
            Some(_) => code += &format!("            Self::{}(_) => {},\n", case.name, case.case)
        }
    }
    code += "        };\n";
    code += &format!("        discriminant.{}(discriminant_value);\n", discriminant_path);
    code += "    }\n";
    code += "}\n";
    code
}

fn gen_union_as_getters(u: &Union, type_path_by_name: &TypePathMap) -> String {
    let generics = get_generics(u);
    let mut code = format!("impl{generics} {}{generics} {{\n", u.name);
    for case in &u.cases {
        match &case.item_type {
            Some(Referenced::Struct(v)) => code += &format!("    pub fn as_{}(&self) -> Option<&{}<&'a [u8]>> {{\n", case.name.to_ascii_lowercase(), type_path_by_name.get(&v.name)),
            Some(Referenced::Message(v)) => code += &format!("    pub fn as_{}(&self) -> Option<&{}<'a>> {{\n", case.name.to_ascii_lowercase(), type_path_by_name.get(&v.name)),
            _ => code += &format!("    pub fn is_{}(&self) -> bool {{\n", case.name.to_ascii_lowercase()),
        }
        code += "        match self {\n";
        match &case.item_type {
            None => {
                code += &format!("            Self::{} => true,\n", case.name);
                code += "            _ => false\n";
            },
            Some(_) => {
                code += &format!("            Self::{}(v) => Some(v),\n", case.name);
                code += "            _ => None\n";
            }
        }
        code += "        }\n";
        code += "    }\n";
    }
    code += "}\n";
    code
}

pub fn gen_union_decl(u: &Union, type_path_by_name: &TypePathMap) -> String {
    let generics = get_generics(u);
    let mut code = format!("#[derive(Copy, Clone, Debug)]\npub enum {}{generics} {{\n", u.name);
    for case in &u.cases {
        match &case.item_type {
            Some(Referenced::Struct(v)) => code += &format!("    {}({}<&'a [u8]>),\n", case.name, type_path_by_name.get(&v.name)),
            Some(Referenced::Message(v)) => code += &format!("    {}({}<'a>),\n", case.name, type_path_by_name.get(&v.name)),
            None => code += &format!("    {},\n", case.name)
        }
    }
    code += "}\n";
    code += &gen_union_from_slice_impl(u, type_path_by_name);
    code += &gen_union_write_to_impl(u, type_path_by_name);
    code += &gen_union_set_discriminant(u, type_path_by_name);
    code += &gen_union_as_getters(u, type_path_by_name);
    code
}
