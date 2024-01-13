#![feature(extern_types)]
#![allow(mutable_transmutes)]
#![feature(c_variadic)]
#![feature(ascii_char)]

extern crate core;

pub mod fast_session;
pub mod fast_feed;
pub mod order_book;
pub mod fast_message;
pub mod fast_server;
pub mod fast_client;
pub mod test;
pub mod fast_template;
pub mod fast_orderbook;


use lib;
