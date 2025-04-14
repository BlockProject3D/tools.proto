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

use bp3d_protoc::compiler::message::{Field, FieldType, Message, Referenced};
use crate::buffer::Builder;
use crate::component::ComponentType;
use crate::field::option::Optional;
use crate::field::primitive::from_fixed_field_type;
use crate::proto::struct_ext::new_structure_internal;

fn new_field_internal(mut builder: Builder<'static>, field: &Field) -> Builder<'static> {
    match &field.ty {
        FieldType::Fixed(v) => builder.primitive(from_fixed_field_type(v.ty, field.endianness)),
        FieldType::Ref(v) => {
            match v {
                Referenced::Struct(v1) => new_structure_internal(Builder::new(&field.name), &*v1),
                Referenced::Message(v1) => new_message_internal(Builder::new(&field.name), &*v1)
            }
        }
        FieldType::Buffer => {
            panic!("Buffer types are currently not supported");
        }
        FieldType::SizedBuffer(_) => {
            panic!("Container types are currently not supported");
        }
        FieldType::FixedContainer(_) | FieldType::Container(_) | FieldType::SizedContainer(_) => {
            panic!("Container types are currently not supported");
        }
        FieldType::Union(_) => {
            panic!("Union types are currently not supported");
        }
        FieldType::Payload => {
            panic!("Payload types are currently not supported");
        }
    }
}

#[derive(Clone)]
struct FieldType1 {
    field: Field
}

impl ComponentType for FieldType1 {
    fn name(&self) -> &str {
        &self.field.name
    }

    fn build(&self, builder: Builder<'static>) -> Builder<'static> {
        new_field_internal(builder, &self.field)
    }
}

fn new_message_internal(mut builder: Builder<'static>, value: &Message) -> Builder<'static> {
    for field in &value.fields {
        if field.optional {
            let ft = FieldType1 {
                field: field.clone()
            };
            let opt = Optional(ft);
            builder = builder.add_child(Builder::new(&field.name).component(opt));
            continue;
        }
        builder = builder.add_child(new_field_internal(Builder::new(&field.name), field));
    }
    builder
}

impl ComponentType for Message {
    fn name(&self) -> &str {
        &self.name
    }

    fn build(&self, builder: Builder<'static>) -> Builder<'static> {
        new_message_internal(builder, self)
    }
}
