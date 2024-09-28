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

mod r#enum;
mod message;
mod message_from_bytes;
mod message_offsets;
mod message_write;
mod solver;
pub mod structure;
mod union;
mod util;

pub use solver::RustImportSolver;

use crate::compiler::Protocol;
use crate::gen::file::B;
use crate::gen::rust::message::gen_message_decl;
use crate::gen::rust::message_from_bytes::gen_message_from_slice_impl;
use crate::gen::rust::message_offsets::gen_message_offsets_decl;
use crate::gen::rust::message_write::gen_message_write_impl;
use crate::gen::rust::r#enum::gen_enum_decl;
use crate::gen::rust::structure::gen_structure_decl;
use crate::gen::rust::union::gen_union_decl;
use crate::gen::{
    file::{File, FileType},
    Generator,
};
use bp3d_debug::trace;
use bp3d_util::simple_error;
use std::collections::HashSet;
use std::path::Path;

simple_error! {
    pub Error {
        Unknown => "unknown"
    }
}

#[derive(Default, Debug)]
pub struct Params<'a> {
    enable_struct_to_mut: bool,
    enable_struct_dupe: HashSet<&'a str>,
    enable_write_async: bool,
    enable_union_set_discriminant: bool,
    enable_list_wrappers: bool,
    enable_message_offsets: bool,
    disable_read: HashSet<&'a str>,
    disable_write: HashSet<&'a str>,
}

impl<'a> Params<'a> {
    pub fn enable_struct_to_mut(mut self, flag: bool) -> Self {
        self.enable_struct_to_mut = flag;
        self
    }

    pub fn enable_struct_dupe(mut self, name: &'a str) -> Self {
        self.enable_struct_dupe.insert(name);
        self
    }

    pub fn enable_union_set_discriminant(mut self, flag: bool) -> Self {
        self.enable_union_set_discriminant = flag;
        self
    }

    pub fn enable_list_wrappers(mut self, flag: bool) -> Self {
        self.enable_list_wrappers = flag;
        self
    }

    pub fn enable_message_offsets(mut self, flag: bool) -> Self {
        self.enable_message_offsets = flag;
        self
    }

    pub fn enable_write_async(mut self, flag: bool) -> Self {
        self.enable_write_async = flag;
        self
    }

    pub fn disable_read(mut self, name: &'a str) -> Self {
        self.disable_read.insert(name);
        self
    }

    pub fn disable_write(mut self, name: &'a str) -> Self {
        self.disable_write.insert(name);
        self
    }
}

pub struct GeneratorRust;

impl Generator for GeneratorRust {
    type Error = Error;
    type Params<'a> = Params<'a>;

    fn generate(proto: &Protocol, params: &Params) -> Result<Vec<File>, Self::Error> {
        trace!({?params}, "Generating protocol {}", proto.full_name);
        let decl_messages_code = proto.messages.iter().map(|v| gen_message_decl(v, &proto.type_path_map, params));
        let impl_from_slice_messages_code =
            proto.messages.iter().map(|v| gen_message_from_slice_impl(v, &proto.type_path_map));
        let impl_write_messages_code =
            proto.messages.iter().map(|v| gen_message_write_impl(v, &proto.type_path_map, params));
        let decl_structures = proto.structs.iter().map(|v| gen_structure_decl(v, &proto.type_path_map, params));
        let decl_enums = proto.enums.iter().map(|v| gen_enum_decl(v));
        let decl_unions = proto.unions.iter().map(|v| gen_union_decl(v, &proto.type_path_map, params));
        let mut files = vec![
            File::new(FileType::Message, "messages.rs", B(decl_messages_code)),
            File::new(
                FileType::MessageReading,
                "messages_from_bytes.rs",
                B(impl_from_slice_messages_code),
            ),
            File::new(
                FileType::MessageWriting,
                "messages_write.rs",
                B(impl_write_messages_code),
            ),
            File::new(FileType::Structure, "structures.rs", B(decl_structures)),
            File::new(FileType::Enum, "enums.rs", B(decl_enums)),
            File::new(FileType::Union, "unions.rs", B(decl_unions)),
        ];
        if params.enable_message_offsets {
            let decl_messages_code_offsets =
                proto.messages.iter().map(|v| gen_message_offsets_decl(v, &proto.type_path_map));
            files.push(File::new(
                FileType::MessageReading,
                "messages_offsets.rs",
                B(decl_messages_code_offsets)
            ));
        }
        Ok(files)
    }

    fn generate_umbrella<'a>(
        proto_name: &str,
        files: impl Iterator<Item = &'a Path>,
        _: &Params,
    ) -> Result<String, Self::Error> {
        let mut code = format!("pub mod {} {{\n", proto_name);
        for file in files {
            code += &format!("include!({:?});\n", file);
        }
        code += "}\n";
        Ok(code)
    }

    fn get_language_extension() -> &'static str {
        "rs"
    }
}
