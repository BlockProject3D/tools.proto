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
use bp3d_util::result::ResultExt;
use bp3d_protoc::api::core::generator::{Context, Generator, Params};
use bp3d_protoc::api::core::loader::Loader;
use bp3d_protoc::compiler::util::imports::ImportSolver;
use crate::Args;

pub struct Builder<'a, I, G> {
    pub context: Context<'a>,
    pub generator: Generator<'a, I, G>,
    pub params: Params
}

impl<'a, I: ImportSolver, G: bp3d_protoc::gen::Generator> Builder<'a, I, G> {
    pub fn new(loader: Loader, args: &'a Args, solver: &'a I, generator: G) -> Self {
        let protocols = loader.compile(solver).expect_exit("failed to compile protocols", 1);
        let params = if let Some(features) = &args.features {
            let mut params = Params::new();
            for f in features {
                f.apply(&mut params);
            }
            params
        } else {
            Params::default()
        };
        let output = args.output.as_deref().unwrap_or(Path::new("./"));
        let (context, mut generator) = Generator::new(protocols, &output, generator);
        if let Some(file_header) = &args.file_header {
            generator.set_file_header(file_header);
        }
        Self {
            context,
            generator,
            params
        }
    }
}
