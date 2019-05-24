use boots::function::{Block, BlockData, Value, ValueData};
use boots::inst::{Inst, InstData};
use boots::utils::VecMap;

pub struct DataFlowGraph {
    insts: VecMap<Inst, InstData>,
    blocks: VecMap<Block, BlockData>,
    values: VecMap<Value, ValueData>,
}

impl DataFlowGraph {
    pub fn new() -> DataFlowGraph {
        DataFlowGraph {
            insts: VecMap::new(),
            blocks: VecMap::new(),
            values: VecMap::new(),
        }
    }

    pub fn new_block(&mut self) -> Block {
        self.blocks.push(BlockData::new())
    }

    pub fn new_inst(&mut self, inst_data: InstData) -> Inst {
        self.insts.push(inst_data)
    }
}
