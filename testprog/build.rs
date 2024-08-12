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

use bp3d_protoc::generate_rust;

fn main() {
    generate_rust(
        |loader| {
            loader.load("./src/test.json5")?;
            loader.load("./src/structs.json5")?;
            loader.load("./src/bits.json5")?;
            loader.load("./src/bits2.json5")?;
            loader.load("./src/views.json5")?;
            loader.load("./src/struct_arrays.json5")?;
            loader.load("./src/enums.json5")?;
            loader.load("./src/values.json5")?;
            loader.load("./src/unions2.json5")?;
            loader.load("./src/arrays.json5")?;
            Ok(())
        },
        |protoc| protoc.set_reads_messages(true).set_writes_messages(true),
    );
    generate_rust(
        |loader| {
            loader.import("./src/enums.json5", "crate::enums")?;
            loader.import("./src/values.json5", "crate::values")?;
            loader.load("./src/unions.json5")?;
            Ok(())
        },
        |protoc| protoc.set_reads_messages(true).set_writes_messages(true),
    );
    generate_rust(
        |loader| {
            loader.import("./src/enums.json5", "crate::enums")?;
            loader.import("./src/values.json5", "crate::values")?;
            loader.import("./src/unions.json5", "crate::unions")?;
            loader.load("./src/lists.json5")?;
            Ok(())
        },
        |protoc| protoc.set_reads_messages(true).set_writes_messages(true),
    );
}
