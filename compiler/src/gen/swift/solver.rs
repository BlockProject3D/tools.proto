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
use crate::gen::template::util::CaseConversion;
use crate::ImportSolver;
use itertools::Itertools;
use std::collections::HashMap;
use crate::compiler::util::imports::ImportResolver;

pub struct SwiftImportSolver {
    import_map: HashMap<String, (Option<String>, Protocol)>,
}

impl Default for SwiftImportSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl SwiftImportSolver {
    pub fn new() -> Self {
        Self {
            import_map: HashMap::new(),
        }
    }

    pub fn iter_imports(&self) -> impl Iterator<Item = &str> {
        self.import_map.values().filter_map(|(k, _)| k.as_deref())
    }

    pub fn gen_import_list(&self) -> String {
        self.iter_imports().map(|v| format!("import {};", v)).join("\n")
    }
}

impl ImportResolver for SwiftImportSolver {
    fn get_protocol_by_name(&self, name: &str) -> Option<&Protocol> {
        self.import_map.get(name).map(|(_, v)| v)
    }

    fn get_full_type_path(&self, protocol: &str, type_name: &str) -> Option<String> {
        if let Some(import_path) = self.import_map.get(protocol).map(|(k, _)| k)? {
            Some(format!("{}.{}{}", import_path, protocol.to_pascal_case(), type_name))
        } else {
            Some(format!("{}{}", protocol.to_pascal_case(), type_name))
        }
    }
}

impl ImportSolver for SwiftImportSolver {
    /// base_import_path is assumed to be the module name containing the protocol, Self if the
    /// module is not external (so not to be imported).
    fn register(&mut self, base_import_path: String, protocol: Protocol) {
        if base_import_path == "Self" {
            self.import_map.insert(protocol.name.clone(), (None, protocol));
        } else {
            self.import_map.insert(protocol.name.clone(), (Some(base_import_path), protocol));
        }
    }
}
