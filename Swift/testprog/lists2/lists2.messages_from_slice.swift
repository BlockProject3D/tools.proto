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

extension Lists2SpanRun: BP3DProto.FromSlice  {
    public typealias Buffer = B;
    public typealias Output = Self;
    public static func from(slice: B) throws -> BP3DProto.Message<Self> {
        var byteOffset = 0;
        let timesMsg = try Lists2Times.from(slice: slice[byteOffset...]);
        byteOffset += timesMsg.size;
        let times = timesMsg.data;
        let varsMsg = try BP3DProto.SizedList<B, BP3DProto.ValueLE<B, UInt8>, BP3DProto.ValueLE<B, UInt16>, UnionsItem<B>>.from(slice: slice[byteOffset...]);
        byteOffset += varsMsg.size;
        let vars = varsMsg.data;

        let _data = Lists2SpanRun(
            times: times,
            vars: vars
        );
        return BP3DProto.Message(size: byteOffset, data: _data);
    }
}

extension Lists2Dataset: BP3DProto.FromSlice  {
    public typealias Buffer = B;
    public typealias Output = Self;
    public static func from(slice: B) throws -> BP3DProto.Message<Self> {
        var byteOffset = 0;
        let runsMsg = try BP3DProto.UnsizedList<B, BP3DProto.ValueLE<B, UInt32>, Lists2SpanRun<B>>.from(slice: slice[byteOffset...]);
        byteOffset += runsMsg.size;
        let runs = runsMsg.data;

        let _data = Lists2Dataset(
            runs: runs
        );
        return BP3DProto.Message(size: byteOffset, data: _data);
    }
}