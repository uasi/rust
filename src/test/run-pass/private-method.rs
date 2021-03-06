// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct cat {
  priv mut meows : uint,

  how_hungry : int,
}

impl cat {
  fn play() {
    self.meows += 1u;
    self.nap();
  }
}

priv impl cat {
    fn nap() { for uint::range(1u, 10u) |_i| { }}
}

fn cat(in_x : uint, in_y : int) -> cat {
    cat {
        meows: in_x,
        how_hungry: in_y
    }
}

pub fn main() {
  let nyan : cat = cat(52u, 99);
  nyan.play();
}
