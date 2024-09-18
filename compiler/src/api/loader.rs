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

use std::path::Path;
use bp3d_debug::{error, trace};
use crate::{compiler, model, Error};
use crate::compiler::util::imports::{ImportSolver, ProtocolStore};

pub struct Loader<'a> {
    models: Vec<(&'a str, model::Protocol)>,
    max_iterations: usize
}

impl<'a> Default for Loader<'a> {
    fn default() -> Self {
        Self::new(16)
    }
}

impl<'a> Loader<'a> {
    pub fn new(max_iterations: usize) -> Self {
        Self {
            models: Vec::new(),
            max_iterations
        }
    }

    pub fn load_from_folder(&mut self, path: impl AsRef<Path>, package: &'a str) -> Result<(), Error> {
        for a in std::fs::read_dir(path).map_err(Error::Io)? {
            let file = a.map_err(Error::Io)?;
            if file.file_name().as_encoded_bytes().ends_with(b".json5") {
                self.load_from_file(&file.path(), package)?
            }
        }
        Ok(())
    }

    pub fn load_from_file(&mut self, path: impl AsRef<Path>, package: &'a str) -> Result<(), Error> {
        let content = std::fs::read_to_string(path).map_err(Error::Io)?;
        let model: model::Protocol = json5::from_str(&content).map_err(Error::Model)?;
        if model.imports.as_ref().map(|v| v.len()).unwrap_or_default() > 0 {
            self.models.insert(0, (package, model));
        } else {
            self.models.push((package, model));
        }
        Ok(())
    }

    pub fn compile<T: ImportSolver>(mut self, solver: &T) -> Result<ProtocolStore<T>, Error> {
        let mut protocols = ProtocolStore::new(solver);
        let mut iterations = self.max_iterations;
        while !self.models.is_empty() && iterations > 0 {
            let (package, model) = self.models.pop().unwrap();
            trace!({imports=?model.imports}, "Solving imports for model {}", model.name);
            if model.imports.as_ref().map(|v| v.iter().any(|v| protocols.get(&v.protocol).is_none())).unwrap_or_default() {
                self.models.insert(0, (package, model));
                iterations -= 1;
                continue;
            }
            let proto = compiler::Protocol::from_model(model, &protocols, package).map_err(Error::Compiler)?;
            protocols.insert(proto);
        }
        if iterations == 0 && self.models.len() > 0 {
            error!("Failed to solve protocol import order in {} iterations, {} model(s) could not be solved...", self.max_iterations, self.models.len());
            return Err(Error::SolverMaxIterations);
        }
        Ok(protocols)
    }
}
