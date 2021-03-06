// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-win32
extern mod std;

struct complainer {
  c: comm::Chan<bool>,
}

impl complainer : Drop {
    fn finalize(&self) {
        error!("About to send!");
        comm::send(self.c, true);
        error!("Sent!");
    }
}

fn complainer(c: comm::Chan<bool>) -> complainer {
    error!("Hello!");
    complainer {
        c: c
    }
}

fn f(c: comm::Chan<bool>) {
    let _c = move complainer(c);
    fail;
}

fn main() {
    let p = comm::Port();
    let c = comm::Chan(&p);
    task::spawn_unlinked(|| f(c) );
    error!("hiiiiiiiii");
    assert comm::recv(p);
}
