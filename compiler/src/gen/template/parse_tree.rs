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

use crate::gen::template::{Error, FunctionMap};
use std::borrow::Cow;

pub struct Variable<'a> {
    pub name: &'a str,
    pub function: Option<fn(&str) -> Cow<str>>,
}

pub enum Component<'a> {
    Constant(&'a str),
    Variable(Variable<'a>),
    NewLine,
}

impl<'a> Component<'a> {
    pub fn parse_variable(
        function_map: &FunctionMap,
        variable: &'a str,
    ) -> Result<Component<'a>, Error> {
        match variable.find(":") {
            None => Ok(Component::Variable(Variable {
                name: variable,
                function: None,
            })),
            Some(id) => {
                let name = &variable[..id];
                let function_name = &variable[id + 1..];
                let function = function_map
                    .get(function_name)
                    .ok_or_else(|| Error::FunctionNotFound(function_name.into()))?;
                Ok(Component::Variable(Variable {
                    name,
                    function: Some(function),
                }))
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FragmentMode {
    Inline,
    Default,
}

impl FragmentMode {
    pub fn from_str(name: &str) -> Option<FragmentMode> {
        match name {
            "inline" => Some(FragmentMode::Inline),
            "Default" => Some(FragmentMode::Default),
            _ => None,
        }
    }
}

pub struct Fragment<'a> {
    pub(crate) name: &'a str,
    pub(crate) content: Vec<Component<'a>>,
    pub(crate) mode: FragmentMode,
}

pub struct Token<'a> {
    start: usize,
    end: usize,
    data: &'a [u8],
}

impl<'a> Token<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            start: 0,
            end: 0,
        }
    }

    pub fn has_next(&self) -> bool {
        self.end < self.data.len()
    }

    pub fn cur(&self) -> u8 {
        self.data[self.end]
    }

    pub fn next(&self) -> Option<u8> {
        if self.end + 1 < self.data.len() {
            Some(self.data[self.end + 1])
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Result<Option<&'a str>, Error> {
        let start = self.start;
        let end = self.end;
        self.start = end + 1;
        self.end = end;
        if end > start && end - start > 0 {
            let data =
                std::str::from_utf8(&self.data[start..end]).map_err(|_| Error::InvalidUTF8)?;
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    pub fn inc(&mut self) {
        self.end += 1;
    }
}
