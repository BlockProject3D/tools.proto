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

use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use bp3d_util::index_map::IndexMap;
use bp3d_util::path::PathExt;
use crate::{compiler, Error};
use crate::compiler::util::imports::{ImportSolver, ProtocolStore};
use crate::gen::file::FileType;

pub struct Context<'a> {
    items: IndexMap<Item<'a>>
}

impl<'a> Context<'a> {
    fn new(size: usize) -> Self {
        Self {
            items: IndexMap::with_capacity(size)
        }
    }

    fn insert(&mut self, item: Item<'a>) {
        self.items.insert(item);
    }

    pub fn get(&self, full_name: &str) -> Option<&Item<'a>> {
        self.items.get(full_name)
    }

    pub fn iter(&self) -> impl Iterator<Item=&Item<'a>> {
        self.items.iter()
    }
}

pub struct Item<'a> {
    full_name: &'a str,
    pub name: &'a str,
    pub path: PathBuf
}

impl<'a> bp3d_util::index_map::Index for Item<'a> {
    type Key = str;

    fn index(&self) -> &Self::Key {
        self.full_name
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Params {
    pub write_messages: bool,
    pub read_messages: bool,
    pub use_enums: bool,
    pub use_structs: bool,
    pub use_messages: bool,
    pub use_unions: bool
}

impl Default for Params {
    fn default() -> Self {
        Self {
            write_messages: true,
            read_messages: true,
            use_enums: true,
            use_structs: true,
            use_messages: true,
            use_unions: true,
        }
    }
}

pub struct Generator<'a, T, G> {
    protocols: ProtocolStore<'a, T>,
    generator: PhantomData<G>,
    out_directory: &'a Path,
    file_header: Option<&'a Path>,
}

impl<'a, T: ImportSolver, G: crate::gen::Generator> Generator<'a, T, G> {
    pub fn new<'b>(protocols: ProtocolStore<'a, T>, out_directory: &'a Path, _: G) -> (Context<'b>, Self) {
        (Context::new(protocols.len()), Self {
            protocols,
            out_directory,
            generator: PhantomData,
            file_header: None
        })
    }

    pub fn protocols(&self) -> &ProtocolStore<'a, T> {
        &self.protocols
    }

    pub fn set_file_header(&mut self, path: &'a Path) -> &mut Self {
        self.file_header = Some(path);
        self
    }

    fn generate_internal<'b>(&self, protocol: &'b compiler::Protocol, params: &Params, generator_params: &G::Params<'_>) -> Result<Item<'b>, Error> {
        let file_header = self.file_header
            .map(std::fs::read_to_string)
            .transpose()
            .map_err(Error::Io)?
            .map(|v| G::generate_file_header(v.lines()));
        let name = protocol.name();
        let files = G::generate(protocol, &generator_params).map_err(|e| Error::Generator(e.to_string()))?;
        let out_path = self.out_directory.join(&name);
        if !out_path.exists() {
            std::fs::create_dir(&out_path).map_err(Error::Io)?;
        }
        let files_iter = files.into_iter().filter(|v| match v.ty() {
            FileType::MessageWriting => params.write_messages,
            FileType::MessageReading => params.read_messages,
            FileType::Message => params.use_messages,
            FileType::Structure => params.use_structs,
            FileType::Enum => params.use_enums,
            FileType::Union => params.use_unions,
        });
        let iter = files_iter.into_iter()
            .map(|v| v.write(&out_path, file_header.as_deref(), G::get_language_extension()))
            .filter_map(|v| match v {
                Ok(o) => o.map(Ok),
                Err(e) => Some(Err(e)),
            })
            .collect::<std::io::Result<Vec<PathBuf>>>()
            .map_err(Error::Io)?;
        let umbrella = G::generate_umbrella(name, iter.iter().map(|v| &**v), &generator_params)
            .map_err(|e| Error::Generator(e.to_string()))?;
        let proto_path = if umbrella.len() > 1 {
            let umbrella_path = out_path.join("umbrella").ensure_extension(G::get_language_extension()).to_path_buf();
            std::fs::write(&umbrella_path, umbrella).map_err(Error::Io)?;
            umbrella_path
        } else {
            out_path
        };
        Ok(Item {
            full_name: &protocol.full_name,
            name,
            path: proto_path
        })
    }

    pub fn generate<'b>(&'b self, context: &mut Context<'b>, full_name: impl AsRef<str>, params: &Params, generator_params: &G::Params<'_>) -> Result<(), Error> {
        if context.get(full_name.as_ref()).is_none() {
            let protocol = self.protocols.get(full_name.as_ref()).ok_or_else(|| Error::ProtocolNotFound(full_name.as_ref().into()))?;
            context.insert(self.generate_internal(protocol, params, generator_params)?);
        }
        Ok(())
    }

    pub fn generate_all<'b>(&'b self, context: &mut Context<'b>, params: &Params, generator_params: &G::Params<'_>) -> Result<(), Error> {
        for protocol in self.protocols.iter() {
            if context.get(&protocol.full_name).is_none() {
                context.insert(self.generate_internal(protocol, params, generator_params)?);
            }
        }
        Ok(())
    }
}
