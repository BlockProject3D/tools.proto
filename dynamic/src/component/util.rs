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
use crate::component::ComponentType;

pub struct DiscoverTool<'a> {
    items: Option<Vec<BufferView<'a>>>,
    children: Option<Vec<BufferView<'a>>>,
    freed_items: Option<Vec<BufferView<'a>>>,
    freed_children: Vec<BufferView<'a>>,
}

impl<'a> DiscoverTool<'a> {
    pub fn new(items: Option<Vec<BufferView<'a>>>, children: Vec<BufferView<'a>>) -> Self {
        Self {
            items: None,
            children: None,
            freed_children: children,
            freed_items: items,
        }
    }

    pub fn add_item(&mut self, view: BufferView<'a>) {
        self.items.get_or_insert_default().push(view);
    }

    pub fn add_child(&mut self, child: BufferView<'a>) {
        self.children.get_or_insert_default().push(child);
    }

    pub fn discover_item(&mut self, ty: &impl ComponentType, loc: Location) {
        //FIXME: This does not work if ty does not build the same type of views than the views already in items.
        let mut view = self.freed_items.get_or_insert_default().pop().unwrap_or(ty.new_instance(false));
        *view.location_mut() = loc;
        self.add_item(view);
    }

    pub fn discover_child(&mut self, ty: &impl ComponentType, loc: Location) {
        //FIXME: This does not work if ty does not build the same type of views than the views already in children.
        let mut view = self.freed_children.pop().unwrap_or(ty.new_instance(false));
        *view.location_mut() = loc;
        self.add_child(view);
    }

    pub fn into_inner(self) -> (Option<Vec<BufferView<'a>>>, Vec<BufferView<'a>>) {
        (self.items, self.children.unwrap_or(self.freed_children))
    }
}
