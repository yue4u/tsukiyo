extern crate redis;
use redis::Commands;

pub fn incr_page_views() -> redis::RedisResult<i32> {
    let client = redis::Client::open("redis://redis")?;
    let mut con = client.get_connection()?;
    con.incr("views",1)
}

pub fn get_page_views() -> redis::RedisResult<i32> {
    let client = redis::Client::open("redis://redis")?;
    let mut con = client.get_connection()?;
    con.get("views")
}
