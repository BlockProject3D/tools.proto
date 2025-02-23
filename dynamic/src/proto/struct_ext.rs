// Copyright (c) 2025, BlockProject 3D
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

use bp3d_util::extension;
use bp3d_protoc::compiler::structure::{Field, FieldType, Structure};
use bp3d_protoc::compiler::util::types::Name;
use crate::buffer::{BufferView, Builder};
use crate::component::ComponentType;

extension! {
    pub extension StructureExt: Structure {
        fn get_field(&self, path: &str) -> Option<&Field>;
    }
}

fn new_structure_internal<'a>(value: &Structure) -> Builder<'a> {
    let mut builder = Builder::new(value.name()).fixed(0, value.byte_size);
    for field in &value.fields {
        let mut field_builder = Builder::new(&field.name)
            .fixed(field.loc.byte_offset, field.loc.byte_size);
        if let Some(primitive) = crate::field::primitive::from_field(field) {
            field_builder = field_builder.primitive(primitive);
        }
        match &field.ty {
            FieldType::Struct(v) => {
                field_builder = field_builder.add_child(new_structure_internal(v));
            }
            _ => ()
        }
        builder = builder.add_child(field_builder);
    }
    builder
}

impl ComponentType for Structure {
    fn new_instance(&self, init_mem: bool) -> BufferView<'static> {
        new_structure_internal(self).build(init_mem)
    }
}

impl StructureExt for Structure {
    fn get_field(&self, path: &str) -> Option<&Field> {
        let mut segments = path.split('.');
        let mut structure = self;
        while let Some(segment) = segments.next() {
            let field = structure.fields.iter().find(|v| v.name == segment)?;
            match &field.ty {
                FieldType::Struct(v) => structure = &*v,
                _ => {
                    match segments.next() {
                        None => return Some(field),
                        Some(_) => return None
                    }
                }
            }
        }
        None
    }
}
