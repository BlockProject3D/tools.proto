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

use std::alloc::{alloc, dealloc, realloc, Layout};
use std::cell::Cell;
use std::ptr::copy_nonoverlapping;
use std::ops::{Range, RangeFrom, RangeTo};
use std::ptr::{write_bytes, NonNull};
use std::slice;

pub trait Index {
    type Output;
    fn index(&self, bytes: &Bytes) -> Self::Output;
}

impl Index for usize {
    type Output = u8;

    fn index(&self, bytes: &Bytes) -> Self::Output {
        let ptr = bytes.bytes.get();
        let len = bytes.len.get();
        if *self > len {
            panic!("Cannot index, index out of bounds: {} > {}", *self, len);
        }
        unsafe { *ptr.add(*self).as_ptr() }
    }
}

impl Index for Range<usize> {
    type Output = Bytes;

    fn index(&self, bytes: &Bytes) -> Self::Output {
        let required_len = self.len();
        let len = bytes.len.get();
        if required_len > len {
            panic!("Cannot slice, range out of bounds: {} > {} ({}..{})", required_len, len, self.start, self.end);
        }
        let bytes = unsafe { bytes.bytes.get().add(self.start) };
        Bytes {
            bytes: Cell::new(bytes),
            len: Cell::new(self.len()),
            owned: false,
        }
    }
}

impl Index for RangeFrom<usize> {
    type Output = Bytes;

    fn index(&self, bytes: &Bytes) -> Self::Output {
        (self.start..bytes.len.get()).index(bytes)
    }
}

impl Index for RangeTo<usize> {
    type Output = Bytes;

    fn index(&self, bytes: &Bytes) -> Self::Output {
        (0..self.end).index(bytes)
    }
}

#[derive(Debug)]
pub struct Bytes {
    bytes: Cell<NonNull<u8>>,
    len: Cell<usize>,
    owned: bool,
}

impl Bytes {
    pub fn from_slice(slice: &[u8]) -> Bytes {
        let ptr = unsafe { alloc(Layout::array::<u8>(slice.len()).unwrap()) };
        unsafe { copy_nonoverlapping(slice.as_ptr(), ptr, slice.len()) };
        Bytes {
            bytes: unsafe { Cell::new(NonNull::new_unchecked(ptr)) },
            len: Cell::new(slice.len()),
            owned: true,
        }
    }

    pub fn with_capacity(capacity: usize) -> Bytes {
        let ptr = unsafe { alloc(Layout::array::<u8>(capacity).unwrap()) };
        unsafe { write_bytes(ptr, 0, capacity) }
        Bytes {
            bytes: unsafe { Cell::new(NonNull::new_unchecked(ptr)) },
            len: Cell::new(capacity),
            owned: true
        }
    }

    pub unsafe fn copy(&mut self, slice: &[u8]) -> bool {
        let added_bytes = self.resize(slice.len());
        copy_nonoverlapping(slice.as_ptr(), self.bytes.get().as_ptr(), slice.len());
        added_bytes == 0
    }

    unsafe fn resize(&mut self, new_len: usize) -> usize {
        let mut ptr = self.bytes.get();
        let len = self.len.get();
        if new_len == len {
            return 0;
        }
        if new_len < len {
            //FIXME: we need capacity support
            return 0;
        }
        if !self.owned {
            panic!("Attempt to reserve bytes on non-owned buffer");
        }
        ptr = unsafe { NonNull::new_unchecked(realloc(ptr.as_ptr(), Layout::array::<u8>(len).unwrap(), new_len)) };
        // The number of bytes which were added in the realloc.
        let ending = new_len - len;
        //unsafe { write_bytes(ptr.add(len).as_ptr(), 0, ending) }
        self.bytes.set(ptr);
        self.len.set(new_len);
        ending
    }

    pub fn as_bytes(&self) -> &[u8] {
        let ptr = self.bytes.get();
        let len = self.len.get();
        unsafe { slice::from_raw_parts(ptr.as_ptr(), len) }
    }

    pub fn index<I: Index>(&self, index: I) -> I::Output {
        index.index(self)
    }

    pub fn read(&self, data: &mut [u8]) {
        let ptr = self.bytes.get();
        let len = self.len.get();
        let slice = unsafe { std::slice::from_raw_parts(ptr.as_ptr(), len) };
        data.copy_from_slice(slice);
    }

    pub fn len(&self) -> usize {
        self.len.get()
    }

    pub fn write(&mut self, data: &[u8]) {
        let ptr = self.bytes.get();
        let len = self.len.get();
        let slice = unsafe { std::slice::from_raw_parts_mut(ptr.as_ptr(), len) };
        slice.copy_from_slice(data);
    }

    pub unsafe fn delete(&mut self) {
        if self.owned {
            unsafe { dealloc(self.bytes.get().as_ptr(), Layout::array::<u8>(self.len.get()).unwrap()) }
        }
    }
}
