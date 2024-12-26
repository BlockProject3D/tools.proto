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

use crate::compiler::util::imports::ImportSolver;
use crate::compiler::Protocol;
use bp3d_util::index_map::{Index, IndexMap};
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub struct Entry<M, U = ()> {
    pub model: M,
    pub userdata: U,
}

impl<U> Index for Entry<Protocol, U> {
    type Key = str;

    fn index(&self) -> &Self::Key {
        self.model.index()
    }
}

pub struct ProtocolStore<'a, T, U> {
    map: IndexMap<Entry<Protocol, U>>,
    solver: &'a T,
}

impl<'a, T: ImportSolver, U> ProtocolStore<'a, T, U> {
    pub fn new(solver: &'a T) -> Self {
        Self {
            map: IndexMap::new(),
            solver,
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn insert(&mut self, entry: Entry<Protocol, U>) {
        self.map.insert(entry)
    }

    pub fn get(&self, full_name: &str) -> Option<&Protocol> {
        self.map.get(full_name).map(|v| &v.model)
    }

    pub fn entry(&self, full_name: &str) -> Option<&Entry<Protocol, U>> {
        self.map.get(full_name)
    }

    pub fn get_full_type_path(&self, protocol: &Protocol, type_name: &str) -> Option<String> {
        self.solver.get_full_type_path(protocol, type_name)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Protocol> {
        self.map.iter().map(|v| &v.model)
    }

    pub fn entries(&self) -> impl Iterator<Item = &Entry<Protocol, U>> {
        self.map.iter()
    }
}

impl<T, U: Debug> Debug for ProtocolStore<'_, T, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("ProtocolStore { solver: ")?;
        f.write_str(std::any::type_name::<T>())?;
        f.write_str(", map: ")?;
        self.map.fmt(f)?;
        f.write_str(" }")
    }
}
