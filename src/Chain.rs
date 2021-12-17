use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};

#[path = "Block.rs"] mod Block;
type tBlock = Block::Block;

#[derive(Serialize, Deserialize)]
pub struct Chain {
    blocks: Vec<tBlock>,
    name: String,
    
}
impl Chain {
    pub fn new(name: String) -> Chain {
        Chain {
            blocks: Vec::new(),
            name: name
        }
    }

    pub fn addBlock(&mut self,mut block: tBlock) {
        let prevBlock : &tBlock = self.blocks.last().unwrap();
        block.id = prevBlock.id + 1;
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
        let mut block = Block::Block::new(0, vec![1,2,3]);
        chain.addBlock(block);
        assert_eq!(chain.getName(), "test".to_string());
    }
}