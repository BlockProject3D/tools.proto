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

use bp3d_protoc::gen::{GeneratorRust, GeneratorSwift, RustParams, SwiftImportSolver};
use bp3d_protoc::util::SimpleImportSolver;
use bp3d_protoc::{Loader, Protoc};
use bp3d_util::result::ResultExt;
use clap::{Parser, ValueEnum};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum Generator {
    /// The Rust code generator.
    Rust,

    /// The Swift code generator.
    Swift,
}

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum Solver {
    /// The simple import solver which is the default for Rust, should work in most cases.
    Simple,

    /// The swift import solver which simplifies dealing with the distinction between foreign and
    /// local imports.
    Swift,
}

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum Feature {
    /// The generated code should be able to write messages.
    WriteMessages,

    /// The generated code should be able to read messages.
    ReadMessages,

    /// The generated code needs enums.
    UseEnums,

    /// The generated code needs structures.
    UseStructs,

    /// The generated code needs messages.
    UseMessages,

    /// The generated code needs unions.
    UseUnions,
}

impl Feature {
    pub fn apply<'a>(&self, protoc: Protoc<'a>) -> Protoc<'a> {
        match self {
            Feature::WriteMessages => protoc.set_writes_messages(true),
            Feature::ReadMessages => protoc.set_reads_messages(true),
            Feature::UseEnums => protoc.set_use_enums(true),
            Feature::UseStructs => protoc.set_use_structs(true),
            Feature::UseMessages => protoc.set_use_messages(true),
            Feature::UseUnions => protoc.set_use_unions(true),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// CLI tool to call BP3D protocol compiler outside build-scripts and access to source code.
pub struct Args {
    /// List of imported protocols, each by pairs of path to the protocol description and import
    /// path.
    #[clap(short = 'i', long = "import", number_of_values = 2)]
    pub imports: Vec<OsString>,
    /// List of input protocol description files to compile.
    #[clap(required=true, num_args=1..)]
    pub inputs: Vec<PathBuf>,
    /// Output directory where to place the generated protocols (an additional directory is created
    /// for each protocol to be compiled).
    #[clap(short = 'o', long = "output")]
    pub output: Option<PathBuf>,
    /// Name of the code generator to use.
    #[clap(short = 'g', long = "generator", default_value = "rust")]
    pub generator: Generator,
    /// Name of the import solver to use.
    #[clap(short = 's', long = "solver", default_value = "simple")]
    pub solver: Solver,
    /// Features to enable, the default is to use the default set of features which includes
    /// everything except reading and writing messages.
    #[clap(short = 'f', long = "feature")]
    pub features: Option<Vec<Feature>>,
    /// The separator to add between the import path of protocol and the type (the default value is
    /// '::', as used by Rust and C++).
    #[clap(long = "separator", default_value = "::")]
    pub import_separator: String,
    /// The file header to include at the top of each generated file, each line in the file header
    /// is already formatted according to the line comments syntax of the chosen target generation
    /// language.
    #[clap(long = "header")]
    pub file_header: Option<PathBuf>,
}

impl Args {
    pub fn iter_imports(&self) -> impl Iterator<Item = (&Path, &str)> {
        self.imports.chunks(2).map(|v| (Path::new(&v[0]), v[1].to_str().unwrap()))
    }
}

fn main() {
    let args = Args::parse();
    let mut loader = Loader::new();
    for (import_file, import_path) in args.iter_imports() {
        loader.import(import_file, import_path).expect_exit("failed to import protocol", 1);
    }
    for input in args.inputs {
        loader.load(input).expect_exit("failed to load protocol", 1);
    }
    let mut swift_solver = SwiftImportSolver::new();
    let mut protoc = match args.solver {
        Solver::Simple => loader.compile(&mut SimpleImportSolver::new(&args.import_separator)),
        Solver::Swift => loader.compile(&mut swift_solver),
    }
    .expect_exit("failed to compile protocols", 1);
    if let Some(features) = args.features {
        for f in features {
            protoc = f.apply(protoc);
        }
    }
    if let Some(file_header) = &args.file_header {
        protoc = protoc.set_file_header(file_header);
    }
    match args.generator {
        Generator::Rust => protoc.generate::<GeneratorRust>(args.output.unwrap_or(PathBuf::from("./")), RustParams::default()),
        Generator::Swift => protoc.generate::<GeneratorSwift>(args.output.unwrap_or(PathBuf::from("./")), swift_solver),
    }
    .expect_exit("failed to generate protocols", 1);
}
