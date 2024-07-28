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

use std::rc::Rc;
use crate::compiler::error::CompilerError;
use crate::compiler::Protocol;
use crate::compiler::structure::{FixedFieldType, Structure};
use crate::model::message::MessageFieldType;
use crate::model::structure::StructFieldType;

#[derive(Clone, Debug)]
pub enum Referenced {
    Struct(Rc<Structure>),
    Message(Rc<Message>)
}

#[derive(Clone, Debug)]
pub struct FixedListField {
    pub ty: FixedFieldType,
    pub max_len: usize,
    pub item_type: Rc<Structure>
}

#[derive(Clone, Debug)]
pub struct VarcharStringField {
    pub ty: FixedFieldType,
    pub max_len: usize
}

#[derive(Clone, Debug)]
pub struct ListField {
    pub ty: FixedFieldType,
    pub max_len: usize,
    pub item_type: Rc<Message>
}

#[derive(Clone, Debug)]
pub enum FieldType {
    Fixed(FixedFieldType),
    Ref(Referenced),
    NullTerminatedString,
    VarcharString(VarcharStringField),
    FixedList(FixedListField)
}

#[derive(Clone, Debug)]
pub enum Payload {
    List(ListField),
    Data
}

#[derive(Clone, Debug)]
pub struct Field<T> {
    pub name: String,
    pub ty: T,
    pub optional: bool
}

#[derive(Clone, Debug)]
pub enum AnyField {
    Payload(Field<Payload>),
    Field(Field<FieldType>)
}

impl AnyField {
    fn from_model(proto: &Protocol, value: crate::model::message::MessageField) -> Result<Self, CompilerError> {
        match value.ty {
            MessageFieldType::Item { name } => {
                let r = proto.structs.get(&name)
                    .map(|v| Referenced::Struct(v.clone()))
                    .or_else(|| proto.messages.get(&name).map(|v| Referenced::Message(v.clone())))
                    .ok_or_else(|| CompilerError::UndefinedReference(name))?;
                match r {
                    Referenced::Struct(r) => {
                        if r.fields.len() == 1 && r.fields[0].as_fixed().is_some() {
                            let fixed = unsafe { r.fields[0].as_fixed().unwrap_unchecked() };
                            Ok(Self::Field(Field {
                                name: value.name,
                                ty: FieldType::Fixed(fixed.ty),
                                optional: value.optional
                            }))
                        } else {
                            Ok(Self::Field(Field {
                                name: value.name,
                                ty: FieldType::Ref(Referenced::Struct(r)),
                                optional: value.optional
                            }))
                        }
                    },
                    Referenced::Message(r) => {
                        Ok(Self::Field(Field {
                            name: value.name,
                            ty: FieldType::Ref(Referenced::Message(r)),
                            optional: value.optional
                        }))
                    }
                }
            },
            MessageFieldType::List { max_len, item } => {
                let r = proto.structs.get(&item)
                    .map(|v| Referenced::Struct(v.clone()))
                    .or_else(|| proto.messages.get(&item).map(|v| Referenced::Message(v.clone())))
                    .ok_or_else(|| CompilerError::UndefinedReference(item))?;
                let ty = FixedFieldType::from_max_value(max_len)?;
                match r {
                    Referenced::Struct(item_type) => {
                        Ok(Self::Field(Field {
                            name: value.name,
                            ty: FieldType::FixedList(FixedListField {
                                item_type,
                                max_len,
                                ty
                            }),
                            optional: value.optional
                        }))
                    },
                    Referenced::Message(item_type) => {
                        Ok(Self::Payload(Field {
                            name: value.name,
                            ty: Payload::List(ListField {
                                item_type,
                                ty,
                                max_len
                            }),
                            optional: value.optional
                        }))
                    }
                }
            },
            MessageFieldType::String { max_len } => {
                match max_len {
                    None => {
                        Ok(Self::Field(Field {
                            name: value.name,
                            ty: FieldType::NullTerminatedString,
                            optional: value.optional
                        }))
                    },
                    Some(max_len) => {
                        let ty = FixedFieldType::from_max_value(max_len)?;
                        Ok(Self::Field(Field {
                            name: value.name,
                            ty: FieldType::VarcharString(VarcharStringField {
                                max_len,
                                ty
                            }),
                            optional: value.optional
                        }))
                    }
                }
            },
            MessageFieldType::Payload => Ok(Self::Payload(Field {
                name: value.name,
                ty: Payload::Data,
                optional: value.optional
            }))
        }
    }
}

#[derive(Clone, Debug)]
pub struct Message {
    pub name: String,
    pub fields: Vec<Field<FieldType>>,
    pub payload: Option<Field<Payload>>
}

impl Message {
    pub fn from_model(proto: &Protocol, value: crate::model::message::Message) -> Result<Message, CompilerError> {
        let unsorted = value.fields.into_iter()
            .map(|v| AnyField::from_model(proto, v))
            .collect::<Result<Vec<AnyField>, CompilerError>>()?;
        let mut payloads = Vec::new();
        let mut fields = Vec::new();
        for v in unsorted {
            match v {
                AnyField::Payload(v) => payloads.push(v),
                AnyField::Field(v) => fields.push(v)
            }
        }
        if payloads.len() > 1 {
            return Err(CompilerError::MultiPayload);
        }
        let payload = payloads.into_iter().next();
        Ok(Message {
            name: value.name,
            fields,
            payload
        })
    }
}
