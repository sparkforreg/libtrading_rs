#![feature(extern_types)]
#![allow(mutable_transmutes)]
#![feature(c_variadic)]

use ::libc;

use fix::fix_common::{self, *};
use fix::fix_message::{self, *};
use fix::fix_session::{self, *};
use fix::test::{self, *};
use fix::fix_server::{self, *};

fn main() {
    fix_server::main();
}