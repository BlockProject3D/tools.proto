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

mod message;
mod message_from_slice;
mod util;
mod message_write;
mod structure;

use bp3d_util::simple_error;
use itertools::Itertools;
use crate::compiler::Protocol;
use crate::gen::{File, Generator};
use crate::gen::rust::message::gen_message_decl;
use crate::gen::rust::message_from_slice::gen_message_from_slice_impl;
use crate::gen::rust::message_write::gen_message_write_impl;
use crate::gen::rust::structure::gen_structure_decl;

simple_error! {
    pub Error {
        Unknown => "unknown"
    }
}

pub struct GeneratorRust;

impl Generator for GeneratorRust {
    type Error = Error;

    fn generate(proto: Protocol) -> Result<Vec<File>, Self::Error> {
        let decl_messages_code = proto.messages.iter().map(|v| gen_message_decl(v)).join("\n");
        let impl_from_slice_messages_code = proto.messages.iter().map(|v| gen_message_from_slice_impl(v)).join("\n");
        let impl_write_messages_code = proto.messages.iter().map(|v| gen_message_write_impl(v)).join("\n");
        let decl_structures = proto.structs.iter().map(|v| gen_structure_decl(v)).join("\n");
        Ok(vec![
            File {
                name: "messages.rs".into(),
                data: decl_messages_code
            },
            File {
                name: "messages_from_slice.rs".into(),
                data: impl_from_slice_messages_code
            },
            File {
                name: "messages_write.rs".into(),
                data: impl_write_messages_code
            },
            File {
                name: "structures.rs".into(),
                data: decl_structures
            }
        ])
    }
}
