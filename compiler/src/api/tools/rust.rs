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

use crate::api::config;
use crate::api::config::model;
use crate::api::config::model::Config;
use crate::api::core::generator::{Context, Generator};
use crate::api::tools::Error;
use crate::api::tools::GenTools;
use crate::gen::{GeneratorRust, RustImportSolver, RustParams};

pub struct Rust;

impl GenTools for Rust {
    type Params<'a> = model::RustParams<'a>;
    type Generator = GeneratorRust;
    type Solver = RustImportSolver;

    fn new_solver() -> Self::Solver {
        RustImportSolver
    }

    fn new_generator() -> Self::Generator {
        GeneratorRust
    }

    fn generate<'a, 'b>(
        generator: &'b Generator<'a, Self::Solver, Self::Generator>,
        context: &mut Context<'b>,
        config: &Config<Self::Params<'_>>,
    ) -> Result<(), Error> {
        config::core::generate(
            generator,
            context,
            config,
            |v| {
                if v.disable_write.is_none()
                    && v.disable_read.is_none()
                    && v.write_async.is_none()
                    && v.union_set_discriminant.is_none()
                    && v.list_wrappers.is_none()
                    && v.struct_to_mut.is_none()
                {
                    return None;
                }
                let mut params = RustParams::default();
                if let Some(v) = &v.disable_read {
                    for v in v {
                        params = params.disable_read(v);
                    }
                }
                if let Some(v) = &v.disable_write {
                    for v in v {
                        params = params.disable_write(v);
                    }
                }
                params = params.enable_write_async(v.write_async.unwrap_or_default());
                params = params.enable_union_set_discriminant(v.union_set_discriminant.unwrap_or_default());
                params = params.enable_list_wrappers(v.list_wrappers.unwrap_or_default());
                params = params.enable_struct_to_mut(v.struct_to_mut.unwrap_or_default());
                Some(params)
            },
            &RustParams::default(),
        )?;
        Ok(())
    }
}
