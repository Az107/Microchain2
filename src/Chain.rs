use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


#[derive(Serialize, Deserialize)]
pub struct Block {
    pub id: u32,
    data: Vec<u8>,
    prev_hash: String
}

impl Block {
    pub fn new(id: u32, data: Vec<u8>) -> Block {
        let block = Block {
            id: id,
            data: data,
            prev_hash: "".to_string()
        };
        block
    }

    pub fn add_data(&mut self,mut data: Vec<u8>) {
        self.data.append(&mut data);
    }

    pub fn clone(&self) -> Block {
        let block = Block {
            id: self.id,
            data: self.data.clone(),
            prev_hash: self.prev_hash.clone()
        };
        block
    }
}
impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.data.hash(state);
        self.prev_hash.hash(state);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Chain {
    blocks: Vec<Block>,
    name: String,
    
}

impl Chain {
    pub fn new(name: String) -> Chain {
        Chain {
            blocks: Vec::new(),
            name: name
        }
    }

    pub fn add_block(&mut self,mut block: Block) {
        let mut hasher = DefaultHasher::new();
        if self.blocks.len() > 0 {
            let prev_block : &Block = self.blocks.last().unwrap();
            block.id = prev_block.id + 1;
            prev_block.hash(&mut hasher);
            block.prev_hash = hasher.finish().to_string();
        }else {
            block.id = 0;
            block.hash(&mut hasher);
            block.prev_hash = hasher.finish().to_string();

        }
        self.blocks.push(block);
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn to_string(&self) -> String {
        let chain_string = serde_json::to_string(&self).unwrap();

        chain_string
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut chain = Chain::new("test".to_string());
        let block = Block::new(0, vec![1,2,3]);
        chain.add_block(block);
        assert_eq!(chain.get_name(), "test".to_string());
    }
}