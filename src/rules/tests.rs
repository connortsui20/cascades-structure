use crate::rules::{transformation, StaticRule};
use crate::{Join, LogicalExpression, Scan};

use super::*;

#[test]
fn basic_transformation() {
    let table1: Arc<Expression> = Arc::new(Expression::Logical(LogicalExpression::Scan(Scan {
        table_id: 1,
        filters: (),
    })));

    let table2 = Arc::new(Expression::Logical(LogicalExpression::Scan(Scan {
        table_id: 2,
        filters: (),
    })));

    let join = Arc::new(Expression::Logical(LogicalExpression::Join(Join {
        left: table1,
        right: table2,
        join_type: (),
    })));

    // Have to use the `as StaticRule` to coerce correctly.
    let rule = transformation::join_commutativity as StaticRule;

    let commute_join = join
        .transform(rule)
        .expect("This join rule should pattern match correctly");

    let revert = commute_join
        .transform(rule)
        .expect("This join rule should pattern match correctly");

    println!("Original:\n{:?}\n", join);
    println!("Commutativity Applied:\n{:?}\n", commute_join);
    println!("Back to Original:\n{:?}\n", revert);
}
