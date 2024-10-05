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

use bp3d_protoc::api::core::loader::Loader;
use bp3d_protoc::api::core::Error;
use bp3d_protoc::gen::RustImportSolver;

const ZST1: &str = "
{
    name: \"test\",
    structs: [
        {
            name: \"Test\",
            fields: []
        }
    ]
}
";

const ZST2: &str = "
{
    name: \"test\",
    structs: [
        {
            name: \"Test\",
            fields: [
                { name: \"v\", info: { type: \"unsigned\", bits: 0 } }
            ]
        }
    ]
}
";

const ZST3: &str = "
{
    name: \"test\",
    structs: [
        {
            name: \"Test\",
            fields: [
                { name: \"v\", info: { type: \"unsigned\", bits: 8 }, array_len: 0 }
            ]
        }
    ]
}
";

#[test]
fn struct_zst1() {
    let mut loader = Loader::new(1);
    loader.load_from_string(ZST1, "").unwrap();
    let err = loader.compile(&RustImportSolver).unwrap_err();
    assert!(matches!(err, Error::Compiler(bp3d_protoc::compiler::Error::ZeroStruct)));
}

#[test]
fn struct_zst2() {
    let mut loader = Loader::new(1);
    loader.load_from_string(ZST2, "").unwrap();
    let err = loader.compile(&RustImportSolver).unwrap_err();
    assert!(matches!(
        err,
        Error::Compiler(bp3d_protoc::compiler::Error::UnsupportedBitSize(0))
    ));
}

#[test]
fn struct_zst3() {
    let mut loader = Loader::new(1);
    loader.load_from_string(ZST3, "").unwrap();
    let err = loader.compile(&RustImportSolver).unwrap_err();
    assert!(matches!(err, Error::Compiler(bp3d_protoc::compiler::Error::ZeroArray)));
}
