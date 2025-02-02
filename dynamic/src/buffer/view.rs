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

use std::cell::Cell;
use std::io::Write;
use std::ops::{Index, IndexMut};
use std::rc::Rc;
use crate::buffer::buffer::Buffer;
use crate::component::Component;

#[derive(Debug)]
pub struct Location {
    pub fixed: bool,
    pub offset: isize,
    pub size: usize,
}

#[derive(Debug)]
pub(super) struct PathComponent {
    pub(super) name: String,
    pub(super) index: Cell<isize>,
    pub(super) parent: Option<Rc<PathComponent>>
}

#[derive(Debug)]
pub struct BufferView<'a> {
    pub(super) path_component: Rc<PathComponent>,
    pub(super) buffer: Buffer<'a>,
    pub(super) children: Vec<BufferView<'a>>,
    pub(super) location: Location,
    pub(super) component: Option<&'static dyn Component>,
    pub(super) items: Option<Vec<BufferView<'a>>>,
    pub(super) flat: Rc<Cell<bool>>
}

impl<'a> BufferView<'a> {
    pub fn location_mut(&mut self) -> &mut Location {
        &mut self.location
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn set_bytes(&mut self, bytes: &'a [u8]) {
        self.buffer = Buffer::Borrowed(bytes);
        self.flat.set(false);
    }

    pub fn copy_from(&mut self, bytes: &[u8]) {
        self.flat.set(unsafe { self.buffer.copy(bytes) });
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.buffer.as_bytes()
    }

    pub fn get(&self, path: &str) -> Option<&BufferView<'a>> {
        let split = path.split(".");
        let mut view = self;
        for name in split {
            let id = name.find('[');
            let name = id.map(|v| &name[..v]).unwrap_or(name);
            view = view.children.iter().find(|child| child.path_component.name == name)?;
            if let Some(id) = id {
                view = view.items.as_ref()?.get(id)?;
            }
        }
        Some(view)
    }

    pub fn add_item(&mut self, view: BufferView<'a>) {
        let mut items = self.items.take().unwrap_or_default();
        view.path_component.index.set((items.len() - 1) as _);
        items.push(view);
        self.items = Some(items);
    }

    pub fn get_mut(&mut self, path: &str) -> Option<&mut BufferView<'a>> {
        let split = path.split(".");
        let mut view = self;
        for name in split {
            let id = name.find('[');
            let name = id.map(|v| &name[..v]).unwrap_or(name);
            view = view.children.iter_mut().find(|child| child.path_component.name == name)?;
            if let Some(id) = id {
                view = view.items.as_mut()?.get_mut(id)?;
            }
        }
        Some(view)
    }

    pub fn iter(&self) -> impl Iterator<Item=&BufferView<'a>> {
        self.children.iter()
    }

    pub fn read_copy(&mut self, bytes: &[u8]) -> bp3d_proto::message::Result<usize> {
        self.copy_from(bytes);
        self.read()
    }

    pub fn read_view(&mut self, bytes: &'a [u8]) -> bp3d_proto::message::Result<usize> {
        self.set_bytes(bytes);
        self.read()
    }

    fn read(&mut self) -> bp3d_proto::message::Result<usize> {
        if self.location.fixed {
            if self.buffer.len() < self.location.size {
                return Err(bp3d_proto::message::Error::Truncated)
            }
            if !self.flat.get() {
                for child in &mut self.children {
                    child.buffer = self.buffer.index(child.location.offset as usize..child.location.offset as usize + child.location.size);
                    child.read()?;
                }
            }
            return Ok(self.location.size);
        }
        let mut offset = 0;
        if let Some(component) = self.component {
            let mut items = self.items.take().unwrap_or_else(Vec::new);
            let size = component.read(self, &mut items)?;
            for (index, item) in items.iter_mut().enumerate() {
                item.path_component.index.set(index as _);
            }
            self.items = Some(items);
            offset = size as isize;
        }
        for child in &mut self.children {
            if child.location.offset != -1 {
                offset = child.location.offset;
            }
            if offset as usize >= self.buffer.len() {
                return Err(bp3d_proto::message::Error::Truncated);
            }
            unsafe { child.buffer.delete() };
            child.buffer = self.buffer.index((offset as usize)..);
            let size = child.read()?;
            if (offset as usize) + size > self.buffer.len() {
                return Err(bp3d_proto::message::Error::Truncated);
            }
            child.buffer = self.buffer.index((offset as usize)..(offset as usize) + size);
            child.location.size = size;
            offset += size as isize;
        }
        Ok(offset as _)
    }

    pub fn shape(&mut self) -> bp3d_proto::message::Result<()> {
        if let Some(component) = self.component {
            let items = self.items.take().unwrap_or_else(Vec::new);
            component.shape(self, &items)?;
            self.items = Some(items);
        }
        if !self.location.fixed {
            for child in &mut self.children {
                child.shape()?
            }
        }
        if self.path_component.parent.is_none() {
            self.flatten()?;
        }
        Ok(())
    }

    fn flatten_internal(&mut self) {
        if self.children.len() > 0 {
            let mut v = Vec::with_capacity(self.buffer.len());
            for child in &mut self.children {
                child.flatten_internal();
                let _ = v.write(child.as_bytes());
            }
            unsafe { self.buffer.copy(v.as_slice()) };
        }
    }

    fn flatten(&mut self) -> bp3d_proto::message::Result<()> {
        if self.flat.get() {
            // Nothing to do view is already flat!
            return Ok(())
        }
        self.flatten_internal();
        self.read()?;
        self.flat.set(true);
        Ok(())
    }

    pub fn name(&self) -> &str {
        &self.path_component.name
    }

    pub fn get_path(&self) -> String {
        let mut v = Vec::new();
        let mut comp = &self.path_component;
        v.push(comp.name.clone());
        while let Some(parent) = &comp.parent {
            comp = parent;
            let index = comp.index.get();
            if index != -1 {
                v.push(format!("[{}].", index) + &comp.name);
            } else {
                v.push(comp.name.clone());
            }
        }
        v.reverse();
        v.join(".")
    }
}

impl<'a, 'b> Index<&'a str> for BufferView<'b> {
    type Output = BufferView<'b>;

    fn index(&self, index: &'a str) -> &Self::Output {
        match self.get(index) {
            Some(view) => view,
            None => panic!("Unable to find view with path '{}'", index)
        }
    }
}

impl<'a, 'b> IndexMut<&'a str> for BufferView<'b> {
    fn index_mut(&mut self, index: &'a str) -> &mut Self::Output {
        match self.get_mut(index) {
            Some(view) => view,
            None => panic!("Unable to find view with path '{}'", index)
        }
    }
}

impl Drop for BufferView<'_> {
    fn drop(&mut self) {
        unsafe { self.buffer.delete() };
    }
}

//TODO: Implement shape/flatten
