use super::block::Block;

type Blocks = Vec<Block>;

// `Blockchain` A struct that represents the blockchain.
#[derive(Debug, Clone)]
pub struct Blockchain {
    // The storage for blocks.
    pub chain: Blocks,
    // Minimum amount of work required to validate
    pub difficulty: usize
}
impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        // First block in the chain.
        let genesis_block = Block::new(
            0,
            String::default(),
            "genesis block".to_string(),
        );
        // Create the blockchain Instance.
        Blockchain {
            chain: vec![genesis_block],
            difficulty,
        }
    }

    pub fn add_block(&mut self, data: String) {
        let mut new_block = Block::new(
            self.chain.len() as u64,
            self.chain[&self.chain.len()-1].hash.clone(),
            data,
        );        
        new_block.mine(self);
        self.chain.push(new_block);
        println!("New block added to chain -> {}", serde_json::to_string_pretty(&(self.chain.last().unwrap())).unwrap());
    }

    pub fn is_chain_valid(&self) -> bool {
        let mut index = 0;
        let chain_len = self.chain.len();
        loop {
            let block = &self.chain[index];
            let computed_hash = block.calculate_hash();
            if !computed_hash.eq(&block.hash) { return false }
            if index >= chain_len - 1 { break }
            else if computed_hash != self.chain[index+1].previous_hash { return false }
            index += 1;
        }
        true
    }
}