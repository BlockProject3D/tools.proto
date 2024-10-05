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
use crate::compiler::r#enum::Enum;
use crate::compiler::structure::Structure;
use crate::compiler::union::Union;
use crate::compiler::util::types::{Name, PtrKey};
use crate::compiler::Protocol;
use crate::model::typedef::Typedef;
use std::rc::Rc;

#[derive(Copy, Clone)]
pub enum Import<'a> {
    Struct(&'a Rc<Structure>),
    Enum(&'a Rc<Enum>),
    Union(&'a Rc<Union>),
    Message(&'a Rc<Message>),
    Type(&'a Rc<Typedef>),
}

impl<'a> Name for Import<'a> {
    fn name(&self) -> &str {
        match self {
            Import::Struct(v) => v.name(),
            Import::Enum(v) => v.name(),
            Import::Union(v) => v.name(),
            Import::Message(v) => v.name(),
            Import::Type(v) => v.name(),
        }
    }
}

impl<'a> PtrKey for Import<'a> {
    fn ptr_key(&self) -> usize {
        match self {
            Import::Struct(v) => v.ptr_key(),
            Import::Enum(v) => v.ptr_key(),
            Import::Union(v) => v.ptr_key(),
            Import::Message(v) => v.ptr_key(),
            Import::Type(v) => v.ptr_key(),
        }
    }
}

impl<'a> Import<'a> {
    pub fn insert(self, type_name: String, proto: &mut Protocol) {
        match self {
            Import::Struct(v) => {
                proto.structs.insert_import(type_name, v.clone());
            }
            Import::Enum(v) => {
                proto.enums.insert_import(type_name, v.clone());
            }
            Import::Union(v) => {
                proto.unions.insert_import(type_name, v.clone());
            }
            Import::Message(v) => {
                proto.messages.insert_import(type_name, v.clone());
            }
            Import::Type(v) => {
                proto.types.insert_import(type_name, v.clone());
            }
        }
    }
}
