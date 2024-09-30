use crate::{Expression, PhysicalProperties, Relation};
use enum_dispatch::enum_dispatch;
use std::sync::Arc;

#[enum_dispatch(Relation)]
#[derive(Debug)]
pub enum LogicalExpression {
    Scan,
    Filter,
    Join,
}

#[derive(Debug)]
pub struct Scan {
    pub table_id: usize,
    pub filters: (),
}

impl Relation for Scan {
    fn children(&self) -> Vec<Arc<Expression>> {
        vec![]
    }

    fn physical_properties(&self) -> Vec<PhysicalProperties> {
        vec![]
    }
}

#[derive(Debug)]
pub struct Filter {
    pub filters: (),
    pub children: Arc<Expression>,
}

impl Relation for Filter {
    fn children(&self) -> Vec<Arc<Expression>> {
        vec![self.children.clone()]
    }

    fn physical_properties(&self) -> Vec<PhysicalProperties> {
        vec![]
    }
}

#[derive(Debug)]
pub struct Join {
    pub join_type: (),
    pub left: Arc<Expression>,
    pub right: Arc<Expression>,
}

impl Relation for Join {
    fn children(&self) -> Vec<Arc<Expression>> {
        vec![self.left.clone(), self.right.clone()]
    }

    fn physical_properties(&self) -> Vec<PhysicalProperties> {
        vec![]
    }
}
