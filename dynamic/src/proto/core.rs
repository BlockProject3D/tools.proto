// Copyright (c) 2025, BlockProject 3D
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
use bp3d_protoc::api::core::loader::Loader;
use bp3d_protoc::compiler::message::Message;
use bp3d_protoc::compiler::Protocol;
use bp3d_protoc::compiler::r#enum::Enum;
use bp3d_protoc::compiler::structure::Structure;
use bp3d_protoc::compiler::union::Union;
use bp3d_protoc::compiler::util::imports::ImportSolver;

struct Solver;

impl ImportSolver for Solver {
    fn get_full_type_path(&self, protocol: &Protocol, type_name: &str) -> Option<String> {
        let package = protocol.package();
        if package.is_empty() {
            Some(format!("{}.{}.{}", package, protocol.name(), type_name))
        } else {
            Some(format!("{}.{}", protocol.name(), type_name))
        }
    }
}

pub struct Proto {
    structures: HashMap<String, Rc<Structure>>,
    messages: HashMap<String, Rc<Message>>,
    unions: HashMap<String, Rc<Union>>,
    enums: HashMap<String, Rc<Enum>>,
}

impl Proto {
    pub fn build(loader: Loader) -> Result<Self, bp3d_protoc::api::core::Error> {
        let mut proto = Proto {
            structures: Default::default(),
            messages: Default::default(),
            unions: Default::default(),
            enums: Default::default(),
        };
        let store = loader.compile(&Solver)?;
        for entry in store.entries() {
            if entry.userdata.is_excluded_from_generation() {
                // Do not build excluded protocols
                continue;
            }
            for value in entry.model.structs.iter() {
                proto.structures.insert(store.get_full_type_path(&entry.model, &value.name).unwrap(), value.clone());
            }
            for value in entry.model.enums.iter() {
                proto.enums.insert(store.get_full_type_path(&entry.model, &value.name).unwrap(), value.clone());
            }
            for value in entry.model.unions.iter() {
                proto.unions.insert(store.get_full_type_path(&entry.model, &value.name).unwrap(), value.clone());
            }
            for value in entry.model.messages.iter() {
                proto.messages.insert(store.get_full_type_path(&entry.model, &value.name).unwrap(), value.clone());
            }
        }
        Ok(proto)
    }

    pub fn get_structure(&self, name: &str) -> Option<&Rc<Structure>> {
        self.structures.get(name)
    }
}
