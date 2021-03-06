// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
// no-prefer-dynamic
// ignore-emscripten no threads support

static mut HIT: bool = false;

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        unsafe { HIT = true; }
    }
}

thread_local!(static FOO: Foo = Foo);

fn main() {
    std::thread::spawn(|| {
        FOO.with(|_| {});
    }).join().unwrap();
    assert!(unsafe { HIT });
}
