use super::*;

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

pub fn join_right_associativity(expr: &Arc<Expression>) -> Option<Arc<Expression>> {
    let Expression::LogicalExpression(LogicalExpression::Join(top_join)) = expr.as_ref() else {
        return None;
    };

    let Expression::LogicalExpression(LogicalExpression::Join(right_join)) =
        top_join.right.as_ref()
    else {
        return None;
    };

    let new_right_join = Join {
        left: right_join.right.clone(),
        right: right_join.left.clone(),
        join_type: (),
    };

    let new_top_join = Join {
        left: top_join.left.clone(),
        right: Arc::new(Expression::LogicalExpression(LogicalExpression::Join(
            new_right_join,
        ))),
        join_type: (),
    };

    Some(Arc::new(Expression::LogicalExpression(
        LogicalExpression::Join(new_top_join),
    )))
}
