use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


#[derive(Serialize, Deserialize)]
pub struct Block {
    pub id: u32,
    data: Vec<Vec<u8>>,
    prev_hash: String
}

impl Block {
    pub fn new(id: u32, data: Vec<Vec<u8>>) -> Block {
        let block = Block {
            id: id,
            data: data,
            prev_hash: "".to_string()
        };
        block
    }

    pub fn add_data(&mut self,data: Vec<u8>) {
        self.data.push(data);

    }

    pub fn get_data(&self) -> Vec<Vec<u8>> {
        self.data.clone()
    }

    pub fn get_data_string(&self) -> Vec<String> {
        let mut result = Vec::new();
        for data in  self.data.clone() {
            result.push(String::from_utf8(data).unwrap());
        }
        result
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_prevhash(&self) -> String {
        self.prev_hash.clone()
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
            // block.hash(&mut hasher);
            // block.prev_hash = hasher.finish().to_string();
            block.prev_hash = "".to_string();

        }
        self.blocks.push(block);
    }

    pub fn get_data(&self) -> Vec<Vec<Vec<u8>>> {
        let mut result : Vec<Vec<Vec<u8>>>   = Vec::new();
        for block in self.blocks.iter() {
            result.push(block.get_data());
        }
        result
    }

    pub fn get_length(&self) -> u32 {
        self.blocks.len() as u32
    }

    pub fn get_block(&self, id: u32) -> Option<Block> {
        for block in self.blocks.iter() {
            if block.id == id {
                return Some(block.clone());
            }
        }
        None
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn to_string(&self) -> String {
        let chain_string = serde_json::to_string(&self).unwrap();

        chain_string
    }

    pub fn verify(&self) -> bool {
        let mut hasher = DefaultHasher::new();
        let mut prev_hash = "".to_string();
        for block in self.blocks.iter() {
            if prev_hash != block.prev_hash {
                return false;
            }
            block.hash(&mut hasher);
            prev_hash = hasher.finish().to_string();
        }
        true
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut chain = Chain::new("test".to_string());
        let blockPart = vec![1,2,3];
        let block = Block::new(0, vec![blockPart.clone()]);
        chain.add_block(block);
        assert_eq!(chain.get_name(), "test".to_string());
    }
}