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

use crate::gen::GeneratorRust;
use crate::util::SimpleImportSolver;
use crate::{Error, Loader, Protoc};

/// A simple function to quickly generate protocols in Rust for use with the Cargo build system.
///
/// # Arguments
///
/// * `load_fn`: a function which loads and imports all required protocols to compile and generate
/// Rust code for.
/// * `configure_fn`: a configuration function to configure the [Protoc](Protoc) for generating.
///
/// # Panics
///
/// This function panics in case the loader, compiler or generator failed and the protocol Rust code
/// could not be generated.
pub fn generate_rust<F: FnOnce(&mut Loader) -> Result<(), Error>, F1: FnOnce(Protoc) -> Protoc>(
    load_fn: F,
    configure_fn: F1,
) {
    let mut loader = Loader::new();
    let res = load_fn(&mut loader);
    if let Err(e) = res {
        panic!("Failed to load protocols: {}", e);
    }
    let protoc = match loader.compile(SimpleImportSolver::new("::")) {
        Err(e) => panic!("Failed to compile protocols: {}", e),
        Ok(v) => v,
    };
    let protoc = configure_fn(protoc);
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let generated = match protoc.generate::<GeneratorRust>(out_dir) {
        Err(e) => panic!("Failed to generate Rust code: {}", e),
        Ok(v) => v,
    };
    for proto in generated {
        println!(
            "cargo::rustc-env=BP3D_PROTOC_{}={}",
            proto.name.to_ascii_uppercase(),
            proto.path.display()
        );
    }
}
