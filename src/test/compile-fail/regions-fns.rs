// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Before fn subtyping was properly implemented,
// we reported errors in this case:

fn not_ok(a: &uint, b: &b/uint) {
    let mut g: fn@(x: &uint) = fn@(x: &b/uint) {};
    //~^ ERROR mismatched types
    g(a);
}

fn main() {
}
