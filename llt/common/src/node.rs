pub type BlockId = u32;
pub type NodeId = u32;
pub type TypeId = u32;

pub struct IR {
    pub blocks: Vec<Block>,
    pub nodes: Vec<Node>,
    pub types: Vec<Type>,
    pub entry: BlockId,
}

pub struct Block {
    pub id: BlockId,
    pub nodes: Vec<NodeId>,

    pub preds: Vec<BlockId>,
    pub succs: Vec<BlockId>,
}

pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
    pub ty: Option<TypeId>,
}

pub enum NodeKind {
    ConstI64(i64),
    ConstBool(bool),

    Add { a: NodeId, b: NodeId },
    Sub { a: NodeId, b: NodeId },
    Mul { a: NodeId, b: NodeId },
    Div { a: NodeId, b: NodeId },

    Load { ptr: NodeId },
    Store { ptr: NodeId, value: NodeId },

    AddressOf { base: NodeId },
    Deref { ptr: NodeId },

    Eq { a: NodeId, b: NodeId },
    Ne { a: NodeId, b: NodeId },
    Lt { a: NodeId, b: NodeId },
    Gt { a: NodeId, b: NodeId },

    Jump { target: BlockId },

    Branch {
        cond: NodeId,
        then_bb: BlockId,
        else_bb: BlockId,
    },

    Return { value: Option<NodeId> },

    Call {
        func: NodeId,
        args: Vec<NodeId>,
    },

    Phi {
        incoming: Vec<(BlockId, NodeId)>,
    },
}

pub enum Type {
    Void,

    Bool,

    I8,
    I16,
    I32,
    I64,

    U8,
    U16,
    U32,
    U64,

    F32,
    F64,

    Ptr(TypeId),

    Array {
        elem: TypeId,
        len: u32,
    },

    Struct {
        fields: Vec<TypeId>,
    },

    Function {
        params: Vec<TypeId>,
        ret: TypeId,
    },
}