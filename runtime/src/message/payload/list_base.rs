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
            pub fn len(&self) -> usize {
                self.len
            }

            pub fn is_empty(&self) -> bool {
                self.len == 0
            }
        }

        impl<'a, B: AsRef<[u8]>, T: WriteTo<'a, Input: ToUsize>, Item> WriteTo<'a> for $t<B, T, Item> {
            type Input = $t<B, T, Item>;

            fn write_to<W: std::io::Write>(input: &Self::Input, mut out: W) -> Result<(), Error> {
                T::write_to(&T::Input::from_usize(input.len), &mut out)?;
                out.write_all(input.data.as_ref())?;
                Ok(())
            }
        }

        #[cfg(feature = "tokio")]
        impl<'a, B: AsRef<[u8]>, T: crate::message::WriteToAsync<'a, Input: ToUsize>, Item> crate::message::WriteToAsync<'a> for $t<B, T, Item> {
            async fn write_to_async<W: tokio::io::AsyncWriteExt + Unpin>(input: &Self::Input, mut out: W) -> crate::message::Result<()> {
                T::write_to_async(&T::Input::from_usize(input.len), &mut out).await?;
                out.write_all(input.data.as_ref()).await?;
                Ok(())
            }
        }
    };
}

pub(crate) use impl_list_base;
