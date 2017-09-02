extern crate ordered_float;
extern crate thrift;
extern crate try_from;

mod kv_server;
use std::collections::HashMap;
use std::convert::{From, Into};
use std::default::Default;


use thrift::server::TServer;
use std::cell::RefCell;
use std::rc::Rc;
use thrift::protocol::{TInputProtocolFactory, TOutputProtocolFactory};
use thrift::protocol::{TCompactInputProtocolFactory, TCompactOutputProtocolFactory};
use thrift::transport::{TFramedReadTransportFactory, TFramedWriteTransportFactory};
use kv_server::{KeyNotFound, ServiceException};
use kv_server::{KVObject};
use kv_server::{KVServerSyncHandler, KVServerSyncProcessor};

fn main() {
    match run() {
        Ok(()) => println!("tutorial server ran successfully"),
        Err(e) => {
            println!("tutorial server failed with error {:?}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> thrift::Result<()> {

    let listen_address = format!("127.0.0.1:6677");

    println!("binding to {}", listen_address);

    let i_tran_fact: Box<TFramedReadTransportFactory> = Box::new(TFramedReadTransportFactory::new());
    let i_prot_fact: Box<TInputProtocolFactory> = Box::new(TCompactInputProtocolFactory::new());

    let o_tran_fact: Box<TFramedWriteTransportFactory> = Box::new(TFramedWriteTransportFactory::new());
    let o_prot_fact: Box<TOutputProtocolFactory> = Box::new(TCompactOutputProtocolFactory::new());

    // demux incoming messages
    let processor = KVServerSyncProcessor::new(KVServer { ..Default::default() });

    // create the server and start listening
    let mut server = TServer::new(i_tran_fact,
                                        i_prot_fact,
                                        o_tran_fact,
                                        o_prot_fact,
                                        processor,
                                        10);

    server.listen(&listen_address)
}

struct KVServer {
    log: HashMap<i32, KVObject>,
}

impl Default for KVServer {
    fn default() -> KVServer {
        KVServer { log: HashMap::new() }
    }
}

impl KVServerSyncHandler for KVServer {
    fn handle_set_key(&self, kv: kv_server::KVObject) -> std::result::Result<bool, thrift::Error> {
         unimplemented!();
    }

    fn handle_get_val(&self, key: String)  -> std::result::Result<std::string::String, thrift::Error>{
         unimplemented!();
    }


    fn handle_get_obj(&self, key: String)  -> std::result::Result<kv_server::KVObject, thrift::Error>{
         unimplemented!();
    }


    fn handle_del_key(&self, key: String) -> std::result::Result<(), thrift::Error> {
         unimplemented!();
    }
}
