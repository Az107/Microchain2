mod Block;

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

    pub fn addBlock(&mut self, block: Block) {
        let prevBlock : Block = self.blocks.last().as_deref();
        block.id = prevBlock.id + 1;
        self.blocks.push(block);
    }

    pub fn getName(&self) -> String {
        self.name.clone()
    }

}