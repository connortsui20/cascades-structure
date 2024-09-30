use enum_dispatch::enum_dispatch;
use rules::Rule;
use std::sync::Arc;

mod logical;
mod physical;
mod rules;

use logical::*;
use physical::*;

/// A Cascades expression, which is either a logical expression or a physical expression.
///
/// We use the [`enum_dispatch`] macro to generate enum bindings for all possible variants. See the
/// documentation in the modules [`logical`] and [`physical`] for more information.
#[enum_dispatch(Relation)]
#[derive(Debug)]
pub enum Expression {
    LogicalExpression,
    PhysicalExpression,
}

/// The trait defining shared behavior between all types of `Expression`s.
///
/// TODO why do we need a data that returns an optional `Value` type here?
#[enum_dispatch]
pub trait Relation {
    fn children(&self) -> Vec<Arc<Expression>>;
}

impl Expression {
    /// Transforms the given input according to a rule, if possible.
    pub fn transform<R: Rule>(self: &Arc<Expression>, rule: R) -> Option<Arc<Expression>> {
        rule(self)
    }
}

#[cfg(test)]
mod tests {
    use rules::{transformation, StaticRule};

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
}
