// xfail-fast

// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Examples from Eric's internship final presentation.
//
// Code is easier to write in emacs, and it's good to be sure all the
// code samples compile (or not) as they should.

#[legacy_records];

use double_buffer::client::*;
use double_buffer::give_buffer;

macro_rules! select_if (
    {
        $index:expr,
        $count:expr,
        $port:path => [
            $($message:path$(($($x: ident),+))dont_type_this*
              -> $next:ident $e:expr),+
        ],
        $( $ports:path => [
            $($messages:path$(($($xs: ident),+))dont_type_this*
              -> $nexts:ident $es:expr),+
        ], )*
    } => {
        if $index == $count {
            match move pipes::try_recv(move $port) {
              $(Some($message($($(move $x,)+)* move next)) => {
                let $next = move next;
                move $e
              })+
              _ => die!()
            }
        } else {
            select_if!(
                $index,
                $count + 1,
                $( $ports => [
                    $($messages$(($($xs),+))dont_type_this*
                      -> $nexts $es),+
                ], )*
            )
        }
    };

    {
        $index:expr,
        $count:expr,
    } => {
        die!()
    }
)

macro_rules! select (
    {
        $( $port:path => {
            $($message:path$(($($x: ident),+))dont_type_this*
              -> $next:ident $e:expr),+
        } )+
    } => ({
        let index = pipes::selecti([$(($port).header()),+]);
        select_if!(index, 0, $( $port => [
            $($message$(($($x),+))dont_type_this* -> $next $e),+
        ], )+)
    })
)

// Types and protocols
pub struct Buffer {
    foo: (),

}

pub impl Buffer : Drop {
    fn finalize(&self) {}
}

proto! double_buffer (
    acquire:send {
        request -> wait_buffer
    }

    wait_buffer:recv {
        give_buffer(::Buffer) -> release
    }

    release:send {
        release(::Buffer) -> acquire
    }
)

// Code examples
fn render(_buffer: &Buffer) {
    // A dummy function.
}

fn draw_frame(+channel: double_buffer::client::acquire) {
    let channel = request(move channel);
    select! (
        channel => {
            give_buffer(buffer) -> channel {
                render(&buffer);
                release(move channel, move buffer)
            }
        }
    );
}

fn draw_two_frames(+channel: double_buffer::client::acquire) {
    let channel = request(move channel);
    let channel = select! (
        channel => {
            give_buffer(buffer) -> channel {
                render(&buffer);
                release(move channel, move buffer)
            }
        }
    );
    let channel = request(move channel);
    select! (
        channel => {
            give_buffer(buffer) -> channel {
                render(&buffer);
                release(move channel, move buffer)
            }
        }
    );
}

#[cfg(bad1)]
fn draw_two_frames_bad1(+channel: double_buffer::client::acquire) {
    let channel = request(channel);
    select! (
        channel => {
            give_buffer(buffer) -> channel {
                render(&buffer);
            }
        }
    );
    let channel = request(channel);
    select! (
        channel => {
            give_buffer(buffer) -> channel {
                render(&buffer);
                release(channel, move buffer)
            }
        }
    );
}

#[cfg(bad2)]
fn draw_two_frames_bad2(+channel: double_buffer::client::acquire) {
    let channel = request(channel);
    select! (
        channel => {
            give_buffer(buffer) -> channel {
                render(&buffer);
                release(channel, move buffer);
                render(&buffer);
                release(channel, move buffer);
            }
        }
    );
}

pub fn main() { }
