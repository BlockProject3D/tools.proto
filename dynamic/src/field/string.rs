// Copyright (c) 2025, BlockProject 3D
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

use bp3d_protoc::model::protocol::Endianness;
use crate::buffer::{BufferView, Builder};
use crate::component::{Component, ComponentType, util::DiscoverTool};
use crate::component::factory::SizeType;
use crate::field::codec::{ByteCodecBE, ByteCodecLE, Codec};

pub struct NullTerminatedString;

struct VarcharStringInner<T>(T);

impl<T: Codec> Component for VarcharStringInner<T> {
    fn read(&self, view: &mut BufferView, _: &mut DiscoverTool) -> bp3d_proto::message::Result<()> {
        let len = self.0.read(view.buffer().as_bytes());
        let data = &mut view["data"];
        data.location_mut().size = len as _;
        Ok(())
    }

    fn shape(&self, view: &mut BufferView, _: &Vec<BufferView>) -> bp3d_proto::message::Result<()> {
        let len = view["data"].buffer().len();
        self.0.write(view["len"].buffer_mut().as_bytes_mut(), len as _);
        Ok(())
    }
}

impl Component for NullTerminatedString {
    fn read(&self, view: &mut BufferView, _: &mut DiscoverTool) -> bp3d_proto::message::Result<()> {
        let mut motherfuckingrust = 0;
        while view.buffer().as_bytes()[motherfuckingrust] != 0x0 {
            motherfuckingrust += 1;
        }
        view.location_mut().size = motherfuckingrust;
        Ok(())
    }

    fn shape(&self, view: &mut BufferView, _: &Vec<BufferView>) -> bp3d_proto::message::Result<()> {
        let motherfuckingrust = view.buffer().len() - 1;
        if view.buffer().as_bytes()[motherfuckingrust] != 0x0 {
            view.buffer_mut().as_bytes_mut()[motherfuckingrust] = 0x0;
        }
        Ok(())
    }
}

pub struct VarcharString(pub SizeType);

impl ComponentType for VarcharString {
    fn build(&self, builder: Builder<'static>) -> Builder<'static> {
        if self.0.get_endianness() == Endianness::Little {
            builder.component(VarcharStringInner(ByteCodecLE)).add_child(Builder::new("len").fixed(0, self.0.get_size()))
                .add_child(Builder::new("data"))
        } else {
            builder.component(VarcharStringInner(ByteCodecBE)).add_child(Builder::new("len").fixed(0, self.0.get_size()))
                .add_child(Builder::new("data"))
        }
    }
}

impl ComponentType for NullTerminatedString {
    fn build(&self, builder: Builder<'static>) -> Builder<'static> {
        builder.component(NullTerminatedString)
    }
}
