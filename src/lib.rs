use neon::prelude::*;
mod Chain;
mod Block;

type tBlock = Block::Block;
type tChain = Chain::Chain;

struct Microchain {
    chain: Chain::Chain,
}




#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    
    Ok(())
}
