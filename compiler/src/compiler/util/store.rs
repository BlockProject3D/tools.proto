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

use bp3d_util::index_map::IndexMap;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct ObjectStore<T> {
    objects: Vec<Rc<T>>,
    objects_by_name: IndexMap<Rc<T>>,
    objects_imports: HashMap<String, Rc<T>>,
}

impl<T: bp3d_util::index_map::Index<Key = str>> ObjectStore<T> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            objects_by_name: IndexMap::new(),
            objects_imports: HashMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Rc<T>> {
        self.objects.iter()
    }

    pub fn get(&self, name: &str) -> Option<&Rc<T>> {
        self.objects_by_name.get(name).or_else(|| self.objects_imports.get(name))
    }

    pub fn insert(&mut self, obj: Rc<T>) {
        self.objects_by_name.insert(obj.clone());
        self.objects.push(obj);
    }

    pub fn insert_import(&mut self, import_name: String, obj: Rc<T>) {
        self.objects_imports.insert(import_name, obj);
    }
}

macro_rules! name_index {
    ($t: ty => $key: ident) => {
        impl crate::compiler::util::types::Name for $t {
            fn name(&self) -> &str {
                &*self.$key
            }
        }

        impl bp3d_util::index_map::Index for $t {
            type Key = str;

            fn index(&self) -> &Self::Key {
                &*self.$key
            }
        }
    };
}

pub(crate) use name_index;
