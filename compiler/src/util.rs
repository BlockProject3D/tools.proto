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
use crate::compiler::Protocol;
use crate::ImportSolver;
use std::collections::HashMap;

pub struct SimpleImportSolver<'a> {
    import_map: HashMap<String, (String, Protocol)>,
    separator: &'a str,
}

impl<'a> ImportSolver for SimpleImportSolver<'a> {
    fn register(&mut self, base_import_path: String, protocol: Protocol) {
        self.import_map.insert(protocol.name.clone(), (base_import_path, protocol));
    }
}

impl<'a> SimpleImportSolver<'a> {
    pub fn new(separator: &'a str) -> Self {
        Self {
            import_map: HashMap::new(),
            separator,
        }
    }
}

impl<'a> Default for SimpleImportSolver<'a> {
    fn default() -> Self {
        Self::new("::")
    }
}

impl<'a> ImportResolver for SimpleImportSolver<'a> {
    fn get_protocol_by_name(&self, name: &str) -> Option<&Protocol> {
        self.import_map.get(name).map(|(_, v)| v)
    }

    fn get_full_type_path(&self, protocol: &str, type_name: &str) -> Option<String> {
        let import_path = self.import_map.get(protocol).map(|(k, _)| k)?;
        Some(format!("{}{}{}", import_path, self.separator, type_name))
    }
}
