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

use crate::buffer::buffer::Buffer;
use crate::buffer::unsafe_buffer::UnsafeBuffer;
use crate::buffer::view::{Location, PathComponent};
use crate::buffer::BufferView;
use crate::component::Component;
use crate::field::primitive::PrimitiveType;
use std::cell::{Cell, UnsafeCell};
use std::rc::Rc;
use std::sync::Arc;

pub struct Builder<'a> {
    name: String,
    children: Vec<BufferView<'a>>,
    location: Location,
    component: Option<Arc<dyn Component>>,
    flat: Rc<Cell<bool>>,
    primitive: Option<Box<dyn PrimitiveType>>,
}

impl<'a> Builder<'a> {
    pub fn new<S: Into<String>>(name: S) -> Builder<'a> {
        Self {
            name: name.into(),
            children: Vec::new(),
            location: Location {
                fixed: false,
                offset: -1,
                size: 0,
            },
            component: None,
            flat: Rc::new(Cell::new(false)),
            primitive: None,
        }
    }

    pub fn primitive(mut self, primitive: Box<dyn PrimitiveType>) -> Self {
        self.primitive = Some(primitive);
        self
    }

    pub fn fixed(mut self, offset: usize, size: usize) -> Builder<'a> {
        self.location.fixed = true;
        self.location.offset = offset as _;
        self.location.size = size;
        self
    }

    pub fn size(mut self, size: usize) -> Builder<'a> {
        self.location.size = size;
        self
    }

    pub fn component(mut self, component: impl Component + 'static) -> Builder<'a> {
        self.component = Some(Arc::new(component));
        self
    }

    pub fn add_child(mut self, mut builder: Builder<'a>) -> Builder<'a> {
        builder.flat = self.flat.clone();
        self.children.push(builder.get());
        self
    }

    fn get(self) -> BufferView<'a> {
        BufferView {
            path_component: Rc::new(PathComponent {
                name: self.name,
                index: Cell::new(-1),
                parent: UnsafeCell::new(None),
            }),
            buffer: Buffer {
                unsafe_buffer: UnsafeBuffer::Borrowed(b""),
                flat: self.flat,
                offset: 0,
            },
            children: self.children,
            location: self.location,
            component: self.component,
            primitive: self.primitive,
            items: None,
        }
    }

    pub fn build(self, init_mem: bool) -> BufferView<'a> {
        let motherfuckingrust = self.location.size;
        let mut view = self.get();
        setup_parents(&mut view);
        if motherfuckingrust > 0 && init_mem {
            view.buffer.unsafe_buffer = UnsafeBuffer::with_capacity(motherfuckingrust);
        }
        view
    }
}

fn setup_parents(view: &mut BufferView) {
    for child in &mut view.children {
        *unsafe { &mut *child.path_component.parent.get() } = Some(view.path_component.clone());
        setup_parents(child);
    }
}
