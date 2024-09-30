use crate::rules::{transformation, StaticRule};
use crate::{Join, LogicalExpression, Scan};

use super::*;

#[test]
fn basic() {
    let table1 = Arc::new(Expression::LogicalExpression(LogicalExpression::Scan(
        Scan {
            table_id: 1,
            filters: (),
        },
    )));

    let table2 = Arc::new(Expression::LogicalExpression(LogicalExpression::Scan(
        Scan {
            table_id: 2,
            filters: (),
        },
    )));

    let join = Arc::new(Expression::LogicalExpression(LogicalExpression::Join(
        Join {
            left: table1,
            right: table2,
            join_type: (),
        },
    )));

    // Have to use the `as StaticRule` to coerce correctly.
    // See: https://github.com/rust-lang/rust/issues/62385
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
