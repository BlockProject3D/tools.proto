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
use crate::compiler::util::{ImportResolver, Name, PtrKey, TypePathMap};
use crate::model::protocol::Endianness;
use std::collections::HashMap;
use std::rc::Rc;
use bp3d_debug::trace;

#[derive(Clone, Debug)]
pub struct Protocol {
    pub name: String,
    pub endianness: Endianness,
    pub type_path_map: TypePathMap,
    pub structs_by_name: HashMap<String, Rc<Structure>>,
    pub messages_by_name: HashMap<String, Rc<Message>>,
    pub enums_by_name: HashMap<String, Rc<Enum>>,
    pub unions_by_name: HashMap<String, Rc<Union>>,
    pub structs: Vec<Rc<Structure>>,
    pub messages: Vec<Rc<Message>>,
    pub enums: Vec<Rc<Enum>>,
    pub unions: Vec<Rc<Union>>,
}

#[derive(Copy, Clone)]
enum Import<'a> {
    Struct(&'a Rc<Structure>),
    Enum(&'a Rc<Enum>),
    Union(&'a Rc<Union>),
    Message(&'a Rc<Message>)
}

impl<'a> Name for Import<'a> {
    fn name(&self) -> &str {
        match self {
            Import::Struct(v) => v.name(),
            Import::Enum(v) => v.name(),
            Import::Union(v) => v.name(),
            Import::Message(v) => v.name()
        }
    }
}

impl<'a> PtrKey for Import<'a> {
    fn ptr_key(&self) -> usize {
        match self {
            Import::Struct(v) => v.ptr_key(),
            Import::Enum(v) => v.ptr_key(),
            Import::Union(v) => v.ptr_key(),
            Import::Message(v) => v.ptr_key()
        }
    }
}

impl<'a> Import<'a> {
    pub fn insert(self, type_name: String, proto: &mut Protocol) {
        match self {
            Import::Struct(v) => { proto.structs_by_name.insert(type_name, v.clone()); },
            Import::Enum(v) => { proto.enums_by_name.insert(type_name, v.clone()); },
            Import::Union(v) => { proto.unions_by_name.insert(type_name, v.clone()); },
            Import::Message(v) => { proto.messages_by_name.insert(type_name, v.clone()); }
        }
    }
}

impl Protocol {
    pub fn from_model<T: ImportResolver>(value: crate::model::Protocol, solver: &T) -> Result<Self, Error> {
        let mut proto = Protocol {
            name: value.name,
            endianness: value.endianness.unwrap_or(Endianness::Little),
            type_path_map: TypePathMap::new(),
            structs_by_name: HashMap::new(),
            messages_by_name: HashMap::new(),
            enums_by_name: HashMap::new(),
            unions_by_name: HashMap::new(),
            structs: Vec::new(),
            messages: Vec::new(),
            enums: Vec::new(),
            unions: Vec::new(),
        };
        if let Some(mut imports) = value.imports {
            let mut solved_imports = Vec::new();
            while let Some(v) = imports.pop() {
                trace!({protocol=&*v.protocol} {type=&*v.type_name}, "Solving import");
                let r = solver.get_protocol_by_name(&v.protocol);
                let r = match r {
                    Some(r) => r,
                    None => return Err(Error::UndefinedReference(v.protocol)),
                };
                let ty = r.structs_by_name.get(&v.type_name).map(Import::Struct)
                    .or_else(|| r.messages_by_name.get(&v.type_name).map(Import::Message))
                    .or_else(|| r.unions_by_name.get(&v.type_name).map(Import::Union))
                    .or_else(|| r.enums_by_name.get(&v.type_name).map(Import::Enum))
                    .ok_or(Error::UnresolvedImport(format!("{}::{}", v.protocol, v.type_name)))?;
                let type_path = solver.get_full_type_path(&v.protocol, &v.type_name)
                    .ok_or(Error::SolverError)?;
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
                proto.enums_by_name.insert(v.name.clone(), v.clone());
                proto.enums.push(v);
            }
        }
        if let Some(structs) = value.structs {
            for v in structs {
                trace!({model=?&v}, "Compiling structure");
                let v = Rc::new(Structure::from_model(&proto, v)?);
                proto.structs_by_name.insert(v.name.clone(), v.clone());
                proto.structs.push(v);
            }
        }
        if let Some(unions) = value.unions {
            for v in unions {
                trace!({model=?&v}, "Compiling union");
                let v = Rc::new(Union::from_model(&proto, v)?);
                proto.unions_by_name.insert(v.name.clone(), v.clone());
                proto.unions.push(v);
            }
        }
        if let Some(messages) = value.messages {
            for v in messages {
                trace!({model=?&v}, "Compiling message");
                let v = Rc::new(Message::from_model(&proto, v)?);
                proto.messages_by_name.insert(v.name.clone(), v.clone());
                proto.messages.push(v);
            }
        }
        Ok(proto)
    }
}
