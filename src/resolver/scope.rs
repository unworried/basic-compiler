use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::cache::DefinitionId;

pub type VarCell = Rc<RefCell<HashMap<String, DefinitionId>>>;

#[derive(Debug)]
pub struct Scope {
    pub variables: VarCell,
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn check_variable(&self, name: &str) -> bool {
        self.variables.borrow().get(name).is_some()
    }

    pub fn declare_variable(&mut self, name: String) -> DefinitionId {
        let mut variable_store = self.variables.borrow_mut();
        let len = variable_store.len();
        variable_store.insert(name, DefinitionId(len));
        DefinitionId(len)
    }
}
