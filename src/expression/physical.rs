use crate::{Expression, PhysicalProperties, Relation};
use enum_dispatch::enum_dispatch;
use std::sync::Arc;

#[enum_dispatch(Relation)]
#[derive(Debug)]
pub enum PhysicalExpression {
    TableScan,
    IndexScan,
    HashJoin,
}

#[derive(Debug)]
pub struct TableScan {
    pub table_id: usize,
    pub filters: (),
}

impl Relation for TableScan {
    fn children(&self) -> Vec<Arc<Expression>> {
        vec![]
    }

    fn physical_properties(&self) -> Vec<PhysicalProperties> {
        vec![]
    }
}

#[derive(Debug)]
pub struct IndexScan {
    pub table: (),
    pub filters: (),
    pub index_id: (),
    pub index_type: (),
}

impl Relation for IndexScan {
    fn children(&self) -> Vec<Arc<Expression>> {
        vec![]
    }

    fn physical_properties(&self) -> Vec<PhysicalProperties> {
        vec![PhysicalProperties::Sorted(42)]
    }
}

#[derive(Debug)]
pub struct HashJoin {
    pub join_type: (),
    pub hash_table_size: usize,
    pub partitions: usize,
    pub left: Arc<Expression>,
    pub right: Arc<Expression>,
}

impl Relation for HashJoin {
    fn children(&self) -> Vec<Arc<Expression>> {
        vec![self.left.clone(), self.right.clone()]
    }

    fn physical_properties(&self) -> Vec<PhysicalProperties> {
        vec![]
    }
}
