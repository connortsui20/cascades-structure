use crate::{Expression, Group, Memo};
use scc::Stack;
use std::sync::{Arc, RwLock};

pub enum Task {}

struct SearchEngine {
    memo: Arc<RwLock<Memo>>,
    tasks: Stack<Task>,
}

impl SearchEngine {
    pub fn optimize_group(&self, group: Arc<Group>, limit: usize) -> Arc<Expression> {
        todo!()
    }

    pub fn explore_group(&self, group: Arc<Group>, limit: usize) {
        todo!()
    }

    pub fn explore_expression(&self, expr: Arc<Expression>, limit: usize) {
        todo!()
    }

    pub fn optimize_expression(&self, expr: Arc<Expression>, limit: usize) {
        todo!()
    }
}
