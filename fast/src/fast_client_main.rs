#![feature(extern_types)]
#![allow(mutable_transmutes)]
#![feature(c_variadic)]

use ::libc;
use ::glib;
use ::libxml;

use fast::fast_feed::{self, *};
use fast::fast_message::{self, *};
use fast::fast_session::{self, *};
use fast::order_book::{self, *};
use fast::test::{self, *};
use fast::fast_template::{self, *};
use fast::fast_client::{self, *};
use fast::fast_orderbook::{self, *};

fn main() {
    fast_client::main();
}