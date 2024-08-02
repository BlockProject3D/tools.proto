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
use testprog::bits::Numbers;

#[test]
fn numbers() {
    let mut nums = Numbers::new_on_stack();
    assert_eq!(nums.size(), 4);
    nums.set_a(-8).set_b(15).set_c(-65536).set_d(127);
    assert_eq!(nums.get_a(), -8);
    assert_eq!(nums.get_b(), 15);
    assert_eq!(nums.get_c(), -65536);
    assert_eq!(nums.get_d(), 127);
    nums.set_c(65535);
    assert_eq!(nums.get_c(), 65535);
    nums.set_a(-7);
    assert_eq!(nums.get_a(), -7);
    assert_eq!(nums.get_raw_a(), 9);
    nums.set_a(-6);
    assert_eq!(nums.get_a(), -6);
    assert_eq!(nums.get_raw_a(), 10);
    nums.set_a(-5);
    assert_eq!(nums.get_a(), -5);
    assert_eq!(nums.get_raw_a(), 11);
    nums.set_a(1);
    assert_eq!(nums.get_a(), 1);
    assert_eq!(nums.get_raw_a(), 1);
    nums.set_a(4);
    assert_eq!(nums.get_a(), 4);
    assert_eq!(nums.get_raw_a(), 4);

}

#[test]
fn numbers_raw() {
    let mut nums = Numbers::new_on_stack();
    nums.set_raw_a(15);
    nums.set_raw_b(15);
    nums.set_raw_c(65536);
    nums.set_raw_d(127);
    nums.set_raw_c(65536);
    assert_eq!(nums.get_raw_a(), 15);
    assert_eq!(nums.get_a(), -1);
    assert_eq!(nums.get_raw_b(), 15);
    assert_eq!(nums.get_raw_c(), 65536);
    assert_eq!(nums.get_raw_d(), 127);
    let mut cur_a = 0;
    for i in 0..15 {
        nums.set_raw_a(i);
        if i > 7 && cur_a > 0 {
            cur_a = -8
        }
        assert_eq!(nums.get_a(), cur_a);
        cur_a += 1;
    }
}
