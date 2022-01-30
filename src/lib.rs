use neon::prelude::*;
use std::io::Write;
use std::fs::File;
use std::fs;
use std::cell::RefCell;


mod chain;


type TBlock = chain::Block;
type TChain = chain::Chain;
type CapsuledMicrochain = JsBox<RefCell<Microchain>>;

struct Microchain {
    chain: TChain,
    block: TBlock

}

impl Microchain {
    fn new(name: String) -> Microchain {
        Microchain {
            chain: TChain::new(name),
            block: TBlock::new(0, vec![])
        }
    }

    fn load_file(path: String) -> Microchain {
        let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
        let chain: TChain = serde_json::from_str(&contents).unwrap();
        let mut microchain = Microchain::new("".to_string());
        microchain.chain = chain;
        microchain
    }

    fn save_block(&mut self) {
        self.chain.add_block(self.block.clone());
        self.block = TBlock::new(0, vec![]);
    }

    fn add_data(&mut self, data: Vec<u8>) {
        self.block.add_data(data);
    }

    fn get_data(&self) -> Vec<Vec<u8>> {
        self.block.get_data()
    }

    fn add_string(&mut self, data: String) {
        self.block.add_data(data.into_bytes());
    }

    fn get_string(&self) -> Vec<String> {
        let mut result = Vec::new();
        for data in self.block.get_data() {
            result.push(String::from_utf8(data).unwrap());
        }
        result
        
    }


    fn get_name(&self) -> String {
        self.chain.get_name()
    }

    fn save_as_file(&self, file_name: String) {
        let chain_string = self.chain.to_string();
        let mut file = File::create(file_name).unwrap();
        file.write_all(chain_string.as_bytes()).unwrap();
    }

}

impl Finalize for Microchain {}

impl Microchain {

    pub fn js_new(mut cx: FunctionContext) -> JsResult<CapsuledMicrochain> {
        let name : String = cx.argument::<JsString>(0)?.value(&mut cx);
        let chain = Microchain::new(name);
        Ok(cx.boxed(RefCell::new(chain)))
    }

    pub fn js_save_block(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let microchain = cx.argument::<CapsuledMicrochain>(0)?;
        let mut chain = microchain.borrow_mut();
        chain.save_block();
        Ok(cx.undefined())
    }

    pub fn js_add_string(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let microchain = cx.argument::<CapsuledMicrochain>(0)?;
        let data = cx.argument::<JsString>(1)?.value(&mut cx);
        let mut chain = microchain.borrow_mut();
        chain.add_string(data);
        Ok(cx.undefined())
    }

    pub fn js_get_data(mut cx: FunctionContext) -> JsResult<JsArray> {
        let microchain = cx.argument::<CapsuledMicrochain>(0)?;
        let chain = microchain.borrow();
        let data = chain.get_data();
        let js_data = JsArray::new(&mut cx, data.len() as u32);
        for (i, data) in data.iter().enumerate() {
            let js_data_item = JsArray::new(&mut cx, data.len() as u32);
            for (j, data_item) in data.iter().enumerate() {
                let data_item_pack = JsNumber::new(&mut cx, *data_item as f64);
                js_data_item.set(&mut cx, j as u32, data_item_pack).unwrap();
            }
            js_data.set(&mut cx, i as u32, js_data_item).unwrap();
        }
        Ok(js_data)
        
    }

    pub fn js_get_string(mut cx: FunctionContext) -> JsResult<JsArray> {
        let microchain = cx.argument::<CapsuledMicrochain>(0)?;
        let chain = microchain.borrow();
        let data = chain.get_string();
        let js_data = JsArray::new(&mut cx, data.len() as u32);
        for (i, d) in data.iter().enumerate() {
            let clean_data = cx.string(d.to_string());
            js_data.set(&mut cx, i as u32, clean_data ).unwrap();
        }
        Ok(js_data)
    }

    pub fn js_get_name(mut cx: FunctionContext) -> JsResult<JsString> {
        let microchain = cx.argument::<CapsuledMicrochain>(0)?;
        let chain = microchain.borrow();
        let name = chain.get_name();
        Ok(cx.string(name))
    }

    pub fn js_save_as_file(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let microchain = cx.argument::<CapsuledMicrochain>(0)?;
        let file_name = cx.argument::<JsString>(1)?.value(&mut cx);
        let chain = microchain.borrow_mut();
        chain.save_as_file(file_name);
        Ok(cx.undefined())
    }

    pub fn js_load_file(mut cx: FunctionContext) -> JsResult<CapsuledMicrochain> {
        let file_name = cx.argument::<JsString>(0)?.value(&mut cx);
        let chain = Microchain::load_file(file_name);
        Ok(cx.boxed(RefCell::new(chain)))
    }

}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    //add js functions
    cx.export_function("new", Microchain::js_new)?;
    cx.export_function("saveBlock", Microchain::js_save_block)?;
    cx.export_function("addString", Microchain::js_add_string)?;
    cx.export_function("saveAsFile", Microchain::js_save_as_file)?;
    cx.export_function("getName", Microchain::js_get_name)?;
    cx.export_function("getData", Microchain::js_get_data)?;
    cx.export_function("getString", Microchain::js_get_string)?;
    cx.export_function("loadFile", Microchain::js_load_file)?;

    Ok(())
}
