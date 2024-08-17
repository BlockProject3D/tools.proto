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

public enum Unions2NestedDiscriminant<B> {
    case v1;
    case v2;

}
extension Unions2NestedDiscriminant where B: BP3DProto.Buffer {
    public static func from(slice: B, discriminant: Unions2Header2<B>) throws -> BP3DProto.Message<Self> {
        let discriminant = discriminant.inner.rawTest;
        switch discriminant {
            case 0:
                return BP3DProto.Message(size: 0, data: Self.v1);
            case 1:
                return BP3DProto.Message(size: 0, data: Self.v2);

            default:
                throw BP3DProto.Error.invalidUnionDiscriminant(UInt(discriminant));
        }
    }

}
extension Unions2NestedDiscriminant where B: BP3DProto.WritableBuffer, B: BP3DProto.Buffer {
    public func write<B1: WritableBuffer>(input: Self, discriminant: Unions2Header2<B>, to _: inout B1) throws {
    }

}
extension Unions2NestedDiscriminant where B: BP3DProto.WritableBuffer, B: BP3DProto.Buffer {
    public func setDiscriminant(_ discriminant: Unions2Header2<B>) {
        var discriminantValue: UInt8 = 0;
        switch self {
            case Self.v1:
                discriminantValue = 0;
                break;
            case Self.v2:
                discriminantValue = 1;
                break;

        };
        discriminant.inner.setRawTest(discriminantValue);
    }
}
extension Unions2NestedDiscriminant {
    public func isv1() -> Bool {
        return self == Self.v1;
    }
    public func isv2() -> Bool {
        return self == Self.v2;
    }

}
