pub struct Block {
    id: u32,
    data: Vec<v8>,
    prevHash: String
}

impl Block {
    pub fn new(id: u32, data: Vec<v8>) -> Block {
        let mut block = Block {
            id: id,
            data: data,
        };
        block
    }

    pub fn addData(&mut self, data: Vec<v8>) {
        self.data.append(&mut data);
    }
}