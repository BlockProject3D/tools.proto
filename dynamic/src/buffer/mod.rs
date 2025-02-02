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

mod view;
mod buffer;
mod bytes;
mod builder;

pub use view::BufferView;
pub use builder::Builder;

#[cfg(test)]
mod tests {
    use crate::buffer::builder::Builder;

    #[test]
    fn basic() {
        let mut view = Builder::new("test")
            .add_child(
                Builder::new("hdr")
                    .fixed(0, 8)
                    .add_child(Builder::new("inner1").fixed(0, 4))
                    .add_child(Builder::new("inner2").fixed(4, 4))
            ).build();
        view["hdr.inner1"].set_bytes(b"abcd");
        view["hdr.inner2"].set_bytes(b"efgh");
        view.shape().unwrap();
        view.copy_from(b"12345678");
        assert_eq!(view["hdr.inner1"].as_bytes(), b"1234");
        assert_eq!(view["hdr.inner2"].as_bytes(), b"5678");
        println!("{:?}", view);
        println!("{:?}", view["hdr.inner1"]);
        assert!(view.get("hdr.inner1[0]").is_none());
        view.copy_from(b"abcdefgh");
        assert_eq!(view["hdr.inner2"].get_path(), "test.hdr.inner2");
        assert_eq!(view["hdr.inner1"].get_path(), "test.hdr.inner1");
        assert_eq!(view["hdr.inner1"].as_bytes(), b"abcd");
        assert_eq!(view["hdr.inner2"].as_bytes(), b"efgh");
    }
}
