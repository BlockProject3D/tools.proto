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

import Foundation;
import BP3DProto;

extension TestTest1: BP3DProto.FromSlice  {
    public typealias Buffer = B;
    public typealias Output = Self;
    public static func from(slice: B) throws -> BP3DProto.Message<Self> {
        var byteOffset = 0;
        let s1Msg = try BP3DProto.NullTerminatedString<B>.from(slice: slice[byteOffset...]);
        byteOffset += s1Msg.size;
        let s1 = s1Msg.data;
        let p1Msg = try BP3DProto.ValueLE<B, UInt32>.from(slice: slice[byteOffset...]);
        byteOffset += p1Msg.size;
        let p1 = p1Msg.data;

        let _data = TestTest1(
            s1: s1,
            p1: p1
        );
        return BP3DProto.Message(size: byteOffset, data: _data);
    }
}

extension TestTest: BP3DProto.FromSlice  {
    public typealias Buffer = B;
    public typealias Output = Self;
    public static func from(slice: B) throws -> BP3DProto.Message<Self> {
        var byteOffset = 0;
        let s1Msg = try BP3DProto.NullTerminatedString<B>.from(slice: slice[byteOffset...]);
        byteOffset += s1Msg.size;
        let s1 = s1Msg.data;
        let s2Msg = try BP3DProto.VarcharString<B, BP3DProto.ValueLE<B, UInt8>>
.from(slice: slice[byteOffset...]);
        byteOffset += s2Msg.size;
        let s2 = s2Msg.data;
        let p1Msg = try BP3DProto.Optional<B, TestTest1>.from(slice: slice[byteOffset...]);
        byteOffset += p1Msg.size;
        let p1 = p1Msg.data;

        let _data = TestTest(
            s1: s1,
            s2: s2,
            p1: p1
        );
        return BP3DProto.Message(size: byteOffset, data: _data);
    }
}
