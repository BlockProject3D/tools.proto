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

use crate::api::core::Error;
use crate::compiler::util::imports::ImportSolver;
use crate::{compiler, model};
use bp3d_debug::{error, trace};
use std::borrow::Cow;
use std::path::Path;
use crate::compiler::util::protocols::{Entry, ProtocolStore};
use crate::model::protocol::Import;

#[derive(Debug, Clone)]
pub struct Options<'a> {
    package: &'a str,
    exclude_from_generation: bool
}

impl Default for Options<'_> {
    fn default() -> Self {
        Self {
            package: "",
            exclude_from_generation: false
        }
    }
}

impl<'a> Options<'a> {
    pub fn from_package(package: &'a str) -> Self {
        Self {
            package,
            exclude_from_generation: false
        }
    }

    pub fn exclude_from_generation(&mut self) -> &mut Self {
        self.exclude_from_generation = true;
        self
    }

    pub fn is_excluded_from_generation(&self) -> bool {
        self.exclude_from_generation
    }
}

pub struct Loader<'a> {
    models: Vec<Entry<model::Protocol, Options<'a>>>,
    max_iterations: usize,
}

impl Default for Loader<'_> {
    fn default() -> Self {
        Self::new(16)
    }
}

impl<'a> Loader<'a> {
    pub fn new(max_iterations: usize) -> Self {
        Self {
            models: Vec::new(),
            max_iterations,
        }
    }

    pub fn load_from_folder(&mut self, path: impl AsRef<Path>, options: &Options<'a>) -> Result<(), Error> {
        trace!({path=?path.as_ref()} {?options}, "Loading folder");
        for a in std::fs::read_dir(path).map_err(Error::Io)? {
            let file = a.map_err(Error::Io)?;
            if file.file_name().as_encoded_bytes().ends_with(b".json5") {
                self.load_from_file(file.path(), options)?
            }
        }
        Ok(())
    }

    pub fn load_from_file(&mut self, path: impl AsRef<Path>, options: &Options<'a>) -> Result<(), Error> {
        trace!({path=?path.as_ref()} {?options}, "Loading file");
        let content = std::fs::read_to_string(path).map_err(Error::Io)?;
        self.load_from_string(content, options)
    }

    pub fn load_from_string(&mut self, content: impl AsRef<str>, options: &Options<'a>) -> Result<(), Error> {
        trace!({content=content.as_ref()} {?options}, "Loading string");
        let model: model::Protocol = json5::from_str(content.as_ref()).map_err(Error::Model)?;
        if model.imports.as_ref().map(|v| v.len()).unwrap_or_default() > 0 {
            self.models.insert(0, Entry { userdata: options.clone(), model });
        } else {
            self.models.push(Entry { userdata: options.clone(), model });
        }
        Ok(())
    }

    pub fn exclude(&mut self, name: &str) {
        self.models.retain(|entry| entry.model.name != name);
    }

    pub fn compile<T: ImportSolver>(mut self, solver: &T) -> Result<ProtocolStore<T, Options<'a>>, Error> {
        let mut protocols = ProtocolStore::new(solver);
        let mut iterations = self.max_iterations;
        while !self.models.is_empty() && iterations > 0 {
            let entry = self.models.pop().unwrap();
            trace!({imports=?entry.model.imports} {iterations=?iterations}, "Solving imports for model {}", entry.model.name);
            let check_not_exists = |package: &str, import: &Import| {
                let full_name = if package.is_empty() || import.protocol.contains("::") {
                    Cow::Borrowed(&import.protocol)
                } else {
                    Cow::Owned(format!("{}::{}", package, import.protocol))
                };
                trace!("Searching for: {}", full_name);
                protocols.get(&full_name).is_none()
            };
            if entry.model
                .imports
                .as_ref()
                .map(|v| v.iter().any(|v| check_not_exists(entry.userdata.package, v)))
                .unwrap_or_default()
            {
                self.models.insert(0, entry);
                iterations -= 1;
                continue;
            }
            let proto = compiler::Protocol::from_model(entry.model, &protocols, entry.userdata.package).map_err(Error::Compiler)?;
            protocols.insert(Entry {
                model: proto,
                userdata: entry.userdata
            });
        }
        if iterations == 0 && !self.models.is_empty() {
            error!(
                "Failed to solve protocol import order in {} iterations, {} model(s) could not be solved...",
                self.max_iterations,
                self.models.len()
            );
            return Err(Error::SolverMaxIterations);
        }
        Ok(protocols)
    }
}
