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

use std::path::{Path, PathBuf};
use bp3d_util::result::ResultExt;
use clap::{Parser, ValueEnum};
use bp3d_protoc::api::tools::GenTools;

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum Generator {
    /// The Rust code generator.
    Rust,

    /// The Swift code generator.
    Swift,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// CLI tool to call BP3D protocol compiler outside build-scripts and access to source code.
pub struct Args {
    /// Input protoc.toml config file.
    #[clap(short = 'i', long = "input")]
    pub input: PathBuf,
    /// Output directory where to place the generated protocols (an additional directory is created
    /// for each protocol to be compiled).
    #[clap(short = 'o', long = "output")]
    pub output: Option<PathBuf>,
    /// Name of the code generator to use.
    #[clap(short = 'g', long = "generator", default_value = "rust")]
    pub generator: Generator
}

fn main() {
    let args = Args::parse();
    let output = args.output.as_deref().unwrap_or(Path::new("./"));
    match args.generator {
        Generator::Rust => bp3d_protoc::api::tools::Rust::run_file(args.input, output, |_| {}),
        Generator::Swift => bp3d_protoc::api::tools::Swift::run_file(args.input, output, |_| {})
    }.expect_exit("Failed to run make tool", 1);
}
