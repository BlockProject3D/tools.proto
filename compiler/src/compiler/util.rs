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
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

pub trait ImportResolver {
    fn get_protocol_by_name(&self, name: &str) -> Option<&Protocol>;
    fn get_full_type_path(&self, protocol: &str, type_name: &str) -> Option<String>;
}

pub trait TypeMapper {
    fn map_local_type<'a>(&self, item_type: &'a str) -> Cow<'a, str>;
    fn map_foreign_type<'a>(&self, item_type: &'a str) -> Cow<'a, str>;
}

pub trait Name {
    fn name(&self) -> &str;
}

pub trait PtrKey {
    fn ptr_key(&self) -> usize;
}

impl<K> PtrKey for Rc<K> {
    fn ptr_key(&self) -> usize {
        &**self as *const K as usize
    }
}

impl<K: Name> Name for Rc<K> {
    fn name(&self) -> &str {
        self.deref().name()
    }
}

#[derive(Clone, Debug)]
pub struct TypePathMap {
    type_path_by_addr: HashMap<usize, String>,
}

impl Default for TypePathMap {
    fn default() -> Self {
        Self::new()
    }
}

impl TypePathMap {
    pub fn new() -> Self {
        Self {
            type_path_by_addr: HashMap::new(),
        }
    }

    pub fn add<K: PtrKey>(&mut self, key: &K, type_path: String) {
        self.type_path_by_addr.insert(key.ptr_key(), type_path);
    }

    pub fn get<'a, K: PtrKey + Name, T: TypeMapper>(&'a self, mapper: &T, item_type: &'a K) -> Cow<'a, str> {
        match self.type_path_by_addr.get(&item_type.ptr_key()) {
            None => mapper.map_local_type(item_type.name()),
            Some(v) => mapper.map_foreign_type(v),
        }
    }

    pub fn get_with_default_prefix<'a, K: PtrKey + Name>(
        &'a self,
        item_type: &'a K,
        default_prefix: &str,
    ) -> Cow<'a, str> {
        match self.type_path_by_addr.get(&item_type.ptr_key()) {
            None => Cow::Owned(format!("{default_prefix}{}", item_type.name())),
            Some(v) => Cow::Borrowed(v),
        }
    }
}

impl ImportResolver for () {
    fn get_protocol_by_name(&self, _: &str) -> Option<&Protocol> {
        None
    }

    fn get_full_type_path(&self, _: &str, _: &str) -> Option<String> {
        None
    }
}

macro_rules! try2 {
    ($value: expr => $err: expr) => {
        match $value {
            Some(v) => v,
            None => return Err($err),
        }
    };
}

pub(crate) use try2;
