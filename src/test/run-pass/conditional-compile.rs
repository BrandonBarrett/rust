// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Crate use statements
#[cfg(bogus)]
use flippity;

#[cfg(bogus)]
const b: bool = false;

const b: bool = true;

#[cfg(bogus)]
#[abi = "cdecl"]
extern mod rustrt {
    #[legacy_exports];
    // This symbol doesn't exist and would be a link error if this
    // module was translated
    fn bogus();
}

#[abi = "cdecl"]
extern mod rustrt {
    #[legacy_exports]; }

#[cfg(bogus)]
type t = int;

type t = bool;

#[cfg(bogus)]
enum tg { foo, }

enum tg { bar, }

#[cfg(bogus)]
struct r {
  i: int,
}

#[cfg(bogus)]
fn r(i:int) -> r {
    r {
        i: i
    }
}

struct r {
  i: int,
}

fn r(i:int) -> r {
    r {
        i: i
    }
}

#[cfg(bogus)]
mod m {
    #[legacy_exports];
    // This needs to parse but would fail in typeck. Since it's not in
    // the current config it should not be typechecked.
    fn bogus() { return 0; }
}

mod m {
    #[legacy_exports];

    // Submodules have slightly different code paths than the top-level
    // module, so let's make sure this jazz works here as well
    #[cfg(bogus)]
    fn f() { }

    fn f() { }
}

// Since the bogus configuration isn't defined main will just be
// parsed, but nothing further will be done with it
#[cfg(bogus)]
fn main() { fail }

fn main() {
    // Exercise some of the configured items in ways that wouldn't be possible
    // if they had the bogus definition
    assert (b);
    let x: t = true;
    let y: tg = bar;

    test_in_fn_ctxt();
}

fn test_in_fn_ctxt() {
    #[cfg(bogus)]
    fn f() { fail }
    fn f() { }
    f();

    #[cfg(bogus)]
    const i: int = 0;
    const i: int = 1;
    assert (i == 1);
}

mod test_foreign_items {
    #[legacy_exports];
    #[abi = "cdecl"]
    extern mod rustrt {
        #[legacy_exports];
        #[cfg(bogus)]
        fn rust_getcwd() -> ~str;
        fn rust_getcwd() -> ~str;
    }
}

mod test_use_statements {
    #[legacy_exports];
    #[cfg(bogus)]
    use flippity_foo;

    extern mod rustrt {
        #[legacy_exports];
        #[cfg(bogus)]
        use flippity_foo;
    }
}

mod test_methods {
    struct Foo {
        bar: uint
    }

    impl Foo: Fooable {
        #[cfg(bogus)]
        static fn what() { }

        static fn what() { }

        #[cfg(bogus)]
        fn the() { }

        fn the() { }
    }

    trait Fooable {
        #[cfg(bogus)]
        static fn what();

        static fn what();

        #[cfg(bogus)]
        fn the();

        fn the();
    }
}
