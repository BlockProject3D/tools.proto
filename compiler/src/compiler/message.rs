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
use crate::compiler::error::Error;
use crate::compiler::Protocol;
use crate::compiler::structure::{FixedFieldType, Structure};
use crate::compiler::union::Union;
use crate::model::message::MessageFieldType;

#[derive(Clone, Debug)]
pub enum Referenced {
    Struct(Rc<Structure>),
    Message(Rc<Message>)
}

impl Referenced {
    pub fn name(&self) -> &str {
        match self {
            Referenced::Struct(v) => &v.name,
            Referenced::Message(v) => &v.name
        }
    }

    pub fn lookup(proto: &Protocol, reference_name: &str) -> Option<Self> {
        proto.structs_by_name.get(reference_name)
            .map(|v| Referenced::Struct(v.clone()))
            .or_else(|| proto.messages_by_name.get(reference_name).map(|v| Referenced::Message(v.clone())))
    }
}

#[derive(Clone, Debug)]
pub struct ArrayField {
    pub ty: FixedFieldType,
    pub item_type: Rc<Structure>
}

#[derive(Clone, Debug)]
pub struct VarcharStringField {
    pub ty: FixedFieldType,
}

#[derive(Clone, Debug)]
pub struct ListField {
    pub ty: FixedFieldType,
    pub item_type: Rc<Message>
}

#[derive(Clone, Debug)]
pub struct FixedField {
    pub ty: FixedFieldType
}

#[derive(Clone, Debug)]
pub struct UnionField {
    pub r: Rc<Union>,
    pub on_name: String,
    pub on_index: usize
}

#[derive(Clone, Debug)]
pub enum FieldType {
    Fixed(FixedField),
    Ref(Referenced),
    NullTerminatedString,
    VarcharString(VarcharStringField),
    Array(ArrayField),
    Union(UnionField),
    List(ListField),
    Payload
}

#[derive(Copy, Clone, Debug)]
pub struct SizeInfo {
    pub is_dyn_sized: bool,
    pub is_element_dyn_sized: bool
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub ty: FieldType,
    pub optional: bool,
    pub size: SizeInfo
}

impl Field {
    fn from_model(proto: &Protocol, unsorted: &[Field], value: crate::model::message::MessageField) -> Result<Self, Error> {
        match value.info {
            MessageFieldType::Item { item_type } => {
                let r = Referenced::lookup(proto, &item_type).ok_or_else(|| Error::UndefinedReference(item_type))?;
                match r {
                    Referenced::Struct(r) => {
                        if r.fields.len() == 1 && r.fields[0].as_fixed().is_some()
                            && r.fields[0].as_fixed().map(|v| v.view.is_transmute()).unwrap_or_default()
                            && r.fields[0].as_fixed().map(|v| v.loc.bit_size % 8 == 0).unwrap_or_default() {
                            let fixed = unsafe { r.fields[0].as_fixed().unwrap_unchecked() };
                            Ok(Field {
                                name: value.name,
                                ty: FieldType::Fixed(FixedField {
                                    ty: fixed.ty
                                }),
                                optional: value.optional.unwrap_or_default(),
                                size: SizeInfo {
                                    is_dyn_sized: false,
                                    is_element_dyn_sized: false
                                }
                            })
                        } else {
                            Ok(Field {
                                name: value.name,
                                ty: FieldType::Ref(Referenced::Struct(r)),
                                optional: value.optional.unwrap_or_default(),
                                size: SizeInfo {
                                    is_dyn_sized: false,
                                    is_element_dyn_sized: false
                                }
                            })
                        }
                    },
                    Referenced::Message(r) => {
                        Ok(Field {
                            name: value.name,
                            optional: value.optional.unwrap_or_default(),
                            size: r.size,
                            ty: FieldType::Ref(Referenced::Message(r))
                        })
                    }
                }
            },
            MessageFieldType::List { max_len, item_type } => {
                let r = Referenced::lookup(proto, &item_type).ok_or_else(|| Error::UndefinedReference(item_type))?;
                let ty = FixedFieldType::from_max_value(max_len)?;
                match r {
                    Referenced::Struct(item_type) => {
                        Ok(Field {
                            name: value.name,
                            ty: FieldType::Array(ArrayField {
                                item_type,
                                ty
                            }),
                            optional: value.optional.unwrap_or_default(),
                            size: SizeInfo {
                                is_element_dyn_sized: false,
                                is_dyn_sized: true
                            }
                        })
                    },
                    Referenced::Message(item_type) => {
                        Ok(Field {
                            name: value.name,
                            ty: FieldType::List(ListField {
                                item_type,
                                ty,
                            }),
                            optional: value.optional.unwrap_or_default(),
                            size: SizeInfo {
                                is_element_dyn_sized: true,
                                is_dyn_sized: true
                            }
                        })
                    }
                }
            },
            MessageFieldType::String { max_len } => {
                match max_len {
                    None => {
                        Ok(Field {
                            name: value.name,
                            ty: FieldType::NullTerminatedString,
                            optional: value.optional.unwrap_or_default(),
                            size: SizeInfo {
                                is_element_dyn_sized: false,
                                is_dyn_sized: true
                            }
                        })
                    },
                    Some(max_len) => {
                        let ty = FixedFieldType::from_max_value(max_len)?;
                        Ok(Field {
                            name: value.name,
                            ty: FieldType::VarcharString(VarcharStringField {
                                ty
                            }),
                            optional: value.optional.unwrap_or_default(),
                            size: SizeInfo {
                                is_element_dyn_sized: false,
                                is_dyn_sized: true
                            }
                        })
                    }
                }
            },
            MessageFieldType::Union { on, item_type } => {
                let (on_index, on_field) = unsorted.iter().enumerate().find_map(|(k, v)| if v.name == on {
                        Some((k, v))
                    } else {
                        None
                    }).ok_or_else(|| Error::UndefinedReference(on))?;
                let r = proto.unions_by_name.get(&item_type).ok_or_else(|| Error::UndefinedReference(item_type))?;
                match &on_field.ty {
                    FieldType::Ref(v) => match v {
                        Referenced::Struct(v) => {
                            if !Rc::ptr_eq(&r.discriminant.root, v) {
                                return Err(Error::UnionTypeMismatch);
                            }
                        }
                        _ => return Err(Error::UnionTypeMismatch)
                    }
                    _ => return Err(Error::UnionTypeMismatch)
                }
                let on_name = on_field.name.clone();
                if value.optional.unwrap_or_default() {
                    eprintln!("WARNING: ignoring unsupported optional flag on union message field!");
                }
                Ok(Field {
                    name: value.name,
                    ty: FieldType::Union(UnionField {
                        r: r.clone(),
                        on_name,
                        on_index
                    }),
                    optional: false,
                    size: r.size
                })
            }
            MessageFieldType::Payload => Ok(Field {
                name: value.name,
                ty: FieldType::Payload,
                optional: value.optional.unwrap_or_default(),
                size: SizeInfo {
                    is_dyn_sized: true,
                    is_element_dyn_sized: true
                }
            }),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Message {
    pub name: String,
    pub fields: Vec<Field>,
    pub size: SizeInfo
}

impl Message {
    pub fn from_model(proto: &Protocol, value: crate::model::message::Message) -> Result<Message, Error> {
        let mut fields = Vec::with_capacity(value.fields.len());
        let mut dyn_sized_elem_count = 0;
        let mut is_dyn_sized = false;
        for v in value.fields {
            let field = Field::from_model(proto, &fields, v)?;
            if field.size.is_dyn_sized {
                is_dyn_sized = true;
            }
            if dyn_sized_elem_count > 0 && (field.size.is_dyn_sized || field.size.is_element_dyn_sized) {
                return Err(Error::VarsizeAfterPayload)
            }
            if field.size.is_element_dyn_sized {
                dyn_sized_elem_count += 1;
            }
            if dyn_sized_elem_count > 1 {
                return Err(Error::MultiPayload)
            }
            fields.push(field);
        }
        Ok(Message {
            name: value.name,
            fields,
            size: SizeInfo {
                is_dyn_sized,
                is_element_dyn_sized: dyn_sized_elem_count > 0
            }
        })
    }
}
