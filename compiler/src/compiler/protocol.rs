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
use crate::compiler::message::{FieldType, Message};
use crate::compiler::r#enum::Enum;
use crate::compiler::structure::Structure;
use crate::compiler::union::Union;
use crate::compiler::util::imports::{ImportSolver, ProtocolStore};
use crate::compiler::util::store::ObjectStore;
use crate::compiler::util::types::{Name, TypePathMap};
use crate::model::protocol::{Description, Endianness};
use bp3d_debug::{info, trace};
use std::borrow::Cow;
use std::rc::Rc;
use crate::compiler::imports::Import;
use crate::model::typedef::Typedef;

impl Name for Typedef {
    fn name(&self) -> &str {
        match self {
            Typedef::Message(v) => &v.name,
            Typedef::Structure(v) => &v.name
        }
    }
}

impl bp3d_util::index_map::Index for Typedef {
    type Key = str;

    fn index(&self) -> &Self::Key {
        match self {
            Typedef::Message(v) => &v.name,
            Typedef::Structure(v) => &v.name
        }
    }
}

#[derive(Clone, Debug)]
pub struct Protocol {
    pub full_name: String,
    pub description: Option<Description>,
    pub endianness: Endianness,
    pub type_path_map: TypePathMap,
    pub structs: ObjectStore<Structure>,
    pub messages: ObjectStore<Message>,
    pub enums: ObjectStore<Enum>,
    pub unions: ObjectStore<Union>,
    pub types: ObjectStore<Typedef>
}

impl bp3d_util::index_map::Index for Protocol {
    type Key = str;

    fn index(&self) -> &Self::Key {
        &self.full_name
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

    pub fn from_model<T: ImportSolver>(
        mut value: crate::model::Protocol,
        protocols: &ProtocolStore<T>,
        package: &str,
    ) -> Result<Self, Error> {
        let full_name = if package.is_empty() {
            value.name
        } else {
            format!("{}::{}", package, value.name)
        };
        let mut proto = Protocol {
            full_name,
            description: value.description,
            endianness: value.endianness.unwrap_or(Endianness::Little),
            type_path_map: TypePathMap::new(),
            structs: ObjectStore::new(),
            messages: ObjectStore::new(),
            enums: ObjectStore::new(),
            unions: ObjectStore::new(),
            types: ObjectStore::new()
        };
        info!("Running import solver pass...");
        if let Some(mut imports) = value.imports {
            let mut solved_imports = Vec::new();
            while let Some(v) = imports.pop() {
                let protocol_path = if package.is_empty() {
                    Cow::Borrowed(&v.protocol)
                } else {
                    Cow::Owned(format!("{}::{}", package, v.protocol))
                };
                trace!({protocol=&**protocol_path} {type=&*v.type_name}, "Solving import");
                let r = protocols.get(&protocol_path);
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
                    .or_else(|| r.types.get(&v.type_name).map(Import::Type))
                    .ok_or(Error::UnresolvedImport(format!("{}::{}", protocol_path, v.type_name)))?;
                let type_path = protocols.get_full_type_path(r, &v.type_name).ok_or(Error::SolverError)?;
                solved_imports.push(ty);
                let count1 = imports.iter().filter(|vv| vv.type_name == v.type_name).count();
                let count2 = solved_imports.iter().filter(|vv| vv.name() == v.type_name).count();
                let is_ambiguous = count1 > 0 || count2 > 1;
                proto.type_path_map.add(&ty, type_path);
                if is_ambiguous {
                    trace!({protocol=&**protocol_path} {type=&*v.type_name}, "Import is ambiguous, import it as {}::{}", v.protocol, v.type_name);
                    ty.insert(format!("{}::{}", v.protocol, v.type_name), &mut proto);
                } else {
                    trace!({protocol=&**protocol_path} {type=&*v.type_name}, "Import is not ambiguous");
                    ty.insert(v.type_name, &mut proto);
                }
            }
        }
        info!("Adding typedefs...");
        if let Some(types) = value.types {
            for v in types {
                trace!({model=?&v}, "Adding typedef to protocol");
                proto.types.insert(Rc::new(v));
            }
        }
        info!("Running type inference pass...");
        if let Some(structs) = &mut value.structs {
            for v in structs {
                for field in &mut v.fields {
                    if let Some(info) = field.item_type.as_ref().map(|v| proto.types.get(v)).flatten().map(|v| v.as_struct()).flatten() {
                        trace!({typedef=?info}, "Inferred {} as {}", field.name, info.name);
                        let name = std::mem::replace(field, info.clone()).name;
                        field.name = name;
                    }
                }
            }
        }
        if let Some(messages) = &mut value.messages {
            for v in messages {
                for field in &mut v.fields {
                    if let Some(info) = field.item_type.as_ref().map(|v| proto.types.get(v)).flatten().map(|v| v.as_message()).flatten() {
                        trace!({typedef=?info}, "Inferred {} as {}", field.name, info.name);
                        let name = std::mem::replace(field, info.clone()).name;
                        field.name = name;
                    }
                }
            }
        }
        info!("Running compiler pass...");
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
        info!("Running list sanitizer pass...");
        for msg in proto.messages.iter() {
            if msg.is_embedded() {
                for field in &msg.fields {
                    let flag = match &field.ty {
                        FieldType::List(v) => v.nested,
                        _ => true
                    };
                    if !flag {
                        return Err(Error::MissingNestedList(format!("{}::{}", msg.name, field.name)));
                    }
                }
            }
        }
        Ok(proto)
    }
}
