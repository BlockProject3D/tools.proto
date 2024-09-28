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

extension UnionsItem: BP3DProto.FromBytes  {
    public typealias Buffer = B;
    public typealias Output = Self;
    public static func from(bytes: B) throws -> BP3DProto.Message<Self> {
        var byteOffset = 0;
        let headerMsg = try EnumsHeader.from(bytes: bytes[byteOffset...]);
        byteOffset += headerMsg.size;
        let header = headerMsg.data;
        let nameMsg = try BP3DProto.NullTerminatedString<B>.from(bytes: bytes[byteOffset...]);
        byteOffset += nameMsg.size;
        let name = nameMsg.data;
        let valueMsg = try UnionsValue.from(bytes: bytes[byteOffset...], discriminant: header);
        byteOffset += valueMsg.size;
        let value = valueMsg.data;

        let _data = UnionsItem(
            header: header,
            name: name,
            value: value
        );
        return BP3DProto.Message(size: byteOffset, data: _data);
    }
}