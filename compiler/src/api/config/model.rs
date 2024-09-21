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

use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize};

#[derive(Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct RustParams<'a> {
    #[serde(borrow)]
    pub disable_read: Option<Vec<&'a str>>,
    pub disable_write: Option<Vec<&'a str>>,
    pub write_async: bool
}

#[derive(Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct Params<T> {
    pub write_messages: Option<bool>,
    pub read_messages: Option<bool>,
    pub use_enums: Option<bool>,
    pub use_structs: Option<bool>,
    pub use_messages: Option<bool>,
    pub use_unions: Option<bool>,
    #[serde(flatten)]
    pub inner: T
}

#[derive(Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct Package<'a> {
    pub name: &'a str,
    pub path: &'a Path,
    pub file_header: Option<&'a Path>,
}

#[derive(Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct Dependency<'a> {
    pub path: &'a Path,
    pub package: &'a str
}

#[derive(Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct Config<'a, T> {
    #[serde(borrow)]
    pub package: Package<'a>,
    #[serde(flatten)]
    pub options: Option<HashMap<&'a str, Params<T>>>,
    pub dependency: Option<Vec<Dependency<'a>>>
}
