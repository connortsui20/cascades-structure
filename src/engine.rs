use crate::{Expression, Group, Memo};
use std::sync::{Arc, RwLock};

struct SearchEngine {
    memo: Arc<RwLock<Memo>>,
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
