#![feature(impl_trait_in_assoc_type)]

use std::{collections::HashMap, sync::Mutex};
use pilota::FastStr;
use volo_gen::my_redis ::{RequestType,ResponseType,RedisResponse};
// use anyhow::Error;
use anyhow::anyhow;
use tokio::sync::broadcast::{Sender, self};
pub struct S {
    pub map: Mutex<HashMap<String,String>>,
    pub channels: Mutex<HashMap<String, Sender<String>>>,
}

#[volo::async_trait]
impl volo_gen::my_redis::RedisService for S 
{
    async fn redis_command(&self, req: volo_gen::my_redis::RedisRequest) 
        -> ::core::result::Result<volo_gen::my_redis ::RedisResponse, ::volo_thrift::AnyhowError>
    {
        match req.request_type {
            RequestType::Set => {
                self.map.lock().unwrap().insert(req.key.unwrap().into_string(), req.value.unwrap().into_string());
                Ok(RedisResponse { response_type: (ResponseType::Set), value: (Some("Ok".into())) })
            },

            RequestType::Get => {
                if let Some(value) = self.map.lock().unwrap().get(&req.key.unwrap().into_string()) {
                    Ok(RedisResponse { response_type: (ResponseType::Get), value: (Some(FastStr::from(value.clone()))) })
                } else {
                    Ok(RedisResponse { response_type: (ResponseType::Get), value: (Some("Key not found".into())) })
                }
            },

            RequestType::Del => {
                if let Some(_) = self.map.lock().unwrap().remove(&req.key.unwrap().into_string()) {
                    Ok(RedisResponse { response_type: (ResponseType::Del), value: (Some("OK".into())) })
                } else {
                    Ok(RedisResponse { response_type: (ResponseType::Del), value: (Some("Key not found".into())) })
                }
            },

            RequestType::Ping => {
                Ok(RedisResponse{
					value: Some("Pong".into()),
					response_type: ResponseType::Ping
				})
            },

            RequestType::Subscribe => {
                
                let key: String = req.key.unwrap().into();
                let (tx, mut rx) = broadcast::channel(16);
                let has_channel: bool ;
                if let Some(tx) =  self.channels.lock().unwrap().get(&key)  {
                    has_channel = true;
                    rx = tx.subscribe();
                    
                } else {
                    has_channel = false;
                    
                }
                if has_channel {
                    let mes = rx.recv().await;
                    match mes {
                        Ok(m) => {
                            Ok(RedisResponse{
                                value: Some(m.clone().into()),
                                response_type: ResponseType::Subscribe
                            })
                        }
                        Err(_e) => {
                            Ok(RedisResponse{
                                value: Some("Error".into()),
                                response_type: ResponseType::Subscribe
                            })
                        }
                    }
                } else {
                    self.channels.lock().unwrap().insert(key, tx);
                    let mes = rx.recv().await;
                    match mes {
                        Ok(m) => {
                            Ok(RedisResponse{
                                value: Some(m.clone().into()),
                                response_type: ResponseType::Subscribe
                            })
                        }
                        Err(_e) => {
                            Ok(RedisResponse{
                                value: Some("Error".into()),
                                response_type: ResponseType::Subscribe
                            })
                        }
                    }
                }
                
            }

            RequestType::Publish => {
                let key: String = req.key.unwrap().into();
                if let Some(tx) = self.channels.lock().unwrap().get(&key) {
                    let info = tx.send(req.value.unwrap().into_string());
                    match info {
                        Ok(num) => {
                            Ok(RedisResponse{
                                value: Some( FastStr::from((num as u8).to_string())),
                                response_type: ResponseType::Publish
                            })
                        },
                        Err(_e) => {
                            Ok(RedisResponse{
                                value: Some("Error".into()),
                                response_type: ResponseType::Publish
                            })
                        },
                    }
                } else {
                    Ok(RedisResponse{
                        value: Some("Error".into()),
                        response_type: ResponseType::Publish
                    })
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct LogService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for LogService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug ,
    anyhow::Error: Into<S::Error>,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let now = std::time::Instant::now();
        tracing::debug!("Received request {:?}", &req);
        let info :Vec<char> = format!("{req:?}").chars().collect();

        for c in info {
            if c == 'M' {
                return Err(anyhow!("reject").into());   
            }
        }

        let resp = self.0.call(cx, req).await;
        tracing::debug!("Sent response {:?}", &resp);
        tracing::info!("Request took {}ms", now.elapsed().as_millis());
        resp
    }
}

pub struct LogLayer;
impl<S> volo::Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(self, inner: S) -> Self::Service {
        LogService(inner)
    }
}
