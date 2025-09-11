use sha2::{Sha256, Digest};
use chrono::Utc;

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: i64,
    data: String,
    prev_hash: String,
    nonce: u64,
    hash: String,
}

impl Block {
    fn new(index: u64, data: String, prev_hash: String, difficulty: usize) -> Block {
        let timestamp = Utc::now().timestamp();
        let mut nonce = 0;
        let mut hash = String::new();

        loop {
            let input = format!("{}{}{}{}{}", index, timestamp, &data, &prev_hash, nonce);
            let hashed = sha256(&input);

            if hashed.starts_with(&"0".repeat(difficulty)) {
                hash = hashed;
                break;
            }

            nonce += 1;
        }

        Block {
            index,
            timestamp,
            data,
            prev_hash,
            nonce,
            hash,
        }
    }
}

fn sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

fn main() {
    let difficulty = 4; // Adjust difficulty (number of leading 0s)

    let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string(), difficulty);
    println!("Mined Genesis Block: {:#?}", genesis_block);

    let second_block = Block::new(1, "Some transaction data".to_string(), genesis_block.hash.clone(), difficulty);
    println!("Mined Second Block: {:#?}", second_block);
}
