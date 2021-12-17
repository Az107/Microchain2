use neon::prelude::*;
use std::io::Write;
use std::fs::File;
use std::cell::RefCell;


mod Chain;


type tBlock = Chain::Block;
type tChain = Chain::Chain;
type capsuledMicrochain = JsBox<RefCell<Microchain>>;

struct Microchain {
    chain: tChain,
    block: tBlock

}

impl Microchain {
    fn new(name: String) -> Microchain {
        Microchain {
            chain: tChain::new(name),
            block: tBlock::new(0, vec![])
        }
    }

    fn saveBlock(&mut self) {
        self.chain.addBlock(self.block.clone());
        self.block = tBlock::new(0, vec![]);
    }

    fn addData(&mut self, data: Vec<u8>) {
        self.block.addData(data);
    }

    fn addString(&mut self, data: String) {
        self.block.addData(data.into_bytes());
    }


    fn getName(&self) -> String {
        self.chain.getName()
    }

    fn saveAsFile(&self, file_name: String) {
        let chain_string = self.chain.to_string();
        let mut file = File::create(file_name).unwrap();
        file.write_all(chain_string.as_bytes()).unwrap();
    }

}

impl Finalize for Microchain {}

impl Microchain {

    pub fn js_new(mut cx: FunctionContext) -> JsResult<capsuledMicrochain> {
        let name : String = cx.argument::<JsString>(0)?.value(&mut cx);
        let chain = Microchain::new(name);
        Ok(cx.boxed(RefCell::new(chain)))
    }

    pub fn js_saveBlock(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let microchain = cx.argument::<capsuledMicrochain>(0)?;
        let mut chain = microchain.borrow_mut();
        chain.saveBlock();
        Ok(cx.undefined())
    }

    pub fn js_addString(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let microchain = cx.argument::<capsuledMicrochain>(0)?;
        let data = cx.argument::<JsString>(1)?.value(&mut cx);
        let mut chain = microchain.borrow_mut();
        chain.addString(data);
        Ok(cx.undefined())
    }

    pub fn js_saveAsFile(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let microchain = cx.argument::<capsuledMicrochain>(0)?;
        let file_name = cx.argument::<JsString>(1)?.value(&mut cx);
        let mut chain = microchain.borrow_mut();
        chain.saveAsFile(file_name);
        Ok(cx.undefined())
    }

}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    //add js functions
    cx.export_function("new", Microchain::js_new)?;
    cx.export_function("saveBlock", Microchain::js_saveBlock)?;
    cx.export_function("addString", Microchain::js_addString)?;
    cx.export_function("saveAsFile", Microchain::js_saveAsFile)?;

    Ok(())
}
