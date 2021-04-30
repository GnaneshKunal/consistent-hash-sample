use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

pub const RING_DIVIDER: f64 = 27782217.355555557;

pub type Port = u16;

pub const REPLICA_SIZE: u8 = 3;

mod bisect;

use bisect::Bisect;

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn calculate_degree<T: Hash>(t: &T) -> u64 {
    (calculate_hash(t) as f64 / RING_DIVIDER) as u64
}

#[derive(Debug)]
pub struct ConsistentHash {
    keys: Bisect<u64>,
    nodes: HashMap<u64, Port>,
}

impl ConsistentHash {
    pub fn new() -> Self {
        Self {
            keys: Bisect::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn get_replicas(port: Port) -> Vec<(String, u64)> {
        (0..REPLICA_SIZE)
            .map(|r| {
                let name = format!("{}:{}", port, r);
                let hash = calculate_degree(&name);
                (name, hash)
            })
            .collect()
    }

    pub fn add_node(&mut self, port: Port) {
        let replicas = ConsistentHash::get_replicas(port);

        for (_, replica_hash) in replicas {
            self.keys.append(replica_hash);

            self.nodes.insert(replica_hash, port);
        }
    }

    pub fn remove_node(&mut self, port: Port) {
        let replicas = ConsistentHash::get_replicas(port);
        for (_, replica_hash) in replicas {
            self.nodes.remove(&replica_hash);
            let index = self.keys.bisect_left(&replica_hash);
            self.keys.remove(index);
        }
    }

    pub fn get_client(&self, key: &str) -> Result<redis::Client, redis::RedisError> {
        let key_hash = calculate_degree(&key);

        let start = self.keys.bisect_right(&key_hash);

        let port = if start == self.keys.len() {
            self.nodes.get(&self.keys.get(0).unwrap()).unwrap()
        } else {
            self.nodes.get(&self.keys.get(start).unwrap()).unwrap()
        };

        redis::Client::open(format!("redis://127.0.0.1:{}/", port))
    }
}
