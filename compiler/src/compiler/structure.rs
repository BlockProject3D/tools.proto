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
use crate::model::structure::StructFieldType;

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

impl FixedFieldType {
    pub fn to_unsigned_integer(self) -> FixedFieldType {
        match self {
            FixedFieldType::Int8 => FixedFieldType::UInt8,
            FixedFieldType::Int16 => FixedFieldType::UInt16,
            FixedFieldType::Int32 => FixedFieldType::UInt32,
            FixedFieldType::Int64 => FixedFieldType::UInt64,
            FixedFieldType::Float32 => FixedFieldType::UInt32,
            FixedFieldType::Float64 => FixedFieldType::UInt64,
            FixedFieldType::Bool => FixedFieldType::UInt8,
            v => v
        }
    }

    pub fn from_max_value(max_value: usize) -> Result<Self, CompilerError> {
        let bit_size = if max_value > u32::MAX as usize {
            64
        } else if max_value > u16::MAX as usize {
            32
        } else if max_value > u8::MAX as usize {
            16
        } else {
            8
        };
        Self::from_model(StructFieldType::Unsigned, bit_size)
    }

    pub fn from_model(ty: StructFieldType, bit_size: usize) -> Result<Self, CompilerError> {
        if ty == StructFieldType::Boolean {
            Ok(Self::Bool)
        } else if ty == StructFieldType::Float && bit_size == 32 {
            Ok(Self::Float32)
        } else if ty == StructFieldType::Float && bit_size == 64 {
            Ok(Self::Float64)
        } else if bit_size > 32 && bit_size <= 64 {
            match ty {
                StructFieldType::Integer => Ok(Self::Int64),
                StructFieldType::Unsigned => Ok(Self::UInt64),
                StructFieldType::Float => Ok(Self::Float64),
                _ => Err(CompilerError::UnsupportedType(ty))
            }
        } else if bit_size > 16 && bit_size <= 32 {
            match ty {
                StructFieldType::Integer => Ok(Self::Int32),
                StructFieldType::Unsigned => Ok(Self::UInt32),
                StructFieldType::Float => Ok(Self::Float64),
                _ => Err(CompilerError::UnsupportedType(ty))
            }
        } else if bit_size > 8 && bit_size <= 16 {
            match ty {
                StructFieldType::Integer => Ok(Self::Int16),
                StructFieldType::Unsigned => Ok(Self::UInt16),
                StructFieldType::Float => Ok(Self::Float32),
                _ => Err(CompilerError::UnsupportedType(ty))
            }
        } else if bit_size > 0 && bit_size <= 8 {
            match ty {
                StructFieldType::Integer => Ok(Self::Int8),
                StructFieldType::Unsigned => Ok(Self::UInt8),
                StructFieldType::Float => Ok(Self::Float32),
                _ => Err(CompilerError::UnsupportedType(ty))
            }
        } else {
            Err(CompilerError::UnsupportedBitSize(bit_size))
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
    pub fn from_model(bit_size: usize, bit_offset: usize) -> Self {
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
}

#[derive(Clone, Debug)]
pub struct FixedField {
    pub name: String,
    pub ty: FixedFieldType,
    pub loc: Location
}

#[derive(Clone, Debug)]
pub struct FixedArrayField {
    pub name: String,
    pub ty: FixedFieldType,
    pub array_len: usize,
    pub loc: Location
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

    fn from_model(proto: &Protocol, last_bit_offset: usize, value: crate::model::structure::StructField) -> Result<(Self, usize), CompilerError> {
        match value.info {
            StructFieldType::Struct { item_type } => {
                let r = proto.structs_by_name.get(&item_type).ok_or_else(|| CompilerError::UndefinedReference(item_type))?;
                Ok((Self::Struct(StructField {
                    name: value.name,
                    r: r.clone(),
                    loc: Location::from_model(r.bit_size, last_bit_offset)
                }), last_bit_offset + r.bit_size))
            },
            _ => {
                let bit_size = value.bits.ok_or(CompilerError::MissingBitSize)?;
                let array_len = value.array_len.unwrap_or(1);
                let ty = FixedFieldType::from_model(value.info, bit_size)?;
                let loc = Location::from_model(bit_size * array_len, last_bit_offset);
                if array_len > 1 {
                    if bit_size % 8 != 0 {
                        return Err(CompilerError::UnalignedArrayCodec);
                    }
                    Ok((Self::Array(FixedArrayField {
                        name: value.name,
                        array_len,
                        ty,
                        loc
                    }), last_bit_offset + bit_size))
                } else {
                    Ok((Self::Fixed(FixedField {
                        name: value.name,
                        ty,
                        loc
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
    pub fn from_model(proto: &Protocol, value: crate::model::structure::Structure) -> Result<Structure, CompilerError> {
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
            fields: fields.collect::<Result<Vec<Field>, CompilerError>>()?,
            bit_size: last_bit_offset,
            byte_size: if last_bit_offset % 8 != 0 {
                (last_bit_offset / 8) + 1
            } else {
                last_bit_offset / 8
            }
        })
    }
}
