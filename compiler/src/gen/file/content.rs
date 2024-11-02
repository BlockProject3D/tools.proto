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

use itertools::Itertools;
use std::fmt::Display;

pub struct Content<'a> {
    header: Option<&'a str>,
    body: String,
    footer: Option<&'a str>,
}

impl<'a> Content<'a> {
    pub fn from_iter<D: Display>(mut iter: impl Iterator<Item = D>) -> Self {
        Self {
            header: None,
            body: iter.join("\n"),
            footer: None,
        }
    }

    pub fn try_from_iter<D: Display, E>(iter: impl Iterator<Item = Result<D, E>>) -> Result<Self, E> {
        let data = iter.collect::<Result<Vec<D>, E>>()?;
        Ok(Self {
            header: None,
            body: data.iter().join("\n"),
            footer: None,
        })
    }

    pub fn from_string(value: String) -> Self {
        Self {
            header: None,
            body: value,
            footer: None,
        }
    }

    pub fn header(&mut self, header: &'a str) -> &mut Self {
        self.header = Some(header);
        self
    }

    pub fn footer(&mut self, footer: &'a str) -> &mut Self {
        self.footer = Some(footer);
        self
    }

    pub fn to_string(&self) -> Option<String> {
        if self.body.is_empty() {
            return None;
        }
        Some(match (self.header, self.footer) {
            (Some(header), None) => format!("{header}{}", self.body),
            (Some(header), Some(footer)) => format!("{header}{}{footer}", self.body),
            (None, Some(footer)) => format!("{}{footer}", self.body),
            (None, None) => self.body.to_string(),
        })
    }
}
