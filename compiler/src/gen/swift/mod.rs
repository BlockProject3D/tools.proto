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

use crate::compiler::Protocol;
use crate::gen::swift::structure::gen_structure_decl;
use crate::gen::{File, FileType, Generator};
use bp3d_util::simple_error;
use itertools::Itertools;
use crate::gen::swift::message::gen_message_decl;
use crate::gen::swift::r#enum::gen_enum_decl;

mod structure;
mod util;
mod r#enum;
mod message;
mod message_from_slice;

simple_error! {
    pub Error {
        Unknown => "unknown"
    }
}

pub struct GeneratorSwift;

impl Generator for GeneratorSwift {
    type Error = Error;

    fn generate(proto: Protocol) -> Result<Vec<File>, Self::Error> {
        let decl_structures = proto
            .structs
            .iter()
            .map(|v| gen_structure_decl(&proto, v))
            .join("\n");
        let decl_enums = proto.enums.iter().map(|v| gen_enum_decl(&proto, v)).join("\n");
        let decl_messages_code =
            proto.messages.iter().map(|v| gen_message_decl(&proto, v)).join("\n");
        Ok(vec![
            File::new(FileType::Structure, format!("{}.structures.swift", proto.name), decl_structures),
            File::new(FileType::Enum, format!("{}.enums.swift", proto.name), decl_enums),
            File::new(FileType::Message, format!("{}.messages.swift", proto.name), decl_messages_code)
        ])
    }

    fn get_language_extension() -> &'static str {
        "swift"
    }
}
