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
use std::ops::Index;
use crate::gen::template::{Error, Scope, Template};

pub trait Render {
    fn render_frag(&self, fragment: &Fragment) -> Result<String, Error>;
}

pub trait RenderToVar<'a>: Render {
    fn render_frag_to_var(&mut self, fragment: &Fragment, key: &'a str) -> Result<&mut Self, Error>;
}

impl<'fragment, 'variable> Render for Template<'fragment, 'variable> {
    fn render_frag(&self, fragment: &Fragment) -> Result<String, Error> {
        self.render(fragment.path, fragment.fragments)
    }
}

impl<'a, 'fragment, 'variable> Render for Scope<'a, 'fragment, 'variable> {
    fn render_frag(&self, fragment: &Fragment) -> Result<String, Error> {
        self.render(fragment.path, fragment.fragments)
    }
}

impl<'a, 'fragment, 'variable> RenderToVar<'variable> for Scope<'a, 'fragment, 'variable> {
    fn render_frag_to_var(&mut self, fragment: &Fragment, key: &'variable str) -> Result<&mut Self, Error> {
        self.render_to_var(fragment.path, fragment.fragments, key)
    }
}

pub struct Fragment<'b> {
    path: &'b str,
    fragments: &'b [&'b str]
}

impl<'b> Fragment<'b> {
    pub fn new(path: &'b str, fragments: &'b [&'b str]) -> Self {
        Self {
            path,
            fragments
        }
    }
}

pub struct Function<'b> {
    map: HashMap<&'b str, Fragment<'b>>
}

impl<'b> Default for Function<'b> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'b> Function<'b> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn fragment(mut self, name: &'b str, path: &'b str, fragments: &'b [&'b str]) -> Self {
        self.map.insert(name, Fragment::new(path, fragments));
        self
    }
}

impl<'b> Index<&str> for Function<'b> {
    type Output = Fragment<'b>;

    fn index(&self, index: &str) -> &Self::Output {
        &self.map[index]
    }
}

pub enum Hook<'a> {
    Fragment(Fragment<'a>),
    Function(Function<'a>)
}

impl<'a> From<Function<'a>> for Hook<'a> {
    fn from(value: Function<'a>) -> Self {
        Self::Function(value)
    }
}

impl<'a> From<Fragment<'a>> for Hook<'a> {
    fn from(value: Fragment<'a>) -> Self {
        Self::Fragment(value)
    }
}

pub struct TemplateHooks<'a> {
    map: HashMap<&'a str, Vec<Hook<'a>>>
}

impl<'a> Default for TemplateHooks<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> TemplateHooks<'a> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn hook(&mut self, name: &'a str, hook: impl Into<Hook<'a>>) -> &mut Self {
        let entry = self.map.entry(name).or_insert(Vec::new());
        entry.push(hook.into());
        self
    }

    pub fn get_fragments(&self, name: &str) -> impl Iterator<Item=&Fragment> {
        self.map.get(name).map(|v| v.iter()).unwrap_or([].iter()).filter_map(|v| match v {
            Hook::Fragment(v) => Some(v),
            Hook::Function(_) => None
        })
    }

    pub fn get_functions(&self, name: &str) -> impl Iterator<Item=&Function> {
        self.map.get(name).map(|v| v.iter()).unwrap_or([].iter()).filter_map(|v| match v {
            Hook::Fragment(_) => None,
            Hook::Function(v) => Some(v)
        })
    }
}
