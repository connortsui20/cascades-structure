use super::StaticRule;
use crate::{Expression, HashJoin, LogicalExpression, PhysicalExpression, TableScan};
use std::sync::Arc;

/// Static implementation rules transforming logical expressions into both logical and physical
/// expressions.
///
/// TODO:
/// We may want to represent this differently to keep track of promise values.
/// Should this allow easy reordering of the rules?
static STATIC_IMPLEMENTATION_RULES: [StaticRule; 2] = [table_scan, hash_join];

/// An implementation rule that turns a logical scan into a table scan.
pub fn table_scan(expr: &Arc<Expression>) -> Option<Arc<Expression>> {
    let Expression::Logical(LogicalExpression::Scan(scan)) = expr.as_ref() else {
        return None;
    };

    Some(Arc::new(Expression::Physical(
        PhysicalExpression::TableScan(TableScan {
            table_id: scan.table_id,
            filters: scan.filters,
        }),
    )))
}

/// An implementation rule that turns a logical join into a hash join.
pub fn hash_join(expr: &Arc<Expression>) -> Option<Arc<Expression>> {
    let Expression::Logical(LogicalExpression::Join(join)) = expr.as_ref() else {
        return None;
    };

    Some(Arc::new(Expression::Physical(
        PhysicalExpression::HashJoin(HashJoin {
            join_type: (),
            hash_table_size: 42,
            partitions: 42,
            left: join.left.clone(),
            right: join.right.clone(),
        }),
    )))
}
