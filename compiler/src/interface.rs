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

use crate::compiler::util::ImportResolver;
use crate::gen::{FileType, Generator};
use crate::{compiler, model, Error};
use std::path::{Path, PathBuf};
use bp3d_util::path::PathExt;

pub trait ImportSolver {
    fn register(&mut self, base_import_path: String, protocol: compiler::Protocol);
}

pub struct Loader {
    models: Vec<model::Protocol>,
    imported_models: Vec<(String, model::Protocol)>,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            models: Vec::new(),
            imported_models: Vec::new(),
        }
    }

    pub fn load(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        let content = std::fs::read_to_string(path).map_err(Error::Io)?;
        let model: model::Protocol = json5::from_str(&content).map_err(Error::Model)?;
        self.models.push(model);
        Ok(())
    }

    pub fn import(
        &mut self,
        path: impl AsRef<Path>,
        import_path: impl Into<String>,
    ) -> Result<(), Error> {
        let content = std::fs::read_to_string(path).map_err(Error::Io)?;
        let model: model::Protocol = json5::from_str(&content).map_err(Error::Model)?;
        self.imported_models.push((import_path.into(), model));
        Ok(())
    }

    pub fn compile<'a, T: ImportResolver + ImportSolver>(self, solver: &mut T) -> Result<Protoc<'a>, Error> {
        for (base_import_path, model) in self.imported_models {
            let compiled =
                compiler::Protocol::from_model(model, solver).map_err(Error::Compiler)?;
            solver.register(base_import_path, compiled);
        }
        let models = self
            .models
            .into_iter()
            .map(|model| compiler::Protocol::from_model(model, solver))
            .collect::<Result<Vec<compiler::Protocol>, compiler::Error>>()
            .map_err(Error::Compiler)?;
        Ok(Protoc::new(models))
    }
}

pub struct Proto {
    pub name: String,
    pub path: PathBuf,
}

pub struct Protoc<'a> {
    protocols: Vec<compiler::Protocol>,
    file_header: Option<&'a Path>,
    write_messages: bool,
    read_messages: bool,
    use_enums: bool,
    use_structs: bool,
    use_messages: bool,
    use_unions: bool,
}

impl<'a> Protoc<'a> {
    pub fn new(protocols: Vec<compiler::Protocol>) -> Self {
        Self {
            protocols,
            write_messages: false,
            read_messages: false,
            use_enums: true,
            use_structs: true,
            use_messages: true,
            use_unions: true,
            file_header: None
        }
    }

    pub fn set_reads_messages(mut self, flag: bool) -> Self {
        self.read_messages = flag;
        self
    }

    pub fn set_writes_messages(mut self, flag: bool) -> Self {
        self.write_messages = flag;
        self
    }

    pub fn set_use_enums(mut self, flag: bool) -> Self {
        self.use_enums = flag;
        self
    }

    pub fn set_use_structs(mut self, flag: bool) -> Self {
        self.use_structs = flag;
        self
    }

    pub fn set_use_messages(mut self, flag: bool) -> Self {
        self.use_messages = flag;
        self
    }

    pub fn set_use_unions(mut self, flag: bool) -> Self {
        self.use_unions = flag;
        self
    }

    pub fn set_file_header(mut self, path: &'a Path) -> Self {
        self.file_header = Some(path);
        self
    }

    pub fn generate<T: Generator>(
        self,
        out_directory: impl AsRef<Path>,
    ) -> Result<Vec<Proto>, Error> {
        let file_header = self.file_header
            .map(|v| std::fs::read_to_string(v))
            .transpose()
            .map_err(Error::Io)?
            .map(|v| T::generate_file_header(v.lines()));
        let mut generated_protocols = Vec::new();
        for proto in self.protocols {
            let name = proto.name.clone();
            let files = T::generate(proto).map_err(|e| Error::Generator(e.to_string()))?;
            let out_path = out_directory.as_ref().join(&name);
            if !out_path.exists() {
                std::fs::create_dir(&out_path).map_err(Error::Io)?;
            }
            let files_iter = files.into_iter().filter(|v| match v.ty() {
                FileType::MessageWriting => self.write_messages,
                FileType::MessageReading => self.read_messages,
                FileType::Message => self.use_messages,
                FileType::Structure => self.use_structs,
                FileType::Enum => self.use_enums,
                FileType::Union => self.use_unions,
            });
            let iter = files_iter
                .into_iter()
                .map(|v| v.write(&out_path, file_header.as_deref(), T::get_language_extension()))
                .filter_map(|v| match v {
                    Ok(o) => o.map(Ok),
                    Err(e) => Some(Err(e)),
                })
                .collect::<std::io::Result<Vec<PathBuf>>>()
                .map_err(Error::Io)?;
            let umbrella = T::generate_umbrella(&name, iter.iter().map(|v| &**v))
                .map_err(|e| Error::Generator(e.to_string()))?;
            let proto_path = if umbrella.len() > 1 {
                let umbrella_path = out_path.join("umbrella").ensure_extension(T::get_language_extension()).to_path_buf();
                std::fs::write(&umbrella_path, umbrella).map_err(Error::Io)?;
                umbrella_path
            } else {
                out_path
            };
            generated_protocols.push(Proto {
                name,
                path: proto_path,
            })
        }
        Ok(generated_protocols)
    }
}
