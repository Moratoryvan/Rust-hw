# MyRedis

## 如何在本地构建使用

```
$ git clone https://github.com/Moratoryvan/Rust-hw.git
$ cd Rust-hw/hw5
$ volo init my_redis idl/my_redis.thrift
$ cargo unpdate 
$ cargo build 
$ cargo run --bin server        #运行服务端
$ cargo run --bin client [cmd]  #运行客户端
```

## 用法

`cmd` 格式

* `ping` 
  ```
  ping
  ```
  返回 `pong` 
* `set`
  ```
  set key value 
  ```
  将 `key`` 的值设置成 `value`` 
* `get`
  ```
  get key
  ```  
  返回 `key` 的值，若不存在，则返回 `Key not found.`
* `del`
  ```
  del key
  ```
  删除 `key`, 若 `key` 不存在，则返回 `Key not found.`
* `subscribe`
  ```
  subscribe channel
  ```
  订阅频道
* `publish`
  ```
  publish channel message
  ```
  向 channel 发送信息，如果当前没有订阅的频道，则返回错误

## 文件路径

```
hw5-myredis/
├── idl
│   └── my_redis.thrift     # Thrift IDL 文件
├── src
│    ├── lib.rs                 # 服务器端实现
│    └── bin
│         ├── client.rs         # 客户端程序
│         └── server.rs         # 服务器程序
├── volo-gen
│    ├── build.rs
│    ├── Cargo.toml
│    ├── src
│    │    └── lib.rs
│    └── volo.yml
├── Cargo.lock
├── Cargo.toml
└── README
```
