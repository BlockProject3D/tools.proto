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
            /// Creates a list or an array from raw parts.
            /// This function assumes that data has the number of items specified by len.
            ///
            /// # Arguments
            ///
            /// * `data`: the data buffer.
            /// * `len`: the number of items to be read from the buffer.
            ///
            /// # Safety
            ///
            /// This function assumes that data has the number of items specified by len.
            /// For all list types (i.e. lists with dynamically sized items), a wrong length will
            /// simply cause an error (truncated) to be returned if the actual buffer has not enough
            /// bytes to contain all items.
            ///
            /// For all array types, (i.e. lists with fixed size items), a wrong length could result
            /// in UB where the array iterator, getter or setter attempts to slice out of bounds
            /// with a future optimization in release builds, currently it will result in a panic.
            pub unsafe fn from_raw_parts(data: B, len: usize) -> $t<B, T, Item> {
                $t {
                    data,
                    len,
                    useless: PhantomData,
                    useless1: PhantomData,
                }
            }

            pub fn new(data: B) -> $t<B, T, Item> {
                unsafe { $t::from_raw_parts(data, 0) }
            }

            pub fn len(&self) -> usize {
                self.len
            }

            pub fn is_empty(&self) -> bool {
                self.len == 0
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
            pub fn write_item<Item: WriteTo<Input = Item>>(
                &mut self,
                item: &Item,
            ) -> Result<(), Error> {
                Item::write_to(item, &mut self.data)?;
                self.len += 1;
                Ok(())
            }

            pub fn write_items<Item: WriteTo<Input = Item>>(
                &mut self,
                items: &[Item],
            ) -> Result<(), Error> {
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
