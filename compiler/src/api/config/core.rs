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

use serde::Deserialize;
use crate::api::config::model::Config;
use crate::api::core::loader::Loader;
use crate::api::core::generator::{Context, Generator, Params};
use crate::compiler::util::imports::{ImportSolver, ProtocolStore};
use crate::api::core::Error;

pub fn parse<'a, T: Deserialize<'a>>(data: &'a str) -> Result<Config<'a, T>, toml::de::Error> {
    toml::from_str(data)
}

pub fn compile<'a, T, I: ImportSolver>(config: &Config<T>, solver: &'a I) -> Result<ProtocolStore<'a, I>, Error> {
    let mut loader = Loader::default();
    loader.load_from_folder(config.package.path, config.package.name)?;
    if let Some(deps) = &config.dependency {
        for dep in deps {
            loader.load_from_file(dep.path, dep.package)?;
        }
    }
    loader.compile(solver)
}

pub fn generate<'a, G: crate::gen::Generator, T, I: ImportSolver, F: Fn(&T) -> Option<G::Params<'a>>>(
    generator: &'a Generator<'a, I, G>,
    context: &mut Context<'a>,
    config: &Config<T>,
    generator_params_converter: F,
    generator_default_params: &G::Params<'a>) -> Result<(), Error> {
    if let Some(options) = &config.options {
        for (protocol_full_name, params) in options {
            let mut p = Params::default();
            if let Some(flag) = params.write_messages {
                p.write_messages = flag;
            }
            if let Some(flag) = params.read_messages {
                p.read_messages = flag;
            }
            if let Some(flag) = params.use_enums {
                p.use_enums = flag;
            }
            if let Some(flag) = params.use_structs {
                p.use_structs = flag;
            }
            if let Some(flag) = params.use_unions {
                p.use_unions = flag;
            }
            if let Some(flag) = params.use_messages {
                p.use_messages = flag;
            }
            if let Some(gp) = generator_params_converter(&params.inner) {
                generator.generate(context, protocol_full_name, &p, &gp)?;
            } else {
                generator.generate(context, protocol_full_name, &p, generator_default_params)?;
            }
        }
    }
    let params = Params::default();
    generator.generate_all(context, &params, generator_default_params)
}
