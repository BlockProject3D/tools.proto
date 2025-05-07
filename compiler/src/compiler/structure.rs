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

use crate::compiler::error::Error;
use crate::compiler::r#enum::Enum;
use crate::compiler::util::objects::name_index;
use crate::compiler::util::try2;
use crate::compiler::util::types::Name;
use crate::compiler::Protocol;
use crate::model::protocol::{Description, Endianness};
use crate::model::structure::{SimpleType, StructFieldRaw, StructFieldView};
use bp3d_debug::trace;
use std::cell::Cell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FixedFieldType {
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    Bool,
}

impl FixedFieldType {
    pub fn get_aligned_bit_size(&self) -> usize {
        match self {
            FixedFieldType::Int8 => 8,
            FixedFieldType::Int16 => 16,
            FixedFieldType::Int32 => 32,
            FixedFieldType::Int64 => 64,
            FixedFieldType::UInt8 => 8,
            FixedFieldType::UInt16 => 16,
            FixedFieldType::UInt32 => 32,
            FixedFieldType::UInt64 => 64,
            FixedFieldType::Float32 => 32,
            FixedFieldType::Float64 => 64,
            FixedFieldType::Bool => 8,
        }
    }

    pub fn is_unsigned(&self) -> bool {
        match self {
            FixedFieldType::Int8 => false,
            FixedFieldType::Int16 => false,
            FixedFieldType::Int32 => false,
            FixedFieldType::Int64 => false,
            FixedFieldType::UInt8 => true,
            FixedFieldType::UInt16 => true,
            FixedFieldType::UInt32 => true,
            FixedFieldType::UInt64 => true,
            FixedFieldType::Float32 => false,
            FixedFieldType::Float64 => false,
            FixedFieldType::Bool => false,
        }
    }

    pub fn is_signed(&self) -> bool {
        match self {
            FixedFieldType::Int8 => true,
            FixedFieldType::Int16 => true,
            FixedFieldType::Int32 => true,
            FixedFieldType::Int64 => true,
            FixedFieldType::UInt8 => false,
            FixedFieldType::UInt16 => false,
            FixedFieldType::UInt32 => false,
            FixedFieldType::UInt64 => false,
            FixedFieldType::Float32 => false,
            FixedFieldType::Float64 => false,
            FixedFieldType::Bool => false,
        }
    }
}

impl Display for FixedFieldType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FixedFieldType::Int8 => f.write_str("Int8"),
            FixedFieldType::Int16 => f.write_str("Int16"),
            FixedFieldType::Int32 => f.write_str("Int32"),
            FixedFieldType::Int64 => f.write_str("Int64"),
            FixedFieldType::UInt8 => f.write_str("UInt8"),
            FixedFieldType::UInt16 => f.write_str("UInt16"),
            FixedFieldType::UInt32 => f.write_str("UInt32"),
            FixedFieldType::UInt64 => f.write_str("UInt64"),
            FixedFieldType::Float32 => f.write_str("Float32"),
            FixedFieldType::Float64 => f.write_str("Float64"),
            FixedFieldType::Bool => f.write_str("Bool"),
        }
    }
}

fn map_numeric(
    ty: SimpleType,
    signed: FixedFieldType,
    unsigned: FixedFieldType,
    float: FixedFieldType,
) -> Option<FixedFieldType> {
    match ty {
        SimpleType::Signed => Some(signed),
        SimpleType::Unsigned => Some(unsigned),
        SimpleType::Float => Some(float),
        _ => None,
    }
}

impl FixedFieldType {
    pub fn get_byte_size(&self) -> usize {
        match self {
            FixedFieldType::Int8 => 1,
            FixedFieldType::Int16 => 2,
            FixedFieldType::Int32 => 4,
            FixedFieldType::Int64 => 8,
            FixedFieldType::UInt8 => 1,
            FixedFieldType::UInt16 => 2,
            FixedFieldType::UInt32 => 4,
            FixedFieldType::UInt64 => 8,
            FixedFieldType::Float32 => 4,
            FixedFieldType::Float64 => 8,
            FixedFieldType::Bool => 1,
        }
    }

    pub fn from_min_max_value(min_value: isize, max_value: isize) -> Result<Self, Error> {
        match min_value < 0 {
            true => {
                let bit_size = if max_value > i32::MAX as isize || min_value < i32::MIN as isize {
                    64
                } else if max_value > i16::MAX as isize || min_value < i16::MIN as isize {
                    32
                } else if max_value > i8::MAX as isize || min_value < i8::MIN as isize {
                    16
                } else {
                    8
                };
                Self::from_model(StructFieldRaw::Signed { bits: bit_size })
            }
            false => {
                let bit_size = if max_value > u32::MAX as isize {
                    64
                } else if max_value > u16::MAX as isize {
                    32
                } else if max_value > u8::MAX as isize {
                    16
                } else {
                    8
                };
                Self::from_model(StructFieldRaw::Unsigned { bits: bit_size })
            }
        }
    }

    pub fn from_max_value(max_value: usize) -> Result<Self, Error> {
        let bit_size = if max_value > u32::MAX as usize {
            64
        } else if max_value > u16::MAX as usize {
            32
        } else if max_value > u8::MAX as usize {
            16
        } else {
            8
        };
        Self::from_model(StructFieldRaw::Unsigned { bits: bit_size })
    }

    pub fn from_model(ty1: StructFieldRaw) -> Result<Self, Error> {
        let motherfuckingrust = ty1.clone();
        let ty = ty1.get_simple_type();
        let bit_size = ty1.get_bit_size();
        if ty == SimpleType::Boolean {
            Ok(Self::Bool)
        } else if ty == SimpleType::Float && bit_size == 32 {
            Ok(Self::Float32)
        } else if ty == SimpleType::Float && bit_size == 64 {
            Ok(Self::Float64)
        } else if bit_size > 32 && bit_size <= 64 {
            map_numeric(ty, Self::Int64, Self::UInt64, Self::Float64).ok_or(Error::UnsupportedType(motherfuckingrust))
        } else if bit_size > 16 && bit_size <= 32 {
            map_numeric(ty, Self::Int32, Self::UInt32, Self::Float64).ok_or(Error::UnsupportedType(motherfuckingrust))
        } else if bit_size > 8 && bit_size <= 16 {
            map_numeric(ty, Self::Int16, Self::UInt16, Self::Float32).ok_or(Error::UnsupportedType(motherfuckingrust))
        } else if bit_size > 0 && bit_size <= 8 {
            map_numeric(ty, Self::Int8, Self::UInt8, Self::Float32).ok_or(Error::UnsupportedType(motherfuckingrust))
        } else {
            Err(Error::UnsupportedBitSize(bit_size))
        }
    }
}

#[derive(Clone, Debug)]
pub struct Location {
    pub byte_offset: usize,
    pub bit_offset: usize,
    pub byte_size: usize,
    pub bit_size: usize,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "bytes {}..{}, bits {}..{}",
            self.byte_offset,
            self.byte_offset + self.byte_size,
            self.bit_offset,
            self.bit_offset + self.bit_size
        )
    }
}

impl Location {
    fn from_model(bit_size: usize, bit_offset: usize) -> Self {
        let byte_offset = bit_offset / 8;
        Self {
            byte_offset,
            bit_offset: bit_offset - byte_offset * 8,
            bit_size,
            byte_size: if bit_size % 8 != 0 {
                (bit_size / 8) + 1
            } else {
                bit_size / 8
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum FieldRaw {
    /// Apply a raw C-like cast (used for unsigned > signed and unsigned > float of same bit size).
    Transmute,

    /// Apply a unsigned > signed cast on a non T-aligned value,
    /// the maximum positive value is passed in.
    SignedCast(usize),

    /// Don't do anything special, just return the raw value.
    None,
}

impl FieldRaw {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    fn from_model(ty: SimpleType, bit_size: usize) -> FieldRaw {
        if ty == SimpleType::Float && bit_size != 32 && bit_size != 64 {
            return FieldRaw::None;
        }
        if ty == SimpleType::Signed && bit_size != 8 && bit_size != 16 && bit_size != 32 && bit_size != 64 {
            let max_value = 1 << (bit_size - 1);
            FieldRaw::SignedCast(max_value - 1)
        } else if ty == SimpleType::Unsigned {
            FieldRaw::None
        } else {
            FieldRaw::Transmute
        }
    }
}

#[derive(Clone, Debug)]
pub enum FieldView {
    /// Apply a float view based on an affine transformation function.
    Float { a: f64, b: f64, a_inv: f64, b_inv: f64 },

    /// Apply an enum view.
    Enum(Rc<Enum>),

    /// Don't do anything special, just return the raw value.
    None,
}

impl FieldView {
    fn from_model(
        proto: &Protocol,
        ty: SimpleType,
        bit_size: usize,
        value: Option<StructFieldView>,
    ) -> Result<Self, Error> {
        match value {
            Some(StructFieldView::Enum { name }) => {
                if ty != SimpleType::Unsigned && ty != SimpleType::Signed {
                    return Err(Error::UnsupportedViewType(ty));
                }
                let r = proto.enums.get(&name).ok_or(Error::UndefinedReference(name))?;
                Ok(FieldView::Enum(r.clone()))
            }
            Some(StructFieldView::FloatRange { min, max }) => {
                if ty != SimpleType::Float {
                    return Err(Error::UnsupportedViewType(ty));
                }
                let raw_max: usize = (1 << bit_size) - 1;
                let a = max / (raw_max as f64);
                let b = min;
                let a_inv = 1.0 / a;
                let b_inv = -b;
                Ok(FieldView::Float { a, b, a_inv, b_inv })
            }
            Some(StructFieldView::FloatMultiplier { multiplier }) => {
                if ty != SimpleType::Float {
                    return Err(Error::UnsupportedViewType(ty));
                }
                let a = multiplier;
                let b = 0.0;
                let a_inv = 1.0 / a;
                let b_inv = 0.0;
                Ok(FieldView::Float { a, b, a_inv, b_inv })
            }
            None => {
                if ty == SimpleType::Float && bit_size != 32 && bit_size != 64 {
                    return Err(Error::UnsupportedViewType(ty));
                }
                Ok(FieldView::None)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct FixedField {
    pub bits_type: FixedFieldType,
    pub raw_type: FixedFieldType,
    pub view_type: FixedFieldType,
    pub raw: FieldRaw,
    pub view: FieldView,
    pub endianness: Endianness,
}

impl Display for FixedField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {} endian", self.bits_type, self.endianness)
    }
}

#[derive(Clone, Debug)]
pub struct FixedArrayField {
    pub ty: FixedFieldType,
    pub array_len: usize,
    pub endianness: Endianness,
    pub item_bit_size: usize,
}

impl Display for FixedArrayField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}], {} endian", self.ty, self.array_len, self.endianness)
    }
}

#[derive(Clone, Debug)]
pub enum FieldType {
    Fixed(FixedField),
    Array(FixedArrayField),
    Struct(Rc<Structure>),
}

impl Display for FieldType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::Fixed(v) => v.fmt(f),
            FieldType::Array(v) => v.fmt(f),
            FieldType::Struct(v) => f.write_str(v.name()),
        }
    }
}

impl FieldType {
    pub fn as_fixed(&self) -> Option<&FixedField> {
        match self {
            FieldType::Fixed(v) => Some(v),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub loc: Location,
    pub description: Option<Description>,
    pub ty: FieldType,
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} ({})", self.name, self.ty, self.loc)
    }
}

impl Field {
    fn from_model(
        proto: &Protocol,
        fields: &[Field],
        mut last_bit_offset: usize,
        value: crate::model::structure::StructField,
    ) -> Result<(Self, usize), Error> {
        if (value.raw.is_none() && value.item_type.is_none()) || (value.raw.is_some() && value.item_type.is_some()) {
            return Err(Error::BadFieldType);
        }
        let (ty, bit_size) = if let Some(info) = value.raw {
            let array_len = value.array_len.unwrap_or(1);
            if array_len == 0 {
                return Err(Error::ZeroArray);
            }
            let mut bit_size = info.get_bit_size();
            let view = FieldView::from_model(proto, info.get_simple_type(), bit_size, value.view)?;
            let raw = FieldRaw::from_model(info.get_simple_type(), bit_size);
            bit_size *= array_len;
            let ty = FixedFieldType::from_model(info)?;
            if array_len > 1 {
                if (bit_size / array_len) % 8 != 0 {
                    return Err(Error::UnalignedArrayCodec);
                }
                (
                    FieldType::Array(FixedArrayField {
                        endianness: proto.endianness,
                        array_len,
                        ty,
                        item_bit_size: bit_size / array_len,
                    }),
                    bit_size,
                )
            } else {
                let bits_type = FixedFieldType::from_model(StructFieldRaw::Unsigned { bits: bit_size })?;
                let raw_type = match (bit_size, ty) {
                    (32, FixedFieldType::Float32) => FixedFieldType::Float32,
                    (64, FixedFieldType::Float64) => FixedFieldType::Float64,
                    (_, FixedFieldType::Float32) => bits_type,
                    (_, FixedFieldType::Float64) => bits_type,
                    _ => ty,
                };
                (
                    FieldType::Fixed(FixedField {
                        endianness: proto.endianness,
                        bits_type,
                        raw_type,
                        view_type: ty,
                        view,
                        raw,
                    }),
                    bit_size,
                )
            }
        } else {
            let item_type = unsafe { value.item_type.unwrap_unchecked() };
            let r = try2!(proto.structs.get(&item_type) => Error::UndefinedReference(item_type));
            trace!("Solved reference {} => {:?}", item_type, r);
            (FieldType::Struct(r.clone()), r.bit_size)
        };
        let loc = match value.offset {
            None => {
                let loc = Location::from_model(bit_size, last_bit_offset);
                last_bit_offset += bit_size;
                loc
            }
            Some(v) => match v.relative_to {
                None => {
                    let start_bits = v.bits.unwrap_or(0);
                    let end_bits = start_bits + bit_size;
                    if end_bits > last_bit_offset {
                        last_bit_offset = end_bits;
                    }
                    Location::from_model(bit_size, start_bits)
                }
                Some(name) => {
                    let field = fields.iter().find(|v| v.name == name).ok_or(Error::UndefinedReference(name))?;
                    let start_bits = field.loc.bit_offset + v.bits.unwrap_or(0);
                    let end_bits = start_bits + bit_size;
                    if end_bits > last_bit_offset {
                        last_bit_offset = end_bits;
                    }
                    Location::from_model(bit_size, start_bits)
                }
            },
        };
        Ok((
            Self {
                name: value.name,
                ty,
                loc,
                description: value.description,
            },
            last_bit_offset,
        ))
    }
}

#[derive(Clone, Debug)]
pub struct Structure {
    pub name: String,
    pub description: Option<Description>,
    pub fields: Vec<Field>,
    pub byte_size: usize,
    pub bit_size: usize,
    used_in_header: Cell<bool>,
}

impl Structure {
    pub fn set_used_in_header(&self) {
        self.used_in_header.set(true);
    }

    pub fn is_used_in_header(&self) -> bool {
        self.used_in_header.get()
    }

    pub fn from_model(proto: &Protocol, value: crate::model::structure::Structure) -> Result<Structure, Error> {
        let mut fields = Vec::with_capacity(value.fields.len());
        let mut last_bit_offset = 0;
        for field in value.fields {
            let (field, new_offset) = Field::from_model(proto, &fields, last_bit_offset, field)?;
            fields.push(field);
            last_bit_offset = new_offset;
        }
        let s = Structure {
            name: value.name,
            description: value.description,
            fields,
            bit_size: last_bit_offset,
            byte_size: if last_bit_offset % 8 != 0 {
                (last_bit_offset / 8) + 1
            } else {
                last_bit_offset / 8
            },
            used_in_header: Cell::new(false),
        };
        if s.bit_size == 0 {
            return Err(Error::ZeroStruct);
        }
        Ok(s)
    }
}

name_index!(Structure => name);
