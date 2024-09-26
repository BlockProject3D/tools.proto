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

mod args;
mod builder;

use crate::args::{Args, Generator};
use crate::builder::Builder;
use bp3d_protoc::api::core::loader::Loader;
use bp3d_protoc::gen::{GeneratorRust, GeneratorSwift, RustImportSolver, RustParams, SwiftImportSolver};
use bp3d_util::result::ResultExt;
use clap::Parser;

fn build_swift(loader: Loader, args: &Args) {
    let mut builder = Builder::new(loader, args, &SwiftImportSolver, GeneratorSwift);
    builder
        .generator
        .generate_all(&mut builder.context, &builder.params, builder.generator.protocols())
        .expect_exit("failed to generate protocols", 1);
}

fn build_rust(loader: Loader, args: &Args) {
    let mut builder = Builder::new(loader, args, &RustImportSolver, GeneratorRust);
    builder
        .generator
        .generate_all(&mut builder.context, &builder.params, &RustParams::default())
        .expect_exit("failed to generate protocols", 1);
}

fn main() {
    let args = Args::parse();
    let mut loader = Loader::default();
    for (import_file, import_path) in args.iter_imports() {
        loader.load_from_file(import_file, import_path).expect_exit("failed to import protocol", 1);
    }
    for input in &args.inputs {
        loader.load_from_file(input, "").expect_exit("failed to load protocol", 1);
    }
    match args.generator {
        Generator::Rust => build_rust(loader, &args),
        Generator::Swift => build_swift(loader, &args),
    };
}
