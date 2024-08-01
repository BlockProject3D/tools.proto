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

use bp3d_proto::util::Size;
use testprog::struct_arrays::Basic;

#[test]
fn basic() {
    let mut basic = Basic::new_on_stack();
    assert_eq!(basic.size(), 58);
    basic.get_p3_mut().set_raw(0, 42.42).set_raw(1, 42.42)
        .set_raw(2, 42.42).set_raw(3, 42.42);
    assert_eq!(basic.get_p3().get_raw(0), 42.42);
    assert_eq!(basic.get_p3().get_raw(1), 42.42);
    assert_eq!(basic.get_p3().get_raw(2), 42.42);
    assert_eq!(basic.get_p3().get_raw(3), 42.42);
    basic.set_p1(424242);
    assert_eq!(basic.get_p1(), 424242);
    basic.get_p2_mut().as_mut()[..14].copy_from_slice(b"this is a test");
    assert_eq!(&basic.get_p2().as_ref()[..14], b"this is a test");
    basic.get_p4_mut().set_raw(0, 0xABCDEF).set_raw(1, 0xABCDEF);
    assert_eq!(basic.get_p4().get_raw(0), 0xABCDEF);
    assert_eq!(basic.get_p4().get_raw(1), 0xABCDEF);
}
