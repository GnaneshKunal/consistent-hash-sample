use ch::ConsistentHash;

use redis::Commands;

fn main() -> Result<(), redis::RedisError> {
    let mut m = ConsistentHash::new();

    m.add_node(6379);
    m.add_node(6380);
    m.add_node(6381);

    let client = m.get_client("hello")?;
    let mut conn: redis::Connection = client.get_connection()?;

    let _: () = conn.set("hello", "world")?;
    println!("{:?}", conn.get::<&str, String>("hello")?);

    Ok(())
}
