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

use std::borrow::Cow;
use std::path::{Path, PathBuf};
use crate::compiler::Protocol;

mod rust;
pub mod template;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum FileType {
    MessageWriting,
    MessageReading,
    Message,
    Structure,
    Enum,
    Union
}

pub struct File {
    name: Cow<'static, str>,
    data: String,
    ty: FileType
}

impl File {
    pub fn new(ty: FileType, name: impl Into<Cow<'static, str>>, data: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ty,
            data: data.into()
        }
    }

    pub fn ty(&self) -> FileType {
        self.ty
    }

    pub fn write(self, out_directory: &Path) -> std::io::Result<Option<PathBuf>> {
        if self.data.len() > 1 {
            let sub_folder = self.name.find("/").map(|id| &self.name[..id]);
            if let Some(sub_folder) = sub_folder {
                std::fs::create_dir(out_directory.join(sub_folder))?;
            }
            let path = out_directory.join(&*self.name);
            std::fs::write(&path, self.data)?;
            Ok(Some(path))
        } else {
            Ok(None)
        }
    }
}

pub trait Generator {
    type Error: std::error::Error;

    fn generate(proto: Protocol) -> Result<Vec<File>, Self::Error>;
    fn generate_umbrella<'a>(_: &str, _: impl Iterator<Item=&'a Path>) -> Result<String, Self::Error> {
        Ok(String::new())
    }
}

pub use rust::GeneratorRust;
