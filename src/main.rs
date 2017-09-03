extern crate ordered_float;
extern crate thrift;
extern crate try_from;

mod kv_server;
use std::collections::HashMap;
use std::convert::{From};
use std::default::Default;


use thrift::server::TServer;
use std::sync::RwLock;
use std::result::Result;
use std::ops::{DerefMut, Deref};
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

struct KVStore {
    store: HashMap<String, String>,
}

impl KVStore{
    fn new() -> KVStore {
        KVStore{store: HashMap::new()}
    }
}

struct KVServer {
    store: RwLock<KVStore>
}

impl Default for KVServer {
    fn default() -> KVServer {
        KVServer { store: RwLock::new(KVStore::new()) }
    }
}

impl KVServerSyncHandler for KVServer {
    fn handle_set_key(&self, kv: KVObject) -> Result<bool, thrift::Error> {
        let key = kv.key.unwrap();
        let value = kv.value.unwrap();
        println!("Storing value: {}  into key: {}", value, key);
        let mut store = self.store.write().unwrap();
        {
            let w_store = store.deref_mut();
            w_store.store.insert(key, value);
        }
        Ok(true)
    }

    fn handle_get_val(&self, key: String)  -> Result<std::string::String, thrift::Error>{
         let store = self.store.read().unwrap();
         let r_store = store.deref();
         if r_store.store.contains_key(&key) {
             let val = r_store.store.get(&key).unwrap().clone();
             Ok(val)
         } else {
             Err(thrift::Error::from(KeyNotFound{ key: Some(key) }))
         }
    }


    fn handle_get_obj(&self, key: String)  -> Result<kv_server::KVObject, thrift::Error> {
        let store = self.store.read().unwrap();
        let r_store = store.deref();
        if r_store.store.contains_key(&key) {
            let val = r_store.store.get(&key).unwrap().clone();
            let obj = kv_server::KVObject{key: Some(key), value: Some(val)};
            Ok(obj)
        } else {
            Err(thrift::Error::from(KeyNotFound{ key: Some(key) }))
        }
    }


    fn handle_del_key(&self, key: String) -> Result<(), thrift::Error> {
        let mut store = self.store.write().unwrap();
        {
            println!("Removing key: {}", key);
            let w_store = store.deref_mut();
            w_store.store.remove(&key);
        }
        Ok(())
    }
}
