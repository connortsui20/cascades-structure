use crate::{Expression, HashJoin, LogicalExpression, PhysicalExpression, TableScan};
use std::sync::Arc;

/// An implementation rule that turns a logical scan into a table scan.
pub fn table_scan(expr: &Arc<Expression>) -> Option<Arc<Expression>> {
    let Expression::LogicalExpression(LogicalExpression::Scan(scan)) = expr.as_ref() else {
        return None;
    };

    Some(Arc::new(Expression::PhysicalExpression(
        PhysicalExpression::TableScan(TableScan {
            table_id: scan.table_id,
            filters: scan.filters,
        }),
    )))
}

/// An implementation rule that turns a logical join into a hash join.
pub fn hash_join(expr: &Arc<Expression>) -> Option<Arc<Expression>> {
    let Expression::LogicalExpression(LogicalExpression::Join(join)) = expr.as_ref() else {
        return None;
    };

    Some(Arc::new(Expression::PhysicalExpression(
        PhysicalExpression::HashJoin(HashJoin {
            join_type: (),
            hash_table_size: 42,
            partitions: 42,
            left: join.left.clone(),
            right: join.right.clone(),
        }),
    )))
}
