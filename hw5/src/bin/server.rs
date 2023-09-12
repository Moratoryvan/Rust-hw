#![feature(impl_trait_in_assoc_type)]
use std::collections::HashMap;

use std::net::SocketAddr;
use std::sync::Mutex;
use tokio::sync::broadcast::Sender;
use volo_example::LogLayer;
use volo_example::S;



#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    volo_gen::my_redis::RedisServiceServer::new(S{
        map: Mutex::new(HashMap::<String,String>::new()),
        channels: Mutex::new(HashMap::<String,Sender<String>>::new()),
    })
    .layer_front(LogLayer)
    .run(addr)
    .await
    .unwrap();


}

