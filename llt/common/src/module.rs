use crate::func::{Function, ConstValue, TypeKind};

#[repr(C)]
pub struct ModuleHandle {
    ptr: *mut Module,
}

#[repr(C)]
pub struct GlobalVarHandle {
    ptr: *mut GlobalVar,
}


pub struct Module {
    pub name: String,
    pub functions: Vec<Function>,
    pub globals: Vec<GlobalVar>,
    pub types: Vec<TypeKind>,
}

pub struct GlobalVar {
    pub name: String,
    pub ty: usize, // индекс в Module.types
    pub initializer: Option<ConstValue>,
    pub is_constant: bool,
}