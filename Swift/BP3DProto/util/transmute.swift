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

import Foundation

public func transmute(_ x: Int8) -> UInt8 {
    UInt8(bitPattern: x)
}

public func transmute(_ x: Int16) -> UInt16 {
    UInt16(bitPattern: x)
}

public func transmute(_ x: Int32) -> UInt32 {
    UInt32(bitPattern: x)
}

public func transmute(_ x: Int64) -> UInt64 {
    UInt64(bitPattern: x)
}

public func transmute(_ x: UInt8) -> Int8 {
    Int8(bitPattern: x)
}

public func transmute(_ x: UInt16) -> Int16 {
    Int16(bitPattern: x)
}

public func transmute(_ x: UInt32) -> Int32 {
    Int32(bitPattern: x)
}

public func transmute(_ x: UInt64) -> Int64 {
    Int64(bitPattern: x)
}

public func transmute(_ x: Float32) -> UInt32 {
    x.bitPattern
}

public func transmute(_ x: Float64) -> UInt64 {
    x.bitPattern
}

public func transmute(_ x: UInt32) -> Float32 {
    Float32(bitPattern: x)
}

public func transmute(_ x: UInt64) -> Float64 {
    Float64(bitPattern: x)
}
