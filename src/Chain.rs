use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


#[derive(Serialize, Deserialize)]
pub struct Block {
    pub id: u32,
    data: Vec<u8>,
    prevHash: String
}

impl Block {
    pub fn new(id: u32, data: Vec<u8>) -> Block {
        let mut block = Block {
            id: id,
            data: data,
            prevHash: "TheseWordsWit".to_string()
        };
        block
    }

    pub fn addData(&mut self,mut data: Vec<u8>) {
        self.data.append(&mut data);
    }

    pub fn clone(&self) -> Block {
        let mut block = Block {
            id: self.id,
            data: self.data.clone(),
            prevHash: self.prevHash.clone()
        };
        block
    }
}
impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.data.hash(state);
        self.prevHash.hash(state);
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

    pub fn addBlock(&mut self,mut block: Block) {
        if (self.blocks.len() > 0) {
            let prevBlock : &Block = self.blocks.last().unwrap();
            block.id = prevBlock.id + 1;
            let mut hasher = DefaultHasher::new();
            prevBlock.hash(&mut hasher);
            block.prevHash = hasher.finish().to_string();
        }else {
            block.id = 0;
        }
        self.blocks.push(block);
    }

    pub fn getName(&self) -> String {
        self.name.clone()
    }

    pub fn to_string(&self) -> String {
        let mut chain_string = serde_json::to_string(&self).unwrap();

        chain_string
    }

}


//make a simple test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut chain = Chain::new("test".to_string());
        let mut block = Block::new(0, vec![1,2,3]);
        chain.addBlock(block);
        assert_eq!(chain.getName(), "test".to_string());
    }
}