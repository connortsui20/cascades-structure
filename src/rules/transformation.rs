use super::StaticRule;
use crate::{Expression, Join, LogicalExpression};
use std::sync::Arc;

/// Static transformation rules transforming logical expressions into equivalent but different
/// logical expressions.
///
/// TODO:
/// We may want to represent this differently to keep track of promise values.
/// Should this allow easy reordering of the rules?
static STATIC_TRANSFORMATION_RULES: [StaticRule; 2] =
    [join_commutativity, join_right_associativity];

/// A rule that defines join commutativity.
///
/// `Join(A, B)` is logically equivalent to `Join(B, A)`.
pub fn join_commutativity(expr: &Arc<Expression>) -> Option<Arc<Expression>> {
    let Expression::LogicalExpression(LogicalExpression::Join(join)) = expr.as_ref() else {
        return None;
    };

    let new_join = Join {
        left: join.right.clone(),
        right: join.left.clone(),
        join_type: (),
    };

    Some(Arc::new(Expression::LogicalExpression(
        LogicalExpression::Join(new_join),
    )))
}

/// A rule that defines join right associativity.
///
/// `Join(Join(A, B), C)` is logically equivalent to `Join(A, Join(B, C))`.
pub fn join_right_associativity(expr: &Arc<Expression>) -> Option<Arc<Expression>> {
    let Expression::LogicalExpression(LogicalExpression::Join(top_join)) = expr.as_ref() else {
        return None;
    };

    let Expression::LogicalExpression(LogicalExpression::Join(left_join)) = top_join.left.as_ref()
    else {
        return None;
    };

    let new_right_join = Join {
        left: left_join.right.clone(),
        right: top_join.right.clone(),
        join_type: (),
    };

    let new_top_join = Join {
        left: left_join.left.clone(),
        right: Arc::new(Expression::LogicalExpression(LogicalExpression::Join(
            new_right_join,
        ))),
        join_type: (),
    };

    Some(Arc::new(Expression::LogicalExpression(
        LogicalExpression::Join(new_top_join),
    )))
}
