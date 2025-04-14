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

use bp3d_protoc::compiler::structure::{Field, FieldRaw, FieldView, FixedFieldType};
use bp3d_protoc::model::protocol::Endianness;
use crate::field::codec::{BitCodecBE, BitCodecLE, ByteCodecBE, ByteCodecLE, Codec};
use crate::field::primitive::{PrimitiveType, Value};
use crate::field::primitive::transform::{Float32Transform, Float64Transform, FloatTransform, NoneTransform, RawTransform, SignedTransform, ViewTransform};

struct Primitive<C: Codec, TRaw: RawTransform, TView: ViewTransform> {
    codec: C,
    raw: TRaw,
    view: TView,
}

impl<C: Codec, TRaw: RawTransform, TView: ViewTransform> Primitive<C, TRaw, TView> {
    pub fn new(codec: C, raw: TRaw, view: TView) -> Self {
        Self {
            codec,
            raw,
            view
        }
    }
}

impl<C: Codec, TRaw: RawTransform, TView: ViewTransform> PrimitiveType for Primitive<C, TRaw, TView> {
    fn get_bin(&self, bytes: &[u8]) -> u64 {
        self.codec.read(bytes)
    }

    fn set_bin(&self, bytes: &mut [u8], bits: u64) {
        self.codec.write(bytes, bits);
    }

    fn get_raw(&self, bytes: &[u8]) -> Value {
        self.raw.bits_to_raw(self.get_bin(bytes))
    }

    fn set_raw(&self, bytes: &mut [u8], raw: Value) {
        self.set_bin(bytes, self.raw.raw_to_bits(raw));
    }

    fn get(&self, bytes: &[u8]) -> Value {
        self.view.raw_to_view(self.get_raw(bytes))
    }

    fn set(&self, bytes: &mut [u8], value: Value) {
        self.set_raw(bytes, self.view.view_to_raw(value));
    }
}

enum Raw {
    Signed(SignedTransform),
    Float32(Float32Transform),
    Float64(Float64Transform),
    None(NoneTransform)
}

enum View {
    Float(FloatTransform),
    None(NoneTransform)
}

enum Codec1 {
    BitLE(BitCodecLE),
    BitBE(BitCodecBE),
    ByteLE(ByteCodecLE),
    ByteBE(ByteCodecBE)
}

fn get_field(codec: Codec1, raw: Raw, view: View) -> Box<dyn PrimitiveType> {
    match (codec, raw, view) {
        (Codec1::BitBE(c), Raw::None(r), View::None(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::BitLE(c), Raw::None(r), View::None(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::ByteLE(c), Raw::None(r), View::None(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::ByteBE(c), Raw::None(r), View::None(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::BitBE(c), Raw::Signed(r), View::None(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::BitLE(c), Raw::Signed(r), View::None(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::ByteLE(c), Raw::Signed(r), View::None(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::ByteBE(c), Raw::Signed(r), View::None(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::BitBE(c), Raw::Float32(r), View::None(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::BitLE(c), Raw::Float32(r), View::None(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::ByteLE(c), Raw::Float32(r), View::None(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::ByteBE(c), Raw::Float32(r), View::None(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::BitBE(c), Raw::Float64(r), View::None(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::BitLE(c), Raw::Float64(r), View::None(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::ByteLE(c), Raw::Float64(r), View::None(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::ByteBE(c), Raw::Float64(r), View::None(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::BitBE(c), Raw::None(r), View::Float(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::BitLE(c), Raw::None(r), View::Float(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::ByteLE(c), Raw::None(r), View::Float(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::ByteBE(c), Raw::None(r), View::Float(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::BitBE(c), Raw::Signed(r), View::Float(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::BitLE(c), Raw::Signed(r), View::Float(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::ByteLE(c), Raw::Signed(r), View::Float(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::ByteBE(c), Raw::Signed(r), View::Float(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::BitBE(c), Raw::Float32(r), View::Float(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::BitLE(c), Raw::Float32(r), View::Float(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::ByteLE(c), Raw::Float32(r), View::Float(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::ByteBE(c), Raw::Float32(r), View::Float(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::BitBE(c), Raw::Float64(r), View::Float(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::BitLE(c), Raw::Float64(r), View::Float(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::ByteLE(c), Raw::Float64(r), View::Float(v)) => Box::new(Primitive::new(c, r, v)),
        (Codec1::ByteBE(c), Raw::Float64(r), View::Float(v)) => Box::new(Primitive::new(c, r, v))
    }
}

/// Attempts to construct a [PrimitiveType] from a structure [Field], returns [None] when the field
/// cannot be represented a [PrimitiveType]. A value of [None] is typically returned when the field is
/// an enum or is not fixed.
pub fn from_field(field: &Field) -> Option<Box<dyn PrimitiveType>> {
    let fixed = field.ty.as_fixed()?;
    let raw = match fixed.raw {
        FieldRaw::Transmute => {
            if fixed.raw_type.is_signed() {
                Raw::Signed(SignedTransform { max_positive: 2u64.pow(fixed.raw_type.get_aligned_bit_size() as u32 - 1) - 1 })
            } else if fixed.raw_type == FixedFieldType::Float32 {
                Raw::Float32(Float32Transform)
            } else if fixed.raw_type == FixedFieldType::Float64 {
                Raw::Float64(Float64Transform)
            } else {
                Raw::None(NoneTransform)
            }
        },
        FieldRaw::SignedCast(max_positive) => Raw::Signed(SignedTransform { max_positive: max_positive as _ }),
        FieldRaw::None => Raw::None(NoneTransform)
    };
    let view = match fixed.view {
        FieldView::Float { a, b, a_inv, b_inv } => View::Float(FloatTransform {
            a,
            b,
            a_inv,
            b_inv,
        }),
        FieldView::Enum(_) => return None,
        _ => View::None(NoneTransform)
    };
    let codec = match (fixed.endianness, (field.loc.bit_size % 8) == 0) {
        (Endianness::Little, true) => Codec1::ByteLE(ByteCodecLE),
        (Endianness::Big, true) => Codec1::ByteBE(ByteCodecBE),
        (Endianness::Little, false) => Codec1::BitLE(BitCodecLE::new(field.loc.bit_offset, field.loc.bit_size)),
        (Endianness::Big, false) => Codec1::BitBE(BitCodecBE::new(field.loc.bit_offset, field.loc.bit_size)),
    };
    Some(get_field(codec, raw, view))
}

/// Creates a [PrimitiveType] directly from a [FixedFieldType] and a field [Endianness].
/// This function does not support loading from bit fields.
///
/// # Arguments
///
/// * `ty`: the type of the field.
/// * `endianness`: the endianness of the field.
///
/// returns: Box<dyn PrimitiveType, Global>
pub fn from_fixed_field_type(ty: FixedFieldType, endianness: Endianness) -> Box<dyn PrimitiveType> {
    let codec = match endianness {
        Endianness::Little => Codec1::ByteLE(ByteCodecLE),
        Endianness::Big => Codec1::ByteBE(ByteCodecBE),
    };
    let raw = if ty.is_signed() {
        Raw::Signed(SignedTransform { max_positive: 2u64.pow(ty.get_aligned_bit_size() as u32 - 1) - 1 })
    } else if ty == FixedFieldType::Float32 {
        Raw::Float32(Float32Transform)
    } else if ty == FixedFieldType::Float64 {
        Raw::Float64(Float64Transform)
    } else {
        Raw::None(NoneTransform)
    };
    let view = View::None(NoneTransform);
    get_field(codec, raw, view)
}
