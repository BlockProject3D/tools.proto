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
use std::fmt::{Debug, Display};
use std::io::Write;
use std::ops::{Index, IndexMut};
use std::rc::Rc;
use crate::buffer::buffer::Buffer;
use crate::component::{Component, ComponentType, DiscoverTool};
use crate::field::primitive::{PrimitiveType, PrimitiveValue, PrimitiveValueMut};

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

pub struct BufferView<'a> {
    pub(super) path_component: Rc<PathComponent>,
    pub(super) buffer: Buffer<'a>,
    pub(super) children: Vec<BufferView<'a>>,
    pub(super) location: Location,
    pub(super) component: Option<&'static dyn Component>,
    pub(super) items: Option<Vec<BufferView<'a>>>,
    pub(super) primitive: Option<Box<dyn PrimitiveType>>,
    pub(super) component_ty: Option<Box<dyn ComponentType>>
}

impl Debug for BufferView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.primitive.is_none() {
            write!(f, "BufferView {{ name: {}, location: {:?}, children: {:?} }}", self.path_component.name, self.location, self.children)
        } else {
            write!(f, "BufferView {{ name: {}, location: {:?}, children: {:?}, primitive }}", self.path_component.name, self.location, self.children)
        }
    }
}

impl Display for BufferView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path_max_width = self.get_path_max_width();
        let hex_max_width = self.get_hex_max_width();
        writeln!(f, "|-{:-^path_max_width$}-|-{:-^6}-|-{:-^6}-|-{:-^hex_max_width$}-|-{:-^10}-|", "", "", "", "", "")?;
        writeln!(f, "| {: ^path_max_width$} | {: ^6} | {: ^6} | {: ^hex_max_width$} | {: ^10} |", "Path", "Offset", "Size", "Hex", "Value")?;
        writeln!(f, "|-{:-^path_max_width$}-|-{:-^6}-|-{:-^6}-|-{:-^hex_max_width$}-|-{:-^10}-|", "", "", "", "", "")?;
        self.dump_children(f, path_max_width, hex_max_width)?;
        Ok(())
    }
}

impl<'a> BufferView<'a> {
    fn get_path_max_width(&self) -> usize {
        let mut path_max_width = 0;
        for child in &self.children {
            let len = child.get_path().len();
            if len > path_max_width {
                path_max_width = len;
            }
            let other_len = child.get_path_max_width();
            if other_len > path_max_width {
                path_max_width = other_len;
            }
        }
        path_max_width
    }

    fn get_hex_max_width(&self) -> usize {
        let mut hex_max_width = 0;
        for child in &self.children {
            let len = (child.buffer.len() + (child.location.offset as usize)) * 3;
            if len > hex_max_width {
                hex_max_width = len;
            }
            let other_len = child.get_hex_max_width();
            if other_len > hex_max_width {
                hex_max_width = other_len;
            }
        }
        hex_max_width
    }

    fn dump_children(&self, f: &mut std::fmt::Formatter<'_>, path_max_width: usize, hex_max_width: usize) -> std::fmt::Result {
        for child in &self.children {
            let value = child.get_primitive().map(|v| v.get().to_string()).unwrap_or("####".into());
            let bytes = format!("{:X?}", child.buffer.as_bytes()).replace(",", "");
            let bytes = &bytes[1..bytes.len() - 1];
            let mut padding = String::from("");
            for _ in 0..child.location.offset {
                padding += ".. ";
            }
            writeln!(f, "| {: <path_max_width$} | {: ^6} | {: ^6} | {: <hex_max_width$} | {: ^10} |", child.get_path(), child.location.offset, child.location.size, padding + bytes, value)?;
            child.dump_children(f, path_max_width, hex_max_width)?;
        }
        Ok(())
    }

    pub fn get_primitive(&self) -> Option<PrimitiveValue> {
        self.primitive.as_ref().map(|v| PrimitiveValue::new(self.buffer.as_bytes(), &**v))
    }

    pub fn get_primitive_mut(&mut self) -> Option<PrimitiveValueMut> {
        self.primitive.as_ref().map(|v| PrimitiveValueMut::new(self.buffer.as_bytes_mut(), &**v))
    }

    pub fn location_mut(&mut self) -> &mut Location {
        &mut self.location
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn buffer(&self) -> &Buffer<'a> {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buffer<'a> {
        &mut self.buffer
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
        self.buffer.copy_from(bytes);
        self.read()
    }

    pub fn read_view(&mut self, bytes: &'a [u8]) -> bp3d_proto::message::Result<usize> {
        self.buffer.set_bytes(bytes);
        self.read()
    }

    fn read(&mut self) -> bp3d_proto::message::Result<usize> {
        if self.location.fixed {
            if self.buffer.len() < self.location.size {
                return Err(bp3d_proto::message::Error::Truncated)
            }
            if !self.buffer.flat.get() {
                for child in &mut self.children {
                    child.buffer.unsafe_buffer = self.buffer.unsafe_buffer.index(child.location.offset as usize..child.location.offset as usize + child.location.size);
                    child.read()?;
                }
            }
            return Ok(self.location.size);
        }
        let mut offset = 0;
        if let Some(component) = self.component {
            let ty = self.component_ty.take();
            let mut tool = DiscoverTool::new(ty.as_deref(), self.items.take(), std::mem::replace(&mut self.children, Vec::new()));
            let size = component.read(self, &mut tool)?;
            let (mut items, children) = tool.into_inner();
            if let Some(items) = &mut items {
                let mut item_offset = 0;
                for (index, item) in items.iter_mut().enumerate() {
                    item.path_component.index.set(index as _);
                    if item.location.offset != -1 {
                        item_offset = item.location.offset;
                    }
                    if item_offset as usize >= self.buffer.len() {
                        return Err(bp3d_proto::message::Error::Truncated);
                    }
                    unsafe { item.buffer.unsafe_buffer.delete() };
                    item.buffer.unsafe_buffer = self.buffer.unsafe_buffer.index((item_offset as usize)..);
                    let size = item.read()?;
                    if (item_offset as usize) + size > self.buffer.len() {
                        return Err(bp3d_proto::message::Error::Truncated);
                    }
                    item.buffer.unsafe_buffer = self.buffer.unsafe_buffer.index((item_offset as usize)..(item_offset as usize) + size);
                    item.location.size = size;
                    item_offset += size as isize;
                }
            }
            self.children = children;
            self.items = items;
            offset = size as isize;
        }
        for child in &mut self.children {
            if child.location.offset != -1 {
                offset = child.location.offset;
            }
            if offset as usize >= self.buffer.len() {
                return Err(bp3d_proto::message::Error::Truncated);
            }
            unsafe { child.buffer.unsafe_buffer.delete() };
            child.buffer.unsafe_buffer = self.buffer.unsafe_buffer.index((offset as usize)..);
            let size = child.read()?;
            if (offset as usize) + size > self.buffer.len() {
                return Err(bp3d_proto::message::Error::Truncated);
            }
            child.buffer.unsafe_buffer = self.buffer.unsafe_buffer.index((offset as usize)..(offset as usize) + size);
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
                let _ = v.write(child.buffer.as_bytes());
            }
            unsafe { self.buffer.unsafe_buffer.copy(v.as_slice()) };
        }
    }

    fn flatten(&mut self) -> bp3d_proto::message::Result<()> {
        if self.buffer.flat.get() {
            // Nothing to do view is already flat!
            return Ok(())
        }
        self.flatten_internal();
        self.read()?;
        self.buffer.flat.set(true);
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
        unsafe { self.buffer.unsafe_buffer.delete() };
    }
}
