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

use crate::model::structure::{SimpleType, StructFieldType};
use bp3d_util::simple_error;

simple_error! {
    pub Error {
        MultiPayload => "message has more than 1 payload",
        VarsizeAfterPayload => "message has 1 or more variable sized fields after the payload",
        UnsupportedBitSize(usize) => "unsupported bit size for fixed field ({}), maximum is 64, minimum is 1",
        UnsupportedType(StructFieldType) => "unsupported field type in struct: {:?}",
        UnsupportedViewType(SimpleType) => "unsupported view for type: {:?}",
        ZeroStruct => "structures must have at least 1 field",
        ZeroArray => "arrays and lists must have at least 1 item",
        UndefinedReference(String) => "undefined reference to '{}'",
        UnresolvedImport(String) => "unresolved import to '{}'",
        UnalignedArrayCodec => "unaligned array in structure",
        SolverError => "failed to resolve imported type",
        ZeroEnum => "enums must have at least 1 variant",
        InvalidUnionDiscriminant => "invalid union discriminant path",
        FloatInUnionDiscriminant => "floats are not allowed as union discriminants",
        InvalidUnionCase(String) => "invalid union case {}",
        UnionTypeMismatch => "mismatch with union discriminant types",
        MissingNestedList(String) => "the list field {} is not allowed to be nested"
    }
}
