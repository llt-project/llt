use std::collections::HashMap;

use crate::node::IR;


pub struct Project {
    irs: Vec<IR>,
    obfuscation_name: HashMap<String, bool>,
}

impl Project {
    pub fn new() -> Self {
        Project { 
            irs: Vec::new(),
            obfuscation_name: HashMap::new() 
        }
    }
}