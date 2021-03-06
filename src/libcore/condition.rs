// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// helper for transmutation, shown below.
type RustClosure = (int,int);
struct Handler<T, U:Copy> {
    handle: RustClosure,
    prev: Option<@Handler<T, U>>,
}

struct Condition<T, U:Copy> {
    key: task::local_data::LocalDataKey<Handler<T,U>>
}

impl<T, U: Copy>  Condition<T,U> {

    fn trap(&self, h: &self/fn(&T) ->U) -> Trap/&self<T,U> {
        unsafe {
            let p : *RustClosure = ::cast::transmute(&h);
            let prev = task::local_data::local_data_get(self.key);
            let h = @Handler{handle: *p, prev: prev};
            move Trap { cond: self, handler: h }
        }
    }

    fn raise(t:&T) -> U  {
        do self.raise_default(t) {
            fail ~"Unhandled condition";
        }
    }

    fn raise_default(t:&T, default: fn() -> U) -> U {
        unsafe {
            match task::local_data::local_data_pop(self.key) {
                None => {
                    debug!("Condition.raise: found no handler");
                    default()
                }

                Some(handler) => {
                    debug!("Condition.raise: found handler");
                    match handler.prev {
                        None => (),
                        Some(hp) =>
                        task::local_data::local_data_set(self.key, hp)
                    }
                    let handle : &fn(&T) -> U =
                        ::cast::transmute(handler.handle);
                    let u = handle(t);
                    task::local_data::local_data_set(self.key,
                                                     handler);
                    move u
                }
            }
        }
    }
}



struct Trap<T, U:Copy> {
    cond: &Condition<T,U>,
    handler: @Handler<T, U>
}

impl<T, U: Copy> Trap<T,U> {
    fn in<V: Copy>(&self, inner: &self/fn() -> V) -> V {
        unsafe {
            let _g = Guard { cond: self.cond };
            debug!("Trap: pushing handler to TLS");
            task::local_data::local_data_set(self.cond.key, self.handler);
            inner()
        }
    }
}

struct Guard<T, U:Copy> {
    cond: &Condition<T,U>,
    drop {
        unsafe {
            debug!("Guard: popping handler from TLS");
            let curr = task::local_data::local_data_pop(self.cond.key);
            match curr {
                None => (),
                Some(h) =>
                match h.prev {
                    None => (),
                    Some(hp) => {
                        task::local_data::local_data_set(self.cond.key, hp)
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod test {

    fn sadness_key(_x: @Handler<int,int>) { }
    const sadness_condition : Condition<int,int> =
        Condition { key: sadness_key };

    fn trouble(i: int) {
        debug!("trouble: raising conition");
        let j = sadness_condition.raise(&i);
        debug!("trouble: handler recovered with %d", j);
    }

    fn nested_trap_test_inner() {

        let mut inner_trapped = false;

        do sadness_condition.trap(|_j| {
            debug!("nested_trap_test_inner: in handler");
            inner_trapped = true;
            0
        }).in {
            debug!("nested_trap_test_inner: in protected block");
            trouble(1);
        }

        assert inner_trapped;
    }

    #[test]
    fn nested_trap_test_outer() {

        let mut outer_trapped = false;

        do sadness_condition.trap(|_j| {
            debug!("nested_trap_test_outer: in handler");
            outer_trapped = true; 0
        }).in {
            debug!("nested_guard_test_outer: in protected block");
            nested_trap_test_inner();
            trouble(1);
        }

        assert outer_trapped;
    }

    fn nested_reraise_trap_test_inner() {

        let mut inner_trapped = false;

        do sadness_condition.trap(|_j| {
            debug!("nested_reraise_trap_test_inner: in handler");
            inner_trapped = true;
            let i = 10;
            debug!("nested_reraise_trap_test_inner: handler re-raising");
            sadness_condition.raise(&i)
        }).in {
            debug!("nested_reraise_trap_test_inner: in protected block");
            trouble(1);
        }

        assert inner_trapped;
    }

    #[test]
    fn nested_reraise_trap_test_outer() {

        let mut outer_trapped = false;

        do sadness_condition.trap(|_j| {
            debug!("nested_reraise_trap_test_outer: in handler");
            outer_trapped = true; 0
        }).in {
            debug!("nested_reraise_trap_test_outer: in protected block");
            nested_reraise_trap_test_inner();
        }

        assert outer_trapped;
    }

    #[test]
    fn test_default() {

        let mut trapped = false;

        do sadness_condition.trap(|j| {
            debug!("test_default: in handler");
            sadness_condition.raise_default(j, || {trapped=true; 5})
        }).in {
            debug!("test_default: in protected block");
            trouble(1);
        }

        assert trapped;
    }

}
