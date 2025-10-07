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

use crate::compiler::builder::FieldBuilder;
use crate::compiler::error::Error;
use crate::compiler::structure::{FixedFieldType, Structure};
use crate::compiler::union::Union;
use crate::compiler::util::objects::name_index;
use crate::compiler::util::types::{Name, PtrKey};
use crate::compiler::Protocol;
use crate::model::message::MessageFieldValue;
use crate::model::protocol::{Description, Endianness};
use crate::model::structure::StructFieldRaw;
use bp3d_debug::{error, trace};
use std::cell::Cell;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Referenced {
    Struct(Arc<Structure>),
    Message(Arc<Message>),
}

impl Name for Referenced {
    fn name(&self) -> &str {
        match self {
            Referenced::Struct(v) => &v.name,
            Referenced::Message(v) => &v.name,
        }
    }
}

impl PtrKey for Referenced {
    fn ptr_key(&self) -> usize {
        match self {
            Referenced::Struct(v) => v.ptr_key(),
            Referenced::Message(v) => v.ptr_key(),
        }
    }
}

impl Referenced {
    pub fn lookup(proto: &Protocol, reference_name: &str) -> Option<Self> {
        proto
            .structs
            .get(reference_name)
            .map(|v| Referenced::Struct(v.clone()))
            .or_else(|| proto.messages.get(reference_name).map(|v| Referenced::Message(v.clone())))
    }
}

#[derive(Clone, Debug)]
pub struct FixedContainerField {
    pub ty: FixedFieldType,
    pub item_type: Arc<Structure>,
}

impl Display for FixedContainerField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FixedContainer<{}, Len = {}>", self.item_type.name, self.ty)
    }
}

#[derive(Clone, Debug)]
pub struct SizedBufferField {
    pub ty: FixedFieldType,
}

impl Display for SizedBufferField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Varbuf<{}>", self.ty)
    }
}

#[derive(Clone, Debug)]
pub struct ContainerField {
    pub ty: FixedFieldType,
    pub item_type: Arc<Message>,
    pub nested: bool,
}

impl Display for ContainerField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Container<{}, Len = {}>", self.item_type.name(), self.ty)
    }
}

#[derive(Clone, Debug)]
pub struct SizedContainerField {
    pub ty: FixedFieldType,
    pub item_type: Arc<Message>,
    pub size_ty: FixedFieldType,
}

impl Display for SizedContainerField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SizedContainer<{}, Len = {}, Size = {}>",
            self.item_type.name(),
            self.ty,
            self.size_ty
        )
    }
}

#[derive(Clone, Debug)]
pub struct FixedField {
    pub ty: FixedFieldType,
}

impl Display for FixedField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ty)
    }
}

#[derive(Clone, Debug)]
pub struct UnionField {
    pub r: Arc<Union>,
}

impl Display for UnionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.r.name())
    }
}

#[derive(Clone, Debug)]
pub enum FieldType {
    Fixed(FixedField),
    Ref(Referenced),

    /// A buffer field is a field which has a known size which can be determined at runtime
    /// (ex: a null-terminated string).
    Buffer,

    /// A sized buffer field is a field which has a known size field based on a configurable type
    /// (ex: a Varchar).
    SizedBuffer(SizedBufferField),

    /// A fixed container is a container which can store only fixed size elements (structures).
    FixedContainer(FixedContainerField),

    Union(UnionField),

    /// A container can store dynamically sized elements.
    Container(ContainerField),

    /// A container which has an additional configurable size field to detect the size in bytes of
    /// the container.
    SizedContainer(SizedContainerField),

    Payload,
}

impl Display for FieldType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::Fixed(v) => v.fmt(f),
            FieldType::Ref(v) => f.write_str(v.name()),
            FieldType::Buffer => f.write_str("Buffer"),
            FieldType::SizedBuffer(v) => v.fmt(f),
            FieldType::FixedContainer(v) => v.fmt(f),
            FieldType::Union(v) => v.fmt(f),
            FieldType::Container(v) => v.fmt(f),
            FieldType::SizedContainer(v) => v.fmt(f),
            FieldType::Payload => f.write_str("Bytes"),
        }
    }
}

impl FieldType {
    pub fn is_message_reference(&self) -> bool {
        match self {
            FieldType::Ref(v) => match v {
                Referenced::Struct(_) => false,
                Referenced::Message(_) => true,
            },
            _ => false,
        }
    }

    pub fn is_union(&self) -> bool {
        matches!(self, FieldType::Union(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, FieldType::SizedBuffer(_) | FieldType::Buffer)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SizeInfo {
    pub is_dyn_sized: bool,
    pub is_element_dyn_sized: bool,
}

#[derive(Clone, Debug)]
pub struct HeaderField {
    pub name: String,
    pub index: usize,
}

impl HeaderField {
    fn from_model(header: Option<String>, fields: &[Field]) -> Result<(Option<Self>, Option<&Field>), Error> {
        match header {
            Some(header) => {
                let (index, field) = fields
                    .iter()
                    .enumerate()
                    .find_map(|(k, v)| if v.name == header { Some((k, v)) } else { None })
                    .ok_or(Error::UndefinedReference(header))?;
                let header = HeaderField {
                    index,
                    name: field.name.clone(),
                };
                Ok((Some(header), Some(field)))
            }
            None => Ok((None, None)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub header: Option<HeaderField>,
    pub ty: FieldType,
    pub optional: bool,
    pub size: SizeInfo,
    pub endianness: Endianness,
    pub description: Option<Description>,
    pub codec: Option<String>,
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (self.optional, &self.codec) {
            (true, None) => write!(f, "{}: {}?, {} endian", self.name, self.ty, self.endianness),
            (false, None) => write!(f, "{}: {}, {} endian", self.name, self.ty, self.endianness),
            (true, Some(v)) => write!(f, "{}: {} ({})?, {} endian", self.name, v, self.ty, self.endianness),
            (false, Some(v)) => write!(f, "{}: {} ({}), {} endian", self.name, v, self.ty, self.endianness),
        }
    }
}

impl Field {
    pub fn codec(&self) -> &str {
        self.codec.as_deref().unwrap_or("base")
    }

    fn from_model(
        proto: &Protocol,
        unsorted: &[Field],
        has_headers: bool,
        value: crate::model::message::MessageField,
    ) -> Result<Self, Error> {
        if (value.value.is_none() && value.item_type.is_none()) || (value.value.is_some() && value.item_type.is_some())
        {
            return Err(Error::BadFieldType);
        }
        let (header, header_field) = HeaderField::from_model(value.header, unsorted)?;
        if let Some(field) = header_field {
            match &field.ty {
                FieldType::Ref(Referenced::Struct(v)) => v.set_used_in_header(),
                v => {
                    error!(
                        "Invalid header field type, expected struct reference, got field type {:?}",
                        v
                    );
                    return Err(Error::InvalidHeaderType);
                }
            }
        }
        let builder = FieldBuilder::new(value.name, value.optional.unwrap_or_default(), proto.endianness)
            .description(value.description)
            .header(header)
            .codec(value.codec);
        if let Some(info) = value.value {
            match info {
                MessageFieldValue::List {
                    max_len,
                    item_type,
                    max_size,
                    nested,
                } => {
                    if max_len == 0 {
                        return Err(Error::ZeroArray);
                    }
                    if builder.has_codec() {
                        return Err(Error::ForbiddenCodec(builder.name().into()));
                    }
                    let r = Referenced::lookup(proto, &item_type).ok_or(Error::UndefinedReference(item_type))?;
                    let ty = FixedFieldType::from_max_value(max_len)?;
                    match r {
                        Referenced::Struct(item_type) => Ok(builder
                            .codec(Some("list".into()))
                            .build(FieldType::FixedContainer(FixedContainerField { item_type, ty }))),
                        Referenced::Message(item_type) => {
                            if let Some(max_size) = max_size {
                                if max_size == 0 {
                                    return Err(Error::ZeroArray);
                                }
                                let size_ty = FixedFieldType::from_max_value(max_size)?;
                                Ok(builder
                                    .codec(Some("list".into()))
                                    .size_info(SizeInfo {
                                        is_element_dyn_sized: false,
                                        is_dyn_sized: true,
                                    })
                                    .build(FieldType::SizedContainer(SizedContainerField {
                                        ty,
                                        item_type,
                                        size_ty,
                                    })))
                            } else {
                                item_type.embedded.set(true);
                                Ok(
                                    builder.codec(Some("list".into())).dynamic_size().build(FieldType::Container(
                                        ContainerField {
                                            ty,
                                            item_type,
                                            nested: nested.unwrap_or_default(),
                                        },
                                    )),
                                )
                            }
                        }
                    }
                }
                MessageFieldValue::Container {
                    max_len,
                    item_type,
                    max_size,
                    nested,
                } => {
                    if max_len == 0 {
                        return Err(Error::ZeroArray);
                    }
                    let r = Referenced::lookup(proto, &item_type).ok_or(Error::UndefinedReference(item_type))?;
                    let ty = FixedFieldType::from_max_value(max_len)?;
                    match r {
                        Referenced::Struct(item_type) => {
                            Ok(builder.build(FieldType::FixedContainer(FixedContainerField { item_type, ty })))
                        }
                        Referenced::Message(item_type) => {
                            if let Some(max_size) = max_size {
                                if max_size == 0 {
                                    return Err(Error::ZeroArray);
                                }
                                let size_ty = FixedFieldType::from_max_value(max_size)?;
                                Ok(builder
                                    .size_info(SizeInfo {
                                        is_element_dyn_sized: false,
                                        is_dyn_sized: true,
                                    })
                                    .build(FieldType::SizedContainer(SizedContainerField {
                                        ty,
                                        item_type,
                                        size_ty,
                                    })))
                            } else {
                                item_type.embedded.set(true);
                                Ok(builder.dynamic_size().build(FieldType::Container(ContainerField {
                                    ty,
                                    item_type,
                                    nested: nested.unwrap_or_default(),
                                })))
                            }
                        }
                    }
                }
                MessageFieldValue::String { max_len } => {
                    if builder.has_codec() {
                        return Err(Error::ForbiddenCodec(builder.name().into()));
                    }
                    match max_len {
                        None => Ok(builder.codec(Some("string".into())).build(FieldType::Buffer)),
                        Some(max_len) => {
                            if max_len == 0 {
                                return Err(Error::ZeroArray);
                            }
                            let ty = FixedFieldType::from_max_value(max_len)?;
                            Ok(builder
                                .codec(Some("string".into()))
                                .build(FieldType::SizedBuffer(SizedBufferField { ty })))
                        }
                    }
                }
                MessageFieldValue::Buffer { max_len } => match max_len {
                    None => Ok(builder.build(FieldType::Buffer)),
                    Some(max_len) => {
                        if max_len == 0 {
                            return Err(Error::ZeroArray);
                        }
                        let ty = FixedFieldType::from_max_value(max_len)?;
                        Ok(builder.build(FieldType::SizedBuffer(SizedBufferField { ty })))
                    }
                },
                MessageFieldValue::Union { name } => {
                    let r = proto.unions.get(&name).ok_or(Error::UndefinedReference(name))?;
                    let header_field = header_field.ok_or(Error::MissingHeaderForUnion)?;
                    match &header_field.ty {
                        FieldType::Ref(Referenced::Struct(v)) => {
                            if !Arc::ptr_eq(&r.discriminant.root, v) {
                                error!(
                                    "Union discriminant type mismatch, expected {}, got {}",
                                    v.name, r.discriminant.root.name
                                );
                                return Err(Error::UnionTypeMismatch);
                            }
                        }
                        _ => unreachable!(),
                    }
                    if value.optional.unwrap_or_default() {
                        eprintln!("WARNING: ignoring unsupported optional flag on union message field!");
                    }
                    Ok(builder.size_info(r.size).build(FieldType::Union(UnionField { r: r.clone() })))
                }
                MessageFieldValue::Payload => Ok(builder.dynamic_size().build(FieldType::Payload)),
                MessageFieldValue::Unsigned { bits } => {
                    let ty = FixedFieldType::from_model(StructFieldRaw::Unsigned { bits })?;
                    Ok(builder.fixed_size().build(FieldType::Fixed(FixedField { ty })))
                }
            }
        } else {
            let item_type = unsafe { value.item_type.unwrap_unchecked() };
            let r = Referenced::lookup(proto, &item_type).ok_or(Error::UndefinedReference(item_type))?;
            match r {
                Referenced::Struct(r) => {
                    let is_single = r.fields.len() == 1;
                    let is_fixed = r.fields[0].ty.as_fixed().is_some();
                    let is_none = r.fields[0].ty.as_fixed().map(|v| v.raw.is_none()).unwrap_or_default();
                    let is_byte_aligned = r.fields[0].loc.bit_size % 8 == 0;
                    let view_is_none = r.fields[0].ty.as_fixed().map(|v| v.view.is_none()).unwrap_or_default();
                    trace!({has_headers} {is_single} {is_fixed} {is_none} {is_byte_aligned}, "Found struct reference: {}", r.name);
                    if !has_headers && is_single && is_fixed && is_none && is_byte_aligned && view_is_none {
                        let fixed = unsafe { r.fields[0].ty.as_fixed().unwrap_unchecked() };
                        Ok(builder.fixed_size().build(FieldType::Fixed(FixedField { ty: fixed.bits_type })))
                    } else {
                        Ok(builder.fixed_size().build(FieldType::Ref(Referenced::Struct(r))))
                    }
                }
                Referenced::Message(r) => Ok(builder.size_info(r.size).build(FieldType::Ref(Referenced::Message(r)))),
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Message {
    pub name: String,
    pub ty: Option<String>,
    pub description: Option<Description>,
    pub fields: Vec<Field>,
    pub size: SizeInfo,
    embedded: Cell<bool>,
}

impl Message {
    pub(crate) fn is_embedded(&self) -> bool {
        self.embedded.get()
    }

    pub fn from_model(proto: &Protocol, value: crate::model::message::Message) -> Result<Message, Error> {
        let mut fields = Vec::with_capacity(value.fields.len());
        let mut dyn_sized_elem_count = 0;
        let mut is_dyn_sized = false;
        let has_headers = value.fields.iter().any(|v| v.header.is_some());
        for v in value.fields {
            let field = Field::from_model(proto, &fields, has_headers, v)?;
            if field.size.is_dyn_sized {
                is_dyn_sized = true;
            }
            if value.ty.is_none()
                && dyn_sized_elem_count > 0
                && (field.size.is_dyn_sized || field.size.is_element_dyn_sized)
            {
                return Err(Error::VarsizeAfterPayload);
            }
            if field.size.is_element_dyn_sized {
                dyn_sized_elem_count += 1;
            }
            if value.ty.is_none() && dyn_sized_elem_count > 1 {
                return Err(Error::MultiPayload);
            }
            fields.push(field);
        }
        Ok(Message {
            name: value.name,
            ty: value.ty,
            description: value.description,
            fields,
            size: SizeInfo {
                is_dyn_sized,
                is_element_dyn_sized: dyn_sized_elem_count > 0,
            },
            embedded: Cell::new(false),
        })
    }
}

name_index!(Message => name);
