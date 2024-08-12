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
use regex::Regex;
use std::borrow::Cow;

enum CaseConvention {
    PascalCase,
    SnakeCase,
    ScreamingCase,
}

fn guess_case_convention(s: &str) -> CaseConvention {
    //Assume all strings are Rust identifiers following standard Rust conventions:
    // snake_case, PascalCase and SCREAMING_CASE.
    let upper1 = (s.as_bytes()[0] >= b'A' && s.as_bytes()[0] <= b'Z') || s.as_bytes()[0] == b'_';
    let upper2 = (s.as_bytes()[s.as_bytes().len() - 1] >= b'A'
        && s.as_bytes()[s.as_bytes().len() - 1] <= b'Z')
        || s.as_bytes()[s.as_bytes().len() - 1] == b'_';
    if upper1 && upper2 {
        CaseConvention::ScreamingCase
    } else if upper1 {
        CaseConvention::PascalCase
    } else {
        CaseConvention::SnakeCase
    }
}

fn capitalize(value: &str) -> Cow<str> {
    if value.len() == 0 {
        return value.into();
    }
    if value.as_bytes()[0] >= b'A' && value.as_bytes()[0] <= b'Z' {
        value.into()
    } else {
        (value[..1].to_ascii_uppercase() + &value[1..]).into()
    }
}

fn decapitalize(value: &str) -> Cow<str> {
    if value.len() == 0 {
        return value.into();
    }
    if value.as_bytes()[0] >= b'A' && value.as_bytes()[0] <= b'Z' {
        (value[..1].to_ascii_lowercase() + &value[1..]).into()
    } else {
        value.into()
    }
}

pub trait CaseConversion<'a> {
    fn to_pascal_case(self) -> Cow<'a, str>;
    fn to_snake_case(self) -> Cow<'a, str>;
    fn to_camel_case(self) -> Cow<'a, str>;
    fn to_screaming_case(self) -> Cow<'a, str>;
}

struct SnakeCase<'a>(&'a str);
struct PascalCase<'a>(&'a str);
struct ScreamingCase<'a>(&'a str);

impl<'a> CaseConversion<'a> for SnakeCase<'a> {
    fn to_pascal_case(self) -> Cow<'a, str> {
        self.0.split("_").map(capitalize).join("").into()
    }

    fn to_snake_case(self) -> Cow<'a, str> {
        self.0.into()
    }

    fn to_camel_case(self) -> Cow<'a, str> {
        self.0
            .split("_")
            .enumerate()
            .map(|(i, v)| if i != 0 { capitalize(v) } else { v.into() })
            .join("")
            .into()
    }

    fn to_screaming_case(self) -> Cow<'a, str> {
        self.0.split("_").map(|v| v.to_uppercase()).join("_").into()
    }
}

impl<'a> CaseConversion<'a> for PascalCase<'a> {
    fn to_pascal_case(self) -> Cow<'a, str> {
        self.0.into()
    }

    fn to_snake_case(self) -> Cow<'a, str> {
        let regex = Regex::new("[A-Z]([a-z]|[0-9])*").unwrap();
        //Unfortunately Rust is a piece of shit unable to understand that to_lowercase is supposed to return an owned value so by definition not a Cow!!!!!
        format!(
            "{}",
            regex.find_iter(self.0).map(|v| v.as_str().to_lowercase()).join("_")
        )
        .into()
    }

    fn to_camel_case(self) -> Cow<'a, str> {
        decapitalize(self.0)
    }

    fn to_screaming_case(self) -> Cow<'a, str> {
        let regex = Regex::new("[A-Z]([a-z]|[0-9])*").unwrap();
        //Unfortunately Rust is a piece of shit unable to understand that to_lowercase is supposed to return an owned value so by definition not a Cow!!!!!
        format!(
            "{}",
            regex.find_iter(self.0).map(|v| v.as_str().to_uppercase()).join("_")
        )
        .into()
    }
}

impl<'a> CaseConversion<'a> for ScreamingCase<'a> {
    fn to_pascal_case(self) -> Cow<'a, str> {
        //Unfortunately Rust is a piece of shit unable to understand that to_owned is supposed to return an owned value so by definition not a Cow!!!!!
        self.0.split("_").map(|v| format!("{}", capitalize(&v.to_lowercase()))).join("").into()
    }

    fn to_snake_case(self) -> Cow<'a, str> {
        self.0.to_lowercase().into()
    }

    fn to_camel_case(self) -> Cow<'a, str> {
        self.0
            .split("_")
            .enumerate()
            .map(|(i, v)| {
                if i != 0 {
                    format!("{}", capitalize(&v.to_lowercase()))
                } else {
                    v.into()
                }
            })
            .join("")
            .into()
    }

    fn to_screaming_case(self) -> Cow<'a, str> {
        self.0.into()
    }
}

macro_rules! impl_case_conversion {
    ($s: expr, $func: ident) => {
        match guess_case_convention($s) {
            CaseConvention::PascalCase => PascalCase($s).$func(),
            CaseConvention::SnakeCase => SnakeCase($s).$func(),
            CaseConvention::ScreamingCase => ScreamingCase($s).$func(),
        }
    };
}

impl<'a> CaseConversion<'a> for &'a str {
    fn to_pascal_case(self) -> Cow<'a, str> {
        impl_case_conversion!(self, to_pascal_case)
    }

    fn to_snake_case(self) -> Cow<'a, str> {
        impl_case_conversion!(self, to_snake_case)
    }

    fn to_camel_case(self) -> Cow<'a, str> {
        impl_case_conversion!(self, to_camel_case)
    }

    fn to_screaming_case(self) -> Cow<'a, str> {
        impl_case_conversion!(self, to_screaming_case)
    }
}
