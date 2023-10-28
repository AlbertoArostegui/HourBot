extern crate redis;

use redis::Commands;
use redis::Connection;
use redis::RedisResult;

pub fn establish_connection() -> RedisResult<Connection> {
    let client = redis::Client::open("redis://localhost/")?;
    let conn = client.get_connection()?;
    Ok(conn)
}

pub fn insert_ts(conn: &mut Connection, user_id: i64, ts: u64) -> RedisResult<()> {
    conn.set(user_id, ts)?;
    Ok(())
}

pub fn get_ts(conn: &mut Connection, user_id: i64) -> RedisResult<String> {
    conn.get(user_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redis_operations() {
        let mut conn = establish_connection().unwrap();
        insert_ts(&mut conn, 1234, 5123).unwrap();
        assert_eq!(get_ts(&mut conn, 1234).unwrap(), "5123");
    }
}