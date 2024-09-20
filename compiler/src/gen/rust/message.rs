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

use crate::compiler::message::Message;
use crate::gen::base::map::{DefaultTypeMapper, TypePathMapper};
use crate::gen::base::message::{gen_message_array_type_decls, generate};
use crate::gen::rust::util::RustUtils;
use crate::gen::template::Template;
use crate::compiler::util::types::TypePathMap;

const TEMPLATE: &[u8] = include_bytes!("./message.template");
const TEMPLATE_EXT: &[u8] = include_bytes!("./message.ext.template");

pub fn gen_message_decl(msg: &Message, type_path_map: &TypePathMap) -> String {
    let type_path_map = TypePathMapper::new(type_path_map, DefaultTypeMapper);
    let mut template = Template::compile(TEMPLATE).unwrap();
    let mut template_ext = Template::compile(TEMPLATE_EXT).unwrap();
    template_ext.var("msg_name", &msg.name);
    template.var(
        "generics",
        RustUtils::get_generics(msg, &type_path_map).to_string_with_defaults(),
    );
    let mut code = generate::<RustUtils, _>(template, msg, &type_path_map);
    code += "\n";
    code += &gen_message_array_type_decls::<RustUtils, _>(&template_ext, msg, &type_path_map);
    code
}
