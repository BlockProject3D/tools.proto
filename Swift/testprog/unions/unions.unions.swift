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

public enum UnionsValue<B: BP3DProto.Buffer> {
    case null;
    case string(ValuesValueString<B>);
    case int8(ValuesValueInt8<B>);
    case int16(ValuesValueInt16<B>);
    case int32(ValuesValueInt32<B>);
    case int64(ValuesValueInt64<B>);
    case uInt8(ValuesValueUInt8<B>);
    case uInt16(ValuesValueUInt16<B>);
    case uInt32(ValuesValueUInt32<B>);
    case uInt64(ValuesValueUInt64<B>);
    case float(ValuesValueFloat<B>);
    case double(ValuesValueDouble<B>);

}
extension UnionsValue {
    public static func from(bytes: B, discriminant: EnumsHeader<B>) throws -> BP3DProto.Message<Self> {
        let discriminant = discriminant.rawType;
        switch discriminant {
            case 0:
                return BP3DProto.Message(size: 0, data: Self.null);
            case 1:
                let msg = try ValuesValueString.from(bytes: bytes);
                return msg.map({ v in Self.string(v) });
            case 2:
                let msg = try ValuesValueInt8.from(bytes: bytes);
                return msg.map({ v in Self.int8(v) });
            case 3:
                let msg = try ValuesValueInt16.from(bytes: bytes);
                return msg.map({ v in Self.int16(v) });
            case 4:
                let msg = try ValuesValueInt32.from(bytes: bytes);
                return msg.map({ v in Self.int32(v) });
            case 5:
                let msg = try ValuesValueInt64.from(bytes: bytes);
                return msg.map({ v in Self.int64(v) });
            case 6:
                let msg = try ValuesValueUInt8.from(bytes: bytes);
                return msg.map({ v in Self.uInt8(v) });
            case 7:
                let msg = try ValuesValueUInt16.from(bytes: bytes);
                return msg.map({ v in Self.uInt16(v) });
            case 8:
                let msg = try ValuesValueUInt32.from(bytes: bytes);
                return msg.map({ v in Self.uInt32(v) });
            case 9:
                let msg = try ValuesValueUInt64.from(bytes: bytes);
                return msg.map({ v in Self.uInt64(v) });
            case 10:
                let msg = try ValuesValueFloat.from(bytes: bytes);
                return msg.map({ v in Self.float(v) });
            case 11:
                let msg = try ValuesValueDouble.from(bytes: bytes);
                return msg.map({ v in Self.double(v) });

            default:
                throw BP3DProto.Error.invalidUnionDiscriminant(UInt(discriminant));
        }
    }

}
extension UnionsValue {
    public static func write<B1: WritableBuffer>(input: Self, discriminant: EnumsHeader<B>, to out: inout B1) throws {
        let discriminant = discriminant.rawType;
        switch input {
            case Self.string(let v):
                if discriminant == 1 {
                    try ValuesValueString.write(input: v, to: &out);
                } else {
                    throw BP3DProto.Error.invalidUnionDiscriminant(UInt(discriminant));
                }
                break;
            case Self.int8(let v):
                if discriminant == 2 {
                    try ValuesValueInt8.write(input: v, to: &out);
                } else {
                    throw BP3DProto.Error.invalidUnionDiscriminant(UInt(discriminant));
                }
                break;
            case Self.int16(let v):
                if discriminant == 3 {
                    try ValuesValueInt16.write(input: v, to: &out);
                } else {
                    throw BP3DProto.Error.invalidUnionDiscriminant(UInt(discriminant));
                }
                break;
            case Self.int32(let v):
                if discriminant == 4 {
                    try ValuesValueInt32.write(input: v, to: &out);
                } else {
                    throw BP3DProto.Error.invalidUnionDiscriminant(UInt(discriminant));
                }
                break;
            case Self.int64(let v):
                if discriminant == 5 {
                    try ValuesValueInt64.write(input: v, to: &out);
                } else {
                    throw BP3DProto.Error.invalidUnionDiscriminant(UInt(discriminant));
                }
                break;
            case Self.uInt8(let v):
                if discriminant == 6 {
                    try ValuesValueUInt8.write(input: v, to: &out);
                } else {
                    throw BP3DProto.Error.invalidUnionDiscriminant(UInt(discriminant));
                }
                break;
            case Self.uInt16(let v):
                if discriminant == 7 {
                    try ValuesValueUInt16.write(input: v, to: &out);
                } else {
                    throw BP3DProto.Error.invalidUnionDiscriminant(UInt(discriminant));
                }
                break;
            case Self.uInt32(let v):
                if discriminant == 8 {
                    try ValuesValueUInt32.write(input: v, to: &out);
                } else {
                    throw BP3DProto.Error.invalidUnionDiscriminant(UInt(discriminant));
                }
                break;
            case Self.uInt64(let v):
                if discriminant == 9 {
                    try ValuesValueUInt64.write(input: v, to: &out);
                } else {
                    throw BP3DProto.Error.invalidUnionDiscriminant(UInt(discriminant));
                }
                break;
            case Self.float(let v):
                if discriminant == 10 {
                    try ValuesValueFloat.write(input: v, to: &out);
                } else {
                    throw BP3DProto.Error.invalidUnionDiscriminant(UInt(discriminant));
                }
                break;
            case Self.double(let v):
                if discriminant == 11 {
                    try ValuesValueDouble.write(input: v, to: &out);
                } else {
                    throw BP3DProto.Error.invalidUnionDiscriminant(UInt(discriminant));
                }
                break;

            default:
                break;
        }
    }

}
extension UnionsValue {
    public func setDiscriminant<B1: BP3DProto.WritableBuffer>(_ discriminant: EnumsHeader<B1>) where B1: BP3DProto.Buffer {
        var discriminantValue: UInt8 = 0;
        switch self {
            case Self.null:
                discriminantValue = 0;
                break;
            case Self.string:
                discriminantValue = 1;
                break;
            case Self.int8:
                discriminantValue = 2;
                break;
            case Self.int16:
                discriminantValue = 3;
                break;
            case Self.int32:
                discriminantValue = 4;
                break;
            case Self.int64:
                discriminantValue = 5;
                break;
            case Self.uInt8:
                discriminantValue = 6;
                break;
            case Self.uInt16:
                discriminantValue = 7;
                break;
            case Self.uInt32:
                discriminantValue = 8;
                break;
            case Self.uInt64:
                discriminantValue = 9;
                break;
            case Self.float:
                discriminantValue = 10;
                break;
            case Self.double:
                discriminantValue = 11;
                break;

        };
        discriminant.setRawType(discriminantValue);
    }
}
extension UnionsValue {
    public func isNull() -> Bool {
        switch self {
            case Self.null:
                return true;
            default:
                return false;
        }
    }
    public func asString() -> ValuesValueString<B>? {
        switch self {
            case Self.string(let v):
                return v;
            default:
                return nil;
        }
    }
    public func asInt8() -> ValuesValueInt8<B>? {
        switch self {
            case Self.int8(let v):
                return v;
            default:
                return nil;
        }
    }
    public func asInt16() -> ValuesValueInt16<B>? {
        switch self {
            case Self.int16(let v):
                return v;
            default:
                return nil;
        }
    }
    public func asInt32() -> ValuesValueInt32<B>? {
        switch self {
            case Self.int32(let v):
                return v;
            default:
                return nil;
        }
    }
    public func asInt64() -> ValuesValueInt64<B>? {
        switch self {
            case Self.int64(let v):
                return v;
            default:
                return nil;
        }
    }
    public func asUInt8() -> ValuesValueUInt8<B>? {
        switch self {
            case Self.uInt8(let v):
                return v;
            default:
                return nil;
        }
    }
    public func asUInt16() -> ValuesValueUInt16<B>? {
        switch self {
            case Self.uInt16(let v):
                return v;
            default:
                return nil;
        }
    }
    public func asUInt32() -> ValuesValueUInt32<B>? {
        switch self {
            case Self.uInt32(let v):
                return v;
            default:
                return nil;
        }
    }
    public func asUInt64() -> ValuesValueUInt64<B>? {
        switch self {
            case Self.uInt64(let v):
                return v;
            default:
                return nil;
        }
    }
    public func asFloat() -> ValuesValueFloat<B>? {
        switch self {
            case Self.float(let v):
                return v;
            default:
                return nil;
        }
    }
    public func asDouble() -> ValuesValueDouble<B>? {
        switch self {
            case Self.double(let v):
                return v;
            default:
                return nil;
        }
    }

}
