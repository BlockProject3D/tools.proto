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

impl<'a> bp3d_proto::message::FromSlice<'a> for Test1<'a> {
    type Output = Self;

    fn from_slice(slice: &'a [u8]) -> Result<bp3d_proto::message::Message<Self>, bp3d_proto::message::Error> {
        use bp3d_proto::message::FromSlice;
        let mut byte_offset: usize = 0;
        let s1_msg = bp3d_proto::util::NullTerminatedString::from_slice(&slice[byte_offset..])?;
        byte_offset += s1_msg.size();
        let s1 = s1_msg.into_inner();
        let p1_msg = u32::from_slice(&slice[byte_offset..])?;
        byte_offset += p1_msg.size();
        let p1 = p1_msg.into_inner();
        let data = Test1 {
            s1,
            p1,
        };
        Message::new(byte_offset, data)
    }
}
impl<'a> bp3d_proto::message::FromSlice<'a> for Test<'a> {
    type Output = Self;

    fn from_slice(slice: &'a [u8]) -> Result<bp3d_proto::message::Message<Self>, bp3d_proto::message::Error> {
        use bp3d_proto::message::FromSlice;
        let mut byte_offset: usize = 0;
        let s1_msg = bp3d_proto::util::NullTerminatedString::from_slice(&slice[byte_offset..])?;
        byte_offset += s1_msg.size();
        let s1 = s1_msg.into_inner();
        let s2_msg = bp3d_proto::util::NullTerminatedString::from_slice(&slice[byte_offset..])?;
        byte_offset += s2_msg.size();
        let s2 = s2_msg.into_inner();
        let p1_msg = bp3d_proto::util::Optional::<Test1>::from_slice(&slice[byte_offset..])?;
        byte_offset += p1_msg.size();
        let p1 = p1_msg.into_inner();
        let data = Test {
            s1,
            s2,
            p1,
        };
        Message::new(byte_offset, data)
    }
}