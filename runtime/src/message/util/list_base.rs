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

macro_rules! impl_list_base {
    ($t: ident) => {
        impl<B, T, Item> $t<B, T, Item> {
            pub unsafe fn from_raw_parts(data: B, len: usize) -> $t<B, T, Item> {
                $t {
                    data,
                    len,
                    useless: PhantomData::default(),
                    useless1: PhantomData::default()
                }
            }

            pub fn new(data: B) -> $t<B, T, Item> {
                unsafe { $t::from_raw_parts(data, 0) }
            }

            pub fn len(&self) -> usize {
                self.len
            }
        }

        impl<B: AsRef<[u8]>, T: WriteTo<Input: ToUsize + Sized>, Item> WriteTo for $t<B, T, Item> {
            type Input = $t<B, T, Item>;

            fn write_to<W: std::io::Write>(input: &Self::Input, mut out: W) -> Result<(), Error> {
                T::write_to(&T::Input::from_usize(input.len), &mut out)?;
                out.write_all(input.data.as_ref())?;
                Ok(())
            }
        }

        impl<B: std::io::Write, T, I> $t<B, T, I> {
            pub fn write_item<Item: WriteTo<Input = Item>>(&mut self, item: &Item) -> Result<(), Error> {
                Item::write_to(item, &mut self.data)?;
                self.len += 1;
                Ok(())
            }

            pub fn write_items<Item: WriteTo<Input = Item>>(&mut self, items: &[Item]) -> Result<(), Error> {
                for item in items {
                    self.write_item(item)?;
                }
                Ok(())
            }
        }

        impl<B: AsRef<[u8]>, T, Item> $t<B, T, Item> {
            pub fn to_ref<Item1>(&self) -> $t<&[u8], T, Item1> {
                unsafe { $t::from_raw_parts(self.data.as_ref(), self.len) }
            }
        }
    };
}

pub(crate) use impl_list_base;
