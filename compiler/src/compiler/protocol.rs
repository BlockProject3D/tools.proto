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

use std::collections::HashMap;
use std::rc::Rc;
use crate::compiler::error::Error;
use crate::compiler::message::Message;
use crate::compiler::r#enum::Enum;
use crate::compiler::structure::Structure;
use crate::compiler::util::{ImportResolver, TypePathMap};

#[derive(Clone, Debug)]
pub struct Protocol {
    pub name: String,
    pub type_path_by_name: TypePathMap,
    pub structs_by_name: HashMap<String, Rc<Structure>>,
    pub messages_by_name: HashMap<String, Rc<Message>>,
    pub enums_by_name: HashMap<String, Rc<Enum>>,
    pub structs: Vec<Rc<Structure>>,
    pub messages: Vec<Rc<Message>>,
    pub enums: Vec<Rc<Enum>>
}

impl Protocol {
    pub fn from_model<T: ImportResolver>(value: crate::model::Protocol, solver: &T) -> Result<Self, Error> {
        let mut proto = Protocol {
            name: value.name,
            type_path_by_name: TypePathMap::new(),
            structs_by_name: HashMap::new(),
            messages_by_name: HashMap::new(),
            enums_by_name: HashMap::new(),
            structs: Vec::new(),
            messages: Vec::new(),
            enums: Vec::new()
        };
        if let Some(imports) = value.imports {
            for v in imports {
                let r = solver.get_protocol_by_name(&v.protocol);
                let r = match r {
                    Some(r) => r,
                    None => return Err(Error::UndefinedReference(v.protocol))
                };
                match r.structs_by_name.get(&v.type_name) {
                    None => {
                        match r.enums_by_name.get(&v.type_name) {
                            None => {
                                let msg = r.messages_by_name.get(&v.type_name).ok_or(Error::UndefinedReference(format!("{}::{}", v.protocol, v.type_name)))?;
                                let type_path = solver.get_full_type_path(&v.protocol, &v.type_name).ok_or(Error::SolverError)?;
                                proto.messages_by_name.insert(v.type_name, msg.clone());
                                proto.type_path_by_name.add(msg.name.clone(), type_path);
                            },
                            Some(vv) => {
                                let type_path = solver.get_full_type_path(&v.protocol, &v.type_name).ok_or(Error::SolverError)?;
                                proto.enums_by_name.insert(v.type_name, vv.clone());
                                proto.type_path_by_name.add(vv.name.clone(), type_path);
                            }
                        }
                    },
                    Some(vv) => {
                        let type_path = solver.get_full_type_path(&v.protocol, &v.type_name).ok_or(Error::SolverError)?;
                        proto.structs_by_name.insert(v.type_name, vv.clone());
                        proto.type_path_by_name.add(vv.name.clone(), type_path);
                    }
                }
            }
        }
        if let Some(structs) = value.structs {
            for v in structs {
                let v = Rc::new(Structure::from_model(&proto, v)?);
                proto.structs_by_name.insert(v.name.clone(), v.clone());
                proto.structs.push(v);
            }
        }
        if let Some(messages) = value.messages {
            for v in messages {
                let v = Rc::new(Message::from_model(&proto, v)?);
                proto.messages_by_name.insert(v.name.clone(), v.clone());
                proto.messages.push(v);
            }
        }
        if let Some(enums) = value.enums {
            for v in enums {
                let v = Rc::new(Enum::from_model(v)?);
                proto.enums_by_name.insert(v.name.clone(), v.clone());
                proto.enums.push(v);
            }
        }
        Ok(proto)
    }
}
