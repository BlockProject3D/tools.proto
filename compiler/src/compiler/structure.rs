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
use crate::compiler::r#enum::Enum;
use crate::model::protocol::Endianness;
use crate::model::structure::{SimpleType, StructFieldType, StructFieldView};

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
    Bool
}

fn map_numeric(ty: SimpleType, signed: FixedFieldType, unsigned: FixedFieldType, float: FixedFieldType) -> Option<FixedFieldType> {
    match ty {
        SimpleType::Signed => Some(signed),
        SimpleType::Unsigned => Some(unsigned),
        SimpleType::Float => Some(float),
        _ => None
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
            FixedFieldType::Bool => 1
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
        Self::from_model(StructFieldType::Unsigned { bits: bit_size })
    }

    pub fn from_model(ty1: StructFieldType) -> Result<Self, Error> {
        let motherfuckingrust = ty1.clone();
        let ty = ty1.get_simple_type();
        let bit_size = ty1.get_bit_size().ok_or(Error::UnsupportedType(ty1))?;
        if ty == SimpleType::Boolean {
            Ok(Self::Bool)
        } else if ty == SimpleType::Float && bit_size == 32 {
            Ok(Self::Float32)
        } else if ty == SimpleType::Float && bit_size == 64 {
            Ok(Self::Float64)
        } else if bit_size > 32 && bit_size <= 64 {
            map_numeric(ty, Self::Int64, Self::UInt64, Self::Float64)
                .ok_or_else(|| Error::UnsupportedType(motherfuckingrust))
        } else if bit_size > 16 && bit_size <= 32 {
            map_numeric(ty, Self::Int32, Self::UInt32, Self::Float64)
                .ok_or_else(|| Error::UnsupportedType(motherfuckingrust))
        } else if bit_size > 8 && bit_size <= 16 {
            map_numeric(ty, Self::Int16, Self::UInt16, Self::Float32)
                .ok_or_else(|| Error::UnsupportedType(motherfuckingrust))
        } else if bit_size > 0 && bit_size <= 8 {
            map_numeric(ty, Self::Int8, Self::UInt8, Self::Float32)
                .ok_or_else(|| Error::UnsupportedType(motherfuckingrust))
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
    pub bit_size: usize
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
            }
        }
    }

    pub fn get_unsigned_integer_type(&self) -> FixedFieldType {
        FixedFieldType::from_model(StructFieldType::Unsigned { bits: self.bit_size }).unwrap()
    }
}

#[derive(Clone, Debug)]
pub enum FieldView {
    /// Apply a float view based on an affine transformation function.
    Float {
        a: f64,
        b: f64,
        a_inv: f64,
        b_inv: f64
    },

    /// Apply an enum view.
    Enum(Rc<Enum>),

    /// Apply a raw C-like cast (used for unsigned > signed and unsigned > float of same bit size).
    Transmute,

    /// Apply a unsigned > signed cast on a non T-aligned value,
    /// the maximum positive value is passed in.
    SignedCast(usize),

    /// Don't do anything special, just return the raw value.
    None
}

impl FieldView {
    pub fn is_transmute(&self) -> bool {
        match self {
            FieldView::Transmute => true,
            FieldView::None => true,
            _ => false
        }
    }

    fn from_model(proto: &Protocol, ty: SimpleType, bit_size: usize, value: Option<StructFieldView>) -> Result<Self, Error> {
        match value {
            Some(StructFieldView::Enum { name }) => {
                if ty != SimpleType::Unsigned {
                    return Err(Error::UnsupportedViewType(ty));
                }
                let r = proto.enums_by_name.get(&name).ok_or_else(|| Error::UndefinedReference(name))?;
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
                if ty == SimpleType::Signed && bit_size != 8 && bit_size != 16 && bit_size != 32 && bit_size != 64 {
                    let max_value = 1 << (bit_size - 1);
                    Ok(FieldView::SignedCast(max_value - 1))
                } else if ty == SimpleType::Unsigned {
                    Ok(FieldView::None)
                } else {
                    Ok(FieldView::Transmute)
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct FixedField {
    pub name: String,
    pub ty: FixedFieldType,
    pub loc: Location,
    pub view: FieldView,
    pub endianness: Endianness
}

#[derive(Clone, Debug)]
pub struct FixedArrayField {
    pub name: String,
    pub ty: FixedFieldType,
    pub array_len: usize,
    pub loc: Location,
    pub endianness: Endianness
}

impl FixedArrayField {
    pub fn item_bit_size(&self) -> usize {
        self.loc.bit_size / self.array_len
    }
}

#[derive(Clone, Debug)]
pub struct StructField {
    pub name: String,
    pub r: Rc<Structure>,
    pub loc: Location
}

#[derive(Clone, Debug)]
pub enum Field {
    Fixed(FixedField),
    Array(FixedArrayField),
    Struct(StructField)
}

impl Field {
    pub fn as_fixed(&self) -> Option<&FixedField> {
        match self {
            Field::Fixed(v) => Some(v),
            _ => None
        }
    }

    pub fn loc(&self) -> &Location {
        match self {
            Field::Fixed(v) => &v.loc,
            Field::Array(v) => &v.loc,
            Field::Struct(v) => &v.loc
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Field::Fixed(v) => &v.name,
            Field::Array(v) => &v.name,
            Field::Struct(v) => &v.name
        }
    }

    fn from_model(proto: &Protocol, last_bit_offset: usize, value: crate::model::structure::StructField) -> Result<(Self, usize), Error> {
        match value.info {
            StructFieldType::Struct { item_type } => {
                let r = proto.structs_by_name.get(&item_type).ok_or_else(|| Error::UndefinedReference(item_type))?;
                Ok((Self::Struct(StructField {
                    name: value.name,
                    r: r.clone(),
                    loc: Location::from_model(r.bit_size, last_bit_offset)
                }), last_bit_offset + r.bit_size))
            },
            _ => {
                let array_len = value.array_len.unwrap_or(1);
                let mut bit_size = value.info.get_bit_size().ok_or(Error::MissingBitSize)?;
                let view = FieldView::from_model(proto, value.info.get_simple_type(), bit_size, value.view)?;
                bit_size *= array_len;
                let ty = FixedFieldType::from_model(value.info)?;
                let loc = Location::from_model(bit_size, last_bit_offset);
                if array_len > 1 {
                    if bit_size % 8 != 0 {
                        return Err(Error::UnalignedArrayCodec);
                    }
                    Ok((Self::Array(FixedArrayField {
                        name: value.name,
                        endianness: proto.endianness,
                        array_len,
                        ty,
                        loc
                    }), last_bit_offset + bit_size))
                } else {
                    Ok((Self::Fixed(FixedField {
                        name: value.name,
                        endianness: proto.endianness,
                        ty,
                        loc,
                        view
                    }), last_bit_offset + bit_size))
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Structure {
    pub name: String,
    pub fields: Vec<Field>,
    pub byte_size: usize,
    pub bit_size: usize
}

impl Structure {
    pub fn from_model(proto: &Protocol, value: crate::model::structure::Structure) -> Result<Structure, Error> {
        let mut last_bit_offset = 0;
        let fields = value.fields.into_iter().map(|v| {
            let res = Field::from_model(proto, last_bit_offset, v);
            if let Ok((_, new_offset)) = res {
                last_bit_offset = new_offset
            }
            res.map(|(field, _)| field)
        });
        Ok(Structure {
            name: value.name,
            fields: fields.collect::<Result<Vec<Field>, Error>>()?,
            bit_size: last_bit_offset,
            byte_size: if last_bit_offset % 8 != 0 {
                (last_bit_offset / 8) + 1
            } else {
                last_bit_offset / 8
            }
        })
    }
}
