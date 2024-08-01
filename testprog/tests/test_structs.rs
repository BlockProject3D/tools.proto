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

use testprog::structs::{Flags, Floats, Numbers};

#[test]
fn numbers() {
    let mut nums = Numbers::new_on_stack();
    nums.set_u_a(0x123456AB).set_a(-424242).set_u_b(0x1234).set_b(-4242).set_u_c(0x12).set_c(-42);
    assert_eq!(nums.get_u_a(), 0x123456AB);
    assert_eq!(nums.get_a(), -424242);
    assert_eq!(nums.get_u_b(), 0x1234);
    assert_eq!(nums.get_b(), -4242);
    assert_eq!(nums.get_u_c(), 0x12);
    assert_eq!(nums.get_c(), -42);
}

#[test]
fn flags() {
    let mut flags = Flags::new_on_stack();
    flags.set_a(true).set_b(true).set_c(true).set_d(true);
    assert!(flags.get_a());
    assert!(flags.get_b());
    assert!(flags.get_c());
    assert!(flags.get_d());
}

#[test]
fn floats() {
    let mut floats = Floats::new_on_stack();
    floats.set_a(4242.0).set_b(4242.4242);
    assert_eq!(floats.get_a(), 4242.0);
    assert_eq!(floats.get_b(), 4242.4242);
}
