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

use crate::buffer::Builder;
use crate::component::factory::{ContainerOptions, Factory, Key};
use crate::component::ComponentType;
use crate::field::option::Optional;
use crate::field::primitive::from_fixed_field_type;
use crate::proto::struct_ext::new_structure_internal;
use bp3d_protoc::compiler::message::{Field, FieldType, Message, Referenced};
use std::ops::Deref;
use std::sync::Arc;

pub struct MessageExt {
    pub(crate) message: Arc<Message>,
    pub(crate) factory: Arc<Factory>,
}

impl Deref for MessageExt {
    type Target = Arc<Message>;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

fn new_field_internal(
    factory: &Arc<Factory>,
    builder: Builder<'static>,
    field: &Field,
) -> crate::component::factory::Result<Builder<'static>> {
    match &field.ty {
        FieldType::Fixed(v) => {
            Ok(builder.size(v.ty.get_byte_size()).primitive(from_fixed_field_type(v.ty, field.endianness)))
        }
        FieldType::Ref(v) => match v {
            Referenced::Struct(v1) => Ok(new_structure_internal(Builder::new(&field.name), &*v1)),
            Referenced::Message(v1) => new_message_internal(factory, Builder::new(&field.name), &*v1),
        },
        FieldType::Buffer => factory.with_component(Key::for_buffer(field.codec.as_ref().unwrap(), None), |comp| {
            comp.build(Builder::new(&field.name))
        }),
        FieldType::SizedBuffer(v) => factory.with_component(
            Key::for_buffer(field.codec.as_ref().unwrap(), Some((v.ty, field.endianness).into())),
            |comp| comp.build(Builder::new(&field.name)),
        ),
        FieldType::FixedContainer(v) => {
            let options = ContainerOptions {
                inner_ty: v.item_type.clone(),
                count_ty: (v.ty, field.endianness).into(),
                size_ty: None,
            };
            factory.with_component(Key::for_container(field.codec.as_ref().unwrap(), options), |comp| {
                comp.build(Builder::new(&field.name))
            })
        }
        FieldType::Container(v) => {
            let options = ContainerOptions {
                inner_ty: Arc::new(MessageExt {
                    factory: factory.clone(),
                    message: v.item_type.clone(),
                }),
                count_ty: (v.ty, field.endianness).into(),
                size_ty: None,
            };
            factory.with_component(Key::for_container(field.codec.as_ref().unwrap(), options), |comp| {
                comp.build(Builder::new(&field.name))
            })
        }
        FieldType::SizedContainer(v) => {
            let options = ContainerOptions {
                inner_ty: Arc::new(MessageExt {
                    factory: factory.clone(),
                    message: v.item_type.clone(),
                }),
                count_ty: (v.ty, field.endianness).into(),
                size_ty: Some((v.size_ty, field.endianness).into()),
            };
            factory.with_component(Key::for_container(field.codec.as_ref().unwrap(), options), |comp| {
                comp.build(Builder::new(&field.name))
            })
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
    field: Field,
    factory: Arc<Factory>,
}

impl ComponentType for FieldType1 {
    fn name(&self) -> &str {
        &self.field.name
    }

    fn build(&self, builder: Builder<'static>) -> Builder<'static> {
        //TODO: Better error handling
        new_field_internal(&self.factory, builder, &self.field).unwrap()
    }
}

fn new_message_internal(
    factory: &Arc<Factory>,
    mut builder: Builder<'static>,
    value: &Message,
) -> crate::component::factory::Result<Builder<'static>> {
    for field in &value.fields {
        if field.optional {
            let ft = FieldType1 {
                field: field.clone(),
                factory: factory.clone(),
            };
            let opt = Optional(ft);
            builder = builder.add_child(Builder::new(&field.name).component(opt));
            continue;
        }
        builder = builder.add_child(new_field_internal(factory, Builder::new(&field.name), field)?);
    }
    Ok(builder)
}

impl ComponentType for MessageExt {
    fn name(&self) -> &str {
        &self.message.name
    }

    fn key(&self) -> usize {
        Arc::as_ptr(&self.message) as _
    }

    fn build(&self, builder: Builder<'static>) -> Builder<'static> {
        //TODO: Better error handling
        new_message_internal(&self.factory, builder, &self.message).unwrap()
    }
}
