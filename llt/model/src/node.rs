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
    ConstI64(i64), // 64-bit integer constant
    ConstBool(bool), // true || false

    Add { a: NodeId, b: NodeId }, // a + b
    Sub { a: NodeId, b: NodeId }, // a - b
    Mul { a: NodeId, b: NodeId }, // a * b
    Div { a: NodeId, b: NodeId }, // a / b
    Rem { a: NodeId, b: NodeId }, // a % b

    Load { ptr: NodeId }, // *ptr low level
    Store { ptr: NodeId, value: NodeId }, // *ptr = value

    AddressOf { base: NodeId }, // &base
    Deref { ptr: NodeId }, // *ptr

    Eq { a: NodeId, b: NodeId }, // a == b
    Ne { a: NodeId, b: NodeId }, // a != b
    Lt { a: NodeId, b: NodeId }, // a < b
    Gt { a: NodeId, b: NodeId }, // a > b

    Jump { target: BlockId }, // goto target

    Branch {
        cond: NodeId,
        then_bb: BlockId,
        else_bb: BlockId,
    }, // if (cond) goto then_bb else goto else_bb

    Return { value: Option<NodeId> }, // return value

    Call {
        func: NodeId,
        args: Vec<NodeId>,
    }, // func(args...)

    Phi {
        incoming: Vec<(BlockId, NodeId)>,
    }, // phi node for SSA form
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