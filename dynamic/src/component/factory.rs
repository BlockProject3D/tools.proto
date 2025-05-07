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

use crate::component::ComponentType;
use bp3d_protoc::compiler::structure::FixedFieldType;
use bp3d_protoc::model::protocol::Endianness;
use bp3d_util::{simple_error, try_opt};
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

simple_error! {
    pub Error {
        AlreadyRegistered(String) => "factory for component '{}' is already registered",
        NotFound(String) => "component name '{}' not found",
        Fail(String) => "factory for component name '{}' has failed"
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SizeType {
    U8,
    U16LE,
    U16BE,
    U32LE,
    U32BE,
    U64LE,
    U64BE,
}

impl SizeType {
    pub fn get_endianness(&self) -> Endianness {
        match self {
            SizeType::U16LE | SizeType::U32LE | SizeType::U64LE => Endianness::Little,
            _ => Endianness::Big,
        }
    }

    pub fn get_size(&self) -> usize {
        match self {
            SizeType::U8 => 1,
            SizeType::U16LE | SizeType::U16BE => 2,
            SizeType::U32LE | SizeType::U32BE => 4,
            SizeType::U64LE | SizeType::U64BE => 8,
        }
    }
}

impl From<(FixedFieldType, Endianness)> for SizeType {
    fn from((value, endianness): (FixedFieldType, Endianness)) -> Self {
        match (value, endianness) {
            (FixedFieldType::Int8 | FixedFieldType::UInt8 | FixedFieldType::Bool, _) => SizeType::U8,
            (FixedFieldType::Int16 | FixedFieldType::UInt16, Endianness::Little) => SizeType::U16LE,
            (FixedFieldType::Int16 | FixedFieldType::UInt16, Endianness::Big) => SizeType::U16BE,
            (FixedFieldType::Int32 | FixedFieldType::UInt32 | FixedFieldType::Float32, Endianness::Little) => {
                SizeType::U32LE
            }
            (FixedFieldType::Int32 | FixedFieldType::UInt32 | FixedFieldType::Float32, Endianness::Big) => {
                SizeType::U32BE
            }
            (FixedFieldType::Int64 | FixedFieldType::UInt64 | FixedFieldType::Float64, Endianness::Little) => {
                SizeType::U64LE
            }
            (FixedFieldType::Int64 | FixedFieldType::UInt64 | FixedFieldType::Float64, Endianness::Big) => {
                SizeType::U64BE
            }
        }
    }
}

pub struct ContainerOptions {
    pub inner_ty: Rc<dyn ComponentType>,
    pub count_ty: SizeType,
    pub size_ty: Option<SizeType>,
}

impl ContainerOptions {
    #[inline(always)]
    fn inner_ty_ptr(&self) -> usize {
        let key = self.inner_ty.key();
        if key != 0 {
            key
        } else {
            Rc::as_ptr(&self.inner_ty) as *const () as _
        }
    }
}

impl Eq for ContainerOptions {}

impl PartialEq for ContainerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.inner_ty_ptr() == other.inner_ty_ptr() && self.count_ty == other.count_ty && self.size_ty == other.size_ty
    }
}

impl Hash for ContainerOptions {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.inner_ty_ptr());
        self.count_ty.hash(state);
        self.size_ty.hash(state);
    }
}

#[derive(Eq, PartialEq, Hash)]
enum CompKeyInner {
    Container(ContainerOptions),
    Buffer(Option<SizeType>),
}

#[derive(Eq, PartialEq, Hash)]
pub struct Key {
    name: String,
    inner: CompKeyInner,
}

impl Key {
    pub fn for_container(name: impl Into<String>, options: ContainerOptions) -> Self {
        Self {
            name: name.into(),
            inner: CompKeyInner::Container(options),
        }
    }

    pub fn for_buffer(name: impl Into<String>, size: Option<SizeType>) -> Self {
        Self {
            name: name.into(),
            inner: CompKeyInner::Buffer(size),
        }
    }
}

pub struct Factory {
    container_factories: HashMap<String, Box<dyn Fn(&ContainerOptions) -> Option<Rc<dyn ComponentType>>>>,
    buffer_factories: HashMap<String, Box<dyn Fn(Option<SizeType>) -> Option<Rc<dyn ComponentType>>>>,
    components: RefCell<HashMap<Key, Rc<dyn ComponentType>>>,
}

impl Factory {
    pub fn new() -> Self {
        Self {
            container_factories: HashMap::new(),
            buffer_factories: HashMap::new(),
            components: RefCell::new(HashMap::new()),
        }
    }

    pub fn add_container_factory<F: Fn(&ContainerOptions) -> Option<Rc<dyn ComponentType>> + 'static>(
        &mut self,
        component_name: impl Into<String>,
        factory: F,
    ) -> Result<()> {
        let component_name = component_name.into();
        if self.container_factories.contains_key(&component_name) && self.buffer_factories.contains_key(&component_name)
        {
            return Err(Error::AlreadyRegistered(component_name));
        }
        self.container_factories.insert(component_name.clone(), Box::new(factory));
        Ok(())
    }

    pub fn add_buffer_factory<F: Fn(Option<SizeType>) -> Option<Rc<dyn ComponentType>> + 'static>(
        &mut self,
        component_name: impl Into<String>,
        factory: F,
    ) -> Result<()> {
        let component_name = component_name.into();
        if self.container_factories.contains_key(&component_name) && self.buffer_factories.contains_key(&component_name)
        {
            return Err(Error::AlreadyRegistered(component_name));
        }
        self.buffer_factories.insert(component_name.clone(), Box::new(factory));
        Ok(())
    }

    pub fn add_component(&mut self, key: Key, component: impl ComponentType + 'static) -> Result<()> {
        if self.components.get_mut().contains_key(&key) {
            return Err(Error::AlreadyRegistered(key.name));
        }
        self.components.get_mut().insert(key, Rc::new(component));
        Ok(())
    }

    pub fn with_component<R>(&self, key: Key, f: impl FnOnce(&Rc<dyn ComponentType>) -> R) -> Result<R> {
        let mut components = self.components.borrow_mut();
        if let Some(component) = components.get(&key) {
            return Ok(f(&*component));
        }
        let ct = match &key.inner {
            CompKeyInner::Container(opts) => self.container_factories.get(&key.name).map(|v| v(opts)),
            CompKeyInner::Buffer(opt) => self.buffer_factories.get(&key.name).map(|v| v(*opt)),
        };
        let ct = try_opt!(try_opt!(ct => Error::NotFound(key.name)) => Error::Fail(key.name));
        let component = components.entry(key).or_insert(ct);
        Ok(f(component))
    }

    pub fn get_component(&self, key: Key) -> Result<Rc<dyn ComponentType>> {
        self.with_component(key, |c| c.clone())
    }
}
