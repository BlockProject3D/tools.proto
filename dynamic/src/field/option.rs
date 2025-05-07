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

use crate::buffer::{BufferView, Location};
use crate::component::{util::DiscoverTool, Component, ComponentType};
use bp3d_debug::trace;

pub struct Optional<T: ComponentType>(pub T);

impl<T: ComponentType> Component for Optional<T> {
    fn read(&self, view: &mut BufferView, items: &mut DiscoverTool) -> bp3d_proto::message::Result<()> {
        view.location_mut().size = 1;
        let v = view.buffer().as_bytes()[0];
        if v != 0 {
            trace!({ v }, "discover child");
            items.discover_child(
                &self.0,
                Location {
                    offset: -1,
                    size: 0,
                    fixed: false,
                },
            );
        }
        Ok(())
    }

    fn shape(&self, view: &mut BufferView, _: &Vec<BufferView>) -> bp3d_proto::message::Result<()> {
        if view.buffer().is_empty() {
            trace!("allocate optional");
            view.buffer_mut().set_bytes(b"\0");
        }
        if !view.is_empty() {
            trace!("child found");
            view.buffer_mut().as_bytes_mut()[0] = 1;
            unsafe { view.iter_mut().next().unwrap_unchecked() }.location_mut().offset = 1;
        } else {
            trace!("no children");
            view.buffer_mut().as_bytes_mut()[0] = 0;
        }
        Ok(())
    }
}
