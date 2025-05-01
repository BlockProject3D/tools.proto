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

use crate::buffer::{BufferView, Builder};
use crate::component::util::DiscoverTool;

pub trait ComponentType {
    /// Returns the type name of this component.
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    /// Adds the necessary information to the given [Builder] to construct a [BufferView]
    /// representing this [ComponentType].
    ///
    /// # Arguments
    ///
    /// * `builder`: the builder to complete.
    ///
    /// returns: Builder
    fn build(&self, builder: Builder<'static>) -> Builder<'static>;

    /// Creates a new [BufferView] representing this [ComponentType].
    ///
    /// # Arguments
    ///
    /// * `init_mem`: true to pre-initialize the memory of this view with zeros, false otherwise.
    ///
    /// returns: BufferView
    fn new_instance(&self, init_mem: bool) -> BufferView<'static> {
        self.build(Builder::new(self.name())).build(init_mem)
    }
}

pub trait Component {
    /// Reads the data given in the BufferView.
    ///
    /// # Arguments
    ///
    /// * `view`: the view to read from.
    /// * `items`: list of items to fill, clear it if no list is to be attached with the [BufferView].
    ///
    /// returns: Result<(), Error>
    fn read(&self, view: &mut BufferView, items: &mut DiscoverTool) -> bp3d_proto::message::Result<()>;

    /// Shapes the given BufferView.
    ///
    /// # Arguments
    ///
    /// * `view`: the view to shape.
    /// * `items`: list of items attached with the [BufferView], this should be written to the
    ///            underlying BufferView.
    ///
    /// returns: Result<(), Error>
    fn shape(&self, view: &mut BufferView, items: &Vec<BufferView>) -> bp3d_proto::message::Result<()>;
}
