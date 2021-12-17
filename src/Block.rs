use serde::{Deserialize, Serialize};


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
            prevHash: "".to_string()
        };
        block
    }

    pub fn addData(&mut self,mut data: Vec<u8>) {
        self.data.append(&mut data);
    }
}