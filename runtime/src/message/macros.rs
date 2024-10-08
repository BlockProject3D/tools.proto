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

#[macro_export]
macro_rules! generate_array_wrapper {
    ($name: ident, $inner: ident, $t: ty) => {
        pub struct $name<'a, B>($crate::message::util::Array<B, $t, $inner<&'a [u8]>>);

        impl<'a, B> $name<'a, B> {
            pub unsafe fn from_raw_parts(data: B, len: usize) -> $name<'a, B> {
                Self($crate::message::util::Array::<B, $t, $inner<&'a [u8]>>::from_raw_parts(data, len))
            }

            pub fn new(data: B) -> $name<'a, B> {
                unsafe { Self::from_raw_parts(data, 0) }
            }

            pub fn from_array(inner: $crate::message::util::Array<B, $t, $inner<&'a [u8]>>) -> Self {
                Self(inner)
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }
        }

        impl<'a, B: std::io::Write> $name<'a, B> {
            pub fn write_item(&mut self, item: &$inner<&[u8]>) -> $crate::message::Result<()> {
                self.0.write_item(item)
            }

            pub fn write_items(&mut self, items: &[$inner<&[u8]>]) -> $crate::message::Result<()> {
                self.0.write_items(items)
            }
        }

        impl<'a, B: AsRef<[u8]>> $name<'a, B> {
            pub fn to_ref(&self) -> $crate::message::util::Array<&[u8], $t, $inner<&[u8]>> {
                self.0.to_ref()
            }

            pub fn from_parts(data: B, len: usize) -> Option<Self> {
                $crate::message::util::Array::<B, $t, $inner<&'a [u8]>>::from_parts(data, len).map(Self)
            }

            pub fn get(&self, index: usize) -> $inner<&[u8]> {
                self.0.get(index)
            }

            pub fn iter<'b>(&'b self) -> $crate::message::util::array::Iter<'b, $inner<&[u8]>> {
                self.0.iter()
            }
        }

        impl<'a, B: AsMut<[u8]>> $name<'a, B> {
            pub fn get_mut(&mut self, index: usize) -> $inner<&mut [u8]> {
                self.0.get_mut(index)
            }

            pub fn iter_mut<'b>(&'b mut self) -> $crate::message::util::array::IterMut<'b, $inner<&mut [u8]>> {
                self.0.iter_mut()
            }
        }
    };
}
