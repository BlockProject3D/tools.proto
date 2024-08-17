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

use std::borrow::Cow;
use std::io::Write;
use std::path::{Path, PathBuf};
use bp3d_util::path::PathExt;
use itertools::Itertools;

pub struct B<T>(pub T);

pub trait Content {
    fn to_string(self) -> Option<String>;
}

impl Content for String {
    fn to_string(self) -> Option<String> {
        Some(self)
    }
}

impl<'a> Content for &'a str {
    fn to_string(self) -> Option<String> {
        Some(self.into())
    }
}

trait Content2<I> {
    fn to_string(self) -> Option<String>;
}

trait Content3<I> {
    fn to_string(self) -> Option<String>;
}

trait Content1<I> {
    fn to_string(self) -> Option<String>;
}

impl<'a, H: AsRef<str>, B: Iterator<Item=&'a str>, F: AsRef<String>> Content3<&'a str> for (H, B, F) {
    fn to_string(mut self) -> Option<String> {
        let data = self.1.join("\n");
        if data.is_empty() {
            None
        } else {
            Some(format!("{}{}{}", self.0.as_ref(), data, self.2.as_ref()))
        }
    }
}

impl<H: AsRef<str>, B: Iterator<Item=String>, F: AsRef<String>> Content3<String> for (H, B, F) {
    fn to_string(mut self) -> Option<String> {
        let data = self.1.join("\n");
        if data.is_empty() {
            None
        } else {
            Some(format!("{}{}{}", self.0.as_ref(), data, self.2.as_ref()))
        }
    }
}

impl<'a, H: AsRef<str>, B: Iterator<Item=&'a str>> Content2<&'a str> for (H, B) {
    fn to_string(mut self) -> Option<String> {
        let data = self.1.join("\n");
        if data.is_empty() {
            None
        } else {
            Some(format!("{}{}", self.0.as_ref(), data))
        }
    }
}

impl<H: AsRef<str>, B: Iterator<Item=String>> Content2<String> for (H, B) {
    fn to_string(mut self) -> Option<String> {
        let data = self.1.join("\n");
        if data.is_empty() {
            None
        } else {
            Some(format!("{}{}", self.0.as_ref(), data))
        }
    }
}

impl<'a, B: Iterator<Item=&'a str>> Content1<&'a str> for B {
    fn to_string(mut self) -> Option<String> {
        Some(self.join("\n"))
    }
}

impl<B: Iterator<Item=String>> Content1<String> for B {
    fn to_string(mut self) -> Option<String> {
        Some(self.join("\n"))
    }
}

impl<T: Iterator> Content for B<T> where T: Content1<T::Item> {
    fn to_string(self) -> Option<String> {
        <T as Content1<T::Item>>::to_string(self.0)
    }
}

impl<H, B: Iterator, F> Content for (H, B, F) where Self: Content3<B::Item> {
    fn to_string(self) -> Option<String> {
        <Self as Content3<B::Item>>::to_string(self)
    }
}

impl<H, B: Iterator> Content for (H, B) where Self: Content2<B::Item> {
    fn to_string(self) -> Option<String> {
        <Self as Content2<B::Item>>::to_string(self)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum FileType {
    MessageWriting,
    MessageReading,
    Message,
    Structure,
    Enum,
    Union,
}

pub struct File {
    name: Cow<'static, str>,
    data: Option<String>,
    ty: FileType,
}

impl File {
    pub fn new(ty: FileType, name: impl Into<Cow<'static, str>>, data: impl Content) -> Self {
        Self {
            name: name.into(),
            ty,
            data: data.to_string(),
        }
    }

    pub fn ty(&self) -> FileType {
        self.ty
    }

    pub fn write(self, out_directory: &Path, file_header: Option<&str>, extension: &str) -> std::io::Result<Option<PathBuf>> {
        if let Some(data) = self.data {
            if data.len() <= 1 {
                return Ok(None);
            }
            let sub_folder = self.name.find("/").map(|id| &self.name[..id]);
            if let Some(sub_folder) = sub_folder {
                std::fs::create_dir(out_directory.join(sub_folder))?;
            }
            let path = out_directory.join(&*self.name).ensure_extension(extension).to_path_buf();
            let mut file = std::fs::File::create(&path)?;
            if let Some(file_header) = file_header {
                file.write_all(file_header.as_bytes())?;
            }
            file.write_all(data.as_bytes())?;
            file.flush()?;
            drop(file);
            Ok(Some(path))
        } else {
            Ok(None)
        }
    }
}
