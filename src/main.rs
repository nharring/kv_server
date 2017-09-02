extern crate ordered_float;
extern crate thrift;
extern crate try_from;

mod kv_server;

use std::cell::RefCell;
use std::rc::Rc;
use thrift::protocol::{TInputProtocolFactory, TOutputProtocolFactory};
use thrift::protocol::{TCompactInputProtocolFactory, TCompactOutputProtocolFactory};
use kv_server::{KeyNotFound, ServiceException};
use kv_server::{KVObject};

fn main() {
    println!("Hello, world!");
}
