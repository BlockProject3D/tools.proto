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

use crate::compiler::error::Error;
use crate::compiler::message::Message;
use crate::compiler::r#enum::Enum;
use crate::compiler::structure::Structure;
use crate::compiler::union::Union;
use crate::model::protocol::Endianness;
use bp3d_debug::trace;
use std::rc::Rc;
use crate::compiler::util::imports::{ImportSolver, ProtocolStore};
use crate::compiler::util::store::ObjectStore;
use crate::compiler::util::types::{Name, PtrKey, TypePathMap};

#[derive(Clone, Debug)]
pub struct Protocol {
    pub full_name: String,
    pub endianness: Endianness,
    pub type_path_map: TypePathMap,
    pub structs: ObjectStore<Structure>,
    pub messages: ObjectStore<Message>,
    pub enums: ObjectStore<Enum>,
    pub unions: ObjectStore<Union>,
}

impl bp3d_util::index_map::Index for Protocol {
    type Key = str;

    fn index(&self) -> &Self::Key {
        &self.full_name
    }
}

#[derive(Copy, Clone)]
enum Import<'a> {
    Struct(&'a Rc<Structure>),
    Enum(&'a Rc<Enum>),
    Union(&'a Rc<Union>),
    Message(&'a Rc<Message>),
}

impl<'a> Name for Import<'a> {
    fn name(&self) -> &str {
        match self {
            Import::Struct(v) => v.name(),
            Import::Enum(v) => v.name(),
            Import::Union(v) => v.name(),
            Import::Message(v) => v.name(),
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
        }
    }
}

impl Protocol {
    pub fn name(&self) -> &str {
        if let Some(id) = self.full_name.rfind("::") {
            &self.full_name[id + 2..]
        } else {
            &self.full_name
        }
    }

    pub fn package(&self) -> &str {
        if let Some(id) = self.full_name.rfind("::") {
            &self.full_name[..id]
        } else {
            ""
        }
    }

    pub fn from_model<T: ImportSolver>(value: crate::model::Protocol, protocols: &ProtocolStore<T>, package: &str) -> Result<Self, Error> {
        let full_name = if package.is_empty() {
            value.name
        } else {
            format!("{}::{}", package, value.name)
        };
        let mut proto = Protocol {
            full_name,
            endianness: value.endianness.unwrap_or(Endianness::Little),
            type_path_map: TypePathMap::new(),
            structs: ObjectStore::new(),
            messages: ObjectStore::new(),
            enums: ObjectStore::new(),
            unions: ObjectStore::new()
        };
        if let Some(mut imports) = value.imports {
            let mut solved_imports = Vec::new();
            while let Some(v) = imports.pop() {
                trace!({protocol=&*v.protocol} {type=&*v.type_name}, "Solving import");
                let r = protocols.get(&v.protocol);
                let r = match r {
                    Some(r) => r,
                    None => return Err(Error::UndefinedReference(v.protocol)),
                };
                let ty = r
                    .structs
                    .get(&v.type_name)
                    .map(Import::Struct)
                    .or_else(|| r.messages.get(&v.type_name).map(Import::Message))
                    .or_else(|| r.unions.get(&v.type_name).map(Import::Union))
                    .or_else(|| r.enums.get(&v.type_name).map(Import::Enum))
                    .ok_or(Error::UnresolvedImport(format!("{}::{}", v.protocol, v.type_name)))?;
                let type_path = protocols.get_full_type_path(r, &v.type_name).ok_or(Error::SolverError)?;
                solved_imports.push(ty);
                let count1 = imports.iter().filter(|vv| vv.type_name == v.type_name).count();
                let count2 = solved_imports.iter().filter(|vv| vv.name() == v.type_name).count();
                let is_ambiguous = count1 > 0 || count2 > 1;
                proto.type_path_map.add(&ty, type_path);
                if is_ambiguous {
                    trace!({protocol=&*v.protocol} {type=&*v.type_name}, "Import is ambiguous, import it as {}::{}", v.protocol, v.type_name);
                    ty.insert(format!("{}::{}", v.protocol, v.type_name), &mut proto);
                } else {
                    trace!({protocol=&*v.protocol} {type=&*v.type_name}, "Import is not ambiguous");
                    ty.insert(v.type_name, &mut proto);
                }
            }
        }
        if let Some(enums) = value.enums {
            for v in enums {
                trace!({model=?&v}, "Compiling enum");
                let v = Rc::new(Enum::from_model(v)?);
                proto.enums.insert(v);
            }
        }
        if let Some(structs) = value.structs {
            for v in structs {
                trace!({model=?&v}, "Compiling structure");
                let v = Rc::new(Structure::from_model(&proto, v)?);
                proto.structs.insert(v);
            }
        }
        if let Some(unions) = value.unions {
            for v in unions {
                trace!({model=?&v}, "Compiling union");
                let v = Rc::new(Union::from_model(&proto, v)?);
                proto.unions.insert(v);
            }
        }
        if let Some(messages) = value.messages {
            for v in messages {
                trace!({model=?&v}, "Compiling message");
                let v = Rc::new(Message::from_model(&proto, v)?);
                proto.messages.insert(v);
            }
        }
        Ok(proto)
    }
}
