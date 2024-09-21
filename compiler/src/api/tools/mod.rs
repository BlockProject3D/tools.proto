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

#[cfg(feature = "gen-rust")]
mod rust;

#[cfg(feature = "gen-swift")]
mod swift;
mod error;

pub use error::Error;
use std::path::Path;
use serde::Deserialize;
use crate::api::config;
use crate::api::core::generator::{Context, Generator};
use crate::compiler::util::imports::ImportSolver;

pub trait GenTools {
    type Params<'a>: Deserialize<'a>;
    type Generator: crate::gen::Generator;
    type Solver: ImportSolver;

    fn new_solver() -> Self::Solver;
    fn new_generator() -> Self::Generator;
    fn generate<'a, 'b>(generator: &'b Generator<'a, Self::Solver, Self::Generator>, context: &mut Context<'b>, config: &config::model::Config<Self::Params<'_>>) -> Result<(), Error>;

    fn run(config: &config::model::Config<Self::Params<'_>>, out_dir: impl AsRef<Path>, post_generation: impl FnOnce(&Context)) -> Result<(), Error> {
        let motherfuckingrust = Self::new_solver();
        let protocols = config::core::compile(config, &motherfuckingrust)?;
        let (mut context, mut generator) = Generator::new(protocols, out_dir.as_ref(), Self::new_generator());
        if let Some(file_header) = config.package.file_header {
            generator.set_file_header(file_header);
        }
        Self::generate(&generator, &mut context, &config)?;
        post_generation(&context);
        Ok(())
    }

    fn run_string(config: impl AsRef<str>, out_dir: impl AsRef<Path>, post_generation: impl FnOnce(&Context)) -> Result<(), Error> {
        let config = config::core::parse::<Self::Params<'_>>(config.as_ref()).map_err(Error::Config)?;
        Self::run(&config, out_dir, post_generation)
    }

    fn run_file(config_file: impl AsRef<Path>, out_dir: impl AsRef<Path>, post_generation: impl FnOnce(&Context)) -> Result<(), Error> {
        let str = std::fs::read_to_string(config_file).map_err(Error::Io)?;
        Self::run_string(str, out_dir, post_generation)
    }
}

#[cfg(feature = "gen-rust")]
pub use rust::Rust;

#[cfg(feature = "gen-swift")]
pub use swift::Swift;
