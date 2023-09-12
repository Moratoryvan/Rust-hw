namespace rs my_redis

enum RequestType {
    Get,
    Set,
    Del,
    Ping,
    Subscribe,
    Publish,
}

struct RedisRequest {
    1: optional string key,
    2: optional string value,
    3: required RequestType request_type,

}

enum ResponseType {
    Get,
    Set,
    Del,
    Ping,
    Subscribe,
    Publish,
}

struct RedisResponse {
    1: required ResponseType response_type,
    2: optional string value,
}

service RedisService {
    RedisResponse RedisCommand(1: RedisRequest req),
}