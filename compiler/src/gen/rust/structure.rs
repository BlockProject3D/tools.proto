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

use crate::compiler::structure::Structure;
use crate::compiler::util::types::{Name, TypePathMap};
use crate::gen::base::map::{DefaultTypeMapper, TypePathMapper};
use crate::gen::base::structure::{generate, Templates};
use crate::gen::hook::{Fragment, TemplateHooks};
use crate::gen::rust::util::RustUtils;
use crate::gen::template::{Options, Template};
use crate::gen::RustParams;

const STRUCT_TEMPLATE: &[u8] = include_bytes!("./structure.template");
const STRUCT_FIELD_TEMPLATE: &[u8] = include_bytes!("./structure.field.template");

pub fn gen_structure_decl(s: &Structure, type_path_map: &TypePathMap, params: &RustParams) -> String {
    let mut options = Options::default();
    if params.disable_read.contains(s.name()) {
        options.disable("from_slice").disable("getters");
    }
    if params.disable_write.contains(s.name()) {
        options.disable("write_to").disable("write_to_async").disable("setters");
    }
    let templates = Templates {
        template: Template::compile_with_options(STRUCT_TEMPLATE, &options).unwrap(),
        field_template: Template::compile_with_options(STRUCT_FIELD_TEMPLATE, &options).unwrap(),
    };
    let mut hooks = TemplateHooks::new();
    if params.enable_write_async {
        hooks.hook("ext", Fragment::new("", &["write_to_async"]));
    }
    generate::<RustUtils, _>(
        templates,
        s,
        &TypePathMapper::new(type_path_map, DefaultTypeMapper),
        &hooks,
    )
}

#[cfg(test)]
mod tests {
    use crate::gen::rust::structure::STRUCT_TEMPLATE;
    use crate::gen::template::Template;

    #[test]
    fn test_template_render() {
        let mut template = Template::compile(STRUCT_TEMPLATE).unwrap();
        template.var("name", "Test");
        let code = template.render("", &["decl"]).unwrap();
        assert_eq!(
            &*code,
            "#[derive(Copy, Clone, Default, Debug)]
pub struct Test<T> {
    data: T
}
"
        )
    }
}
