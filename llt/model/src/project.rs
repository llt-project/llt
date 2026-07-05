use crate::node::IR;

pub struct Project {
    irs: Vec<IR>,
}

impl Project {
    pub fn new() -> Self {
        Project { irs: Vec::new() }
    }
}