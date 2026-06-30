#[repr(C)]
pub struct FuncHandle {
    ptr: *mut Function,
}

#[repr(C)]
pub struct BlockHandle {
    ptr: *mut BasicBlock,
}

#[repr(C)]
pub struct InstHandle {
    ptr: *mut Instruction,
}

pub struct Function {
    pub name: String,
    pub params: Vec<usize>,         // индексы типов
    pub ret: Option<usize>,

    pub blocks: Vec<BasicBlock>,    // индекс в этом Vec == BlockId
    pub insts: Vec<Instruction>,    // индекс в этом Vec == InstId == ValueId

    pub entry: usize,               // индекс блока
}

pub struct BasicBlock {
    pub name: String,
    pub insts: Vec<usize>,          // индексы в Function.insts, в порядке выполнения
    pub preds: Vec<usize>,          // индексы блоков-предшественников
}

pub struct Instruction {
    pub opcode: Opcode,
    pub ty: Option<usize>,          // индекс типа результата, None если без результата
    pub operands: Vec<usize>,       // индексы значений-операндов (= индексы в Function.insts)
    pub block: usize,               // индекс блока, которому принадлежит инструкция
}

pub enum Opcode {
    // Арифметика
    Add, Sub, Mul, SDiv, UDiv,
    // Память
    Alloca, Load, Store,
    // Терминаторы
    Br { target: usize },
    CondBr { cond: usize, then_bb: usize, else_bb: usize },
    Ret { value: Option<usize> },
    // SSA
    Phi { incomings: Vec<(usize, usize)> }, // (block_idx, value_idx)
    Call { callee: usize, args: Vec<usize> }, // индекс функции в Module.functions
    Const(ConstValue),
}

pub enum ConstValue {
    I32(i32),
    I64(i64),
    F64(f64),
    Bool(bool),
}

#[derive(Clone, PartialEq)]
pub enum TypeKind {
    I1, I8, I16, I32, I64,
    F32, F64,
    Ptr(usize),                     // индекс типа, на который указываем
    Array(usize, u64),
    Struct(Vec<usize>),
    Void,
}