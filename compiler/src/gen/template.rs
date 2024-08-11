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
use std::collections::HashMap;
use bp3d_util::simple_error;
use itertools::Itertools;

simple_error! {
    pub Error {
        InvalidUTF8 => "invalid UTF-8 string in template",
        InvalidPop => "invalid when no fragment is on the stack",
        NoFragment => "cannot push code: no fragment is on the stack",
        FragmentNotFound(String) => "fragment not found: {}",
        VariableNotFound(String) => "variable not found: {}"
    }
}

#[derive(Eq, PartialEq)]
pub enum Component<'a> {
    Constant(&'a str),
    Variable(&'a str),
    NewLine
}

pub struct Fragment<'a> {
    name: &'a str,
    content: Vec<Component<'a>>
}

struct Token<'a> {
    start: usize,
    end: usize,
    data: &'a [u8]
}

impl<'a> Token<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            start: 0,
            end: 0
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
            let data = std::str::from_utf8(&self.data[start..end]).map_err(|_| Error::InvalidUTF8)?;
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    pub fn inc(&mut self) {
        self.end += 1;
    }
}

pub struct Template<'fragment, 'variable> {
    fragments: HashMap<String, Fragment<'fragment>>,
    variables: HashMap<&'variable str, Cow<'variable, str>>
}

impl<'fragment, 'variable> Template<'fragment, 'variable> {
    pub fn compile(data: &'fragment [u8]) -> Result<Self, Error> {
        let mut fragments = HashMap::new();
        let mut frag_stack = Vec::new();
        let lines = data.split(|v| *v == b'\n');
        for line in lines {
            if line.starts_with(b"#fragment push ") {
                let name = std::str::from_utf8(&line[15..]).map_err(|_| Error::InvalidUTF8)?;
                frag_stack.push(Fragment {
                    name,
                    content: Vec::new()
                });
            } else if line.starts_with(b"#fragment pop") {
                if frag_stack.is_empty() {
                    return Err(Error::InvalidPop);
                }
                let combined_name = frag_stack.iter().map(|v| v.name).join(".");
                //SAFETY: this is fine because the fragment stack must not be empty at this point.
                let mut fragment = unsafe { frag_stack.pop().unwrap_unchecked() };
                if fragment.content.last().map(|v| v == &Component::NewLine).unwrap_or(false) {
                    fragment.content.pop();
                }
                fragments.insert(combined_name, fragment);
            } else if !line.is_empty() {
                let cur_fragment = frag_stack.last_mut().ok_or(Error::NoFragment)?;
                let mut token = Token::new(line);
                while token.has_next() {
                    if token.cur() == b'{' {
                        if token.next() == Some(b'{') {
                            token.inc();
                        }
                        if let Some(component) = token.pop()?.map(Component::Constant) {
                            cur_fragment.content.push(component);
                        }
                    } else if token.cur() == b'}' {
                        if token.next() == Some(b'}') {
                            token.inc();
                            if let Some(component) = token.pop()?.map(Component::Constant) {
                                cur_fragment.content.push(component);
                            }
                        } else {
                            if let Some(component) = token.pop()?.map(Component::Variable) {
                                cur_fragment.content.push(component);
                            }
                        }
                    }
                    token.inc()
                }
                if let Some(component) = token.pop()?.map(Component::Constant) {
                    cur_fragment.content.push(component);
                }
                cur_fragment.content.push(Component::NewLine);
            }
        }
        Ok(Template {
            fragments,
            variables: HashMap::new()
        })
    }

    pub fn var(&mut self, key: &'variable str, value: impl Into<Cow<'variable, str>>) -> &mut Self {
        self.variables.insert(key, value.into());
        self
    }

    pub fn var_d(&mut self, key: &'variable str, value: impl ToString) -> &mut Self {
        self.variables.insert(key, value.to_string().into());
        self
    }

    fn render_internal(&self, variables: &HashMap<&str, Cow<str>>, path: &str, fragments: &[&str]) -> Result<String, Error> {
        let mut rendered = Vec::new();
        for name in fragments {
            let name: Cow<str> = match path.is_empty() {
                false => Cow::Owned(format!("{}.{}", path, name)),
                true => Cow::Borrowed(name)
            };
            let fragment = self.fragments.get(&*name).ok_or_else(|| Error::FragmentNotFound(String::from(&*name)))?;
            let sub_rendered = fragment.content.iter().map(|v| match v {
                Component::Constant(v) => Ok(*v),
                Component::Variable(v) => variables.get(v).map(|v| &**v).ok_or_else(|| Error::VariableNotFound(String::from(*v))),
                Component::NewLine => Ok("\n")
            }).collect::<Result<Vec<&str>, Error>>()?.join("");
            rendered.push(sub_rendered);
        }
        Ok(rendered.join(""))
    }

    pub fn scope(&self) -> Scope {
        Scope {
            template: self,
            variables: self.variables.clone()
        }
    }

    pub fn render(&self, path: &str, fragments: &[&str]) -> Result<String, Error> {
        self.render_internal(&self.variables, path, fragments)
    }
}

pub struct Scope<'a, 'fragment, 'variable> {
    template: &'a Template<'fragment, 'variable>,
    variables: HashMap<&'variable str, Cow<'variable, str>>
}

impl<'a, 'fragment, 'variable> Scope<'a, 'fragment, 'variable> {
    pub fn var(&mut self, key: &'variable str, value: impl Into<Cow<'variable, str>>) -> &mut Self {
        self.variables.insert(key, value.into());
        self
    }

    pub fn var_d(&mut self, key: &'variable str, value: impl ToString) -> &mut Self {
        self.variables.insert(key, value.to_string().into());
        self
    }

    pub fn render_to_var(&mut self, path: &str, fragments: &[&str], key: &'variable str) -> Result<&mut Self, Error> {
        let str = self.template.render_internal(&self.variables, path, fragments)?;
        self.variables.insert(key, str.into());
        Ok(self)
    }

    pub fn render(&self, path: &str, fragments: &[&str]) -> Result<String, Error> {
        self.template.render_internal(&self.variables, path, fragments)
    }
}
