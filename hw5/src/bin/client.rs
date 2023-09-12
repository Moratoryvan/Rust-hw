use lazy_static::lazy_static;
use pilota::FastStr;
// use tracing::Subscriber;
// use tracing_subscriber::FmtSubscriber;
// use volo::Unwrap;
use volo_gen::my_redis::{RedisRequest, RequestType};
use std::net::SocketAddr;
use volo_example::LogLayer;

lazy_static! {
    static ref CLIENT: volo_gen::my_redis::RedisServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::my_redis::RedisServiceClientBuilder::new("MyRedis")
            .layer_outer(LogLayer)  
            .address(addr)
            .build()
    };
}

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut args: Vec<String> = std::env::args().collect();

    let req = match args[1].to_lowercase().as_str() {
        "set" => {
            RedisRequest{
                key: Some(FastStr::from(args.remove(2))),
                value: Some(FastStr::from(args.remove(2))),
                request_type: RequestType::Set,
            }
        },

        "get" => {
            RedisRequest{
                key: Some(FastStr::from(args.remove(2))),
                value: None,
                request_type: RequestType::Get,
            }
        },

        "del" => {
            RedisRequest{
                key: Some(FastStr::from(args.remove(2))),
                value: None,
                request_type: RequestType::Del,
            }
        },

        "ping" => {
            RedisRequest{
                key: None,
                value: None,
                request_type: RequestType::Ping,
            }
        }, 
        "subscribe" => {
            RedisRequest{
                key: Some(FastStr::from(args.remove(2))),
                value: None,
                request_type: RequestType::Subscribe,
            }
            
        }

        "publish" => {
            RedisRequest{
                key: Some(FastStr::from(args.remove(2))),
                value: Some(FastStr::from(args.remove(2))),
                request_type: RequestType::Publish,
            }
        }
        _ => {
            panic!("Unknown command");
        },
    };

    let resp = CLIENT.redis_command(req).await;
    match resp {
        Ok(info) => {
            match info.response_type {
                volo_gen::my_redis::ResponseType::Get => {
                    println!("{}", info.value.unwrap());
                },

                volo_gen::my_redis::ResponseType::Set => {
                    println!("{}", info.value.unwrap());
                },

                volo_gen::my_redis::ResponseType::Del => {
                    println!("{}", info.value.unwrap());
                },

                volo_gen::my_redis::ResponseType::Ping => {
                    println!("{}", info.value.unwrap());
                },

                volo_gen::my_redis::ResponseType::Subscribe => {
                    println!("{}", info.value.unwrap());
                },

                volo_gen::my_redis::ResponseType::Publish => {
                    let message: String = info.value.clone().unwrap().into();
                    if message != "Error" {
                        println!("Publish sucess. The number of subscriber is {}", message);
                    } else {
                        println!("There is no subscriber.");
                    }
                    
                }
            }
        }
        Err(e) => tracing::error!("{:?}", e),
    }
}

