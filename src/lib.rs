use enum_dispatch::enum_dispatch;
use std::sync::Arc;

mod logical;
mod physical;
mod transformation_rules;

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

/// A representation of a Cascades rule.
///
/// A Cascades rule is defined as anything that is equivalent to a function that takes in an
/// [`Arc<Expression>`], pattern matches against it (either returning `true` or `false`), and if it
/// successfully matches, applies a transformation.
///
/// By combining the Cascades' `CheckPattern` and `Transform` functions, we do not have to traverse
/// the tree of relations more than once to transform the `Expression`.
///
/// TODO is it okay to make that optimization?
pub trait Rule: Fn(&Arc<Expression>) -> Option<Arc<Expression>> {}

impl Expression {
    /// This makes `transform` generic over all types that implement the `Rule` function signature.
    ///
    /// This can be useful for when a developer wants to dynamically introduce their own rules.
    ///
    /// TODO would this actually work?
    pub fn transform_generic<R: Rule>(self: &Arc<Expression>, rule: R) -> Option<Arc<Expression>> {
        rule(self)
    }

    /// Transforms the given input according to a rule, if possible.
    pub fn transform(
        self: &Arc<Expression>,
        rule: fn(&Arc<Expression>) -> Option<Arc<Expression>>,
    ) -> Option<Arc<Expression>> {
        rule(self)
    }
}

#[cfg(test)]
mod tests {
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

        let commute_join = join
            .transform(transformation_rules::join_commutativity)
            .expect("This join rule should pattern match correctly");

        let revert = commute_join
            .transform(transformation_rules::join_commutativity)
            .expect("This join rule should pattern match correctly");

        println!("Original:\n{:?}\n", join);
        println!("Commutativity Applied:\n{:?}\n", commute_join);
        println!("Back to Original:\n{:?}\n", revert);
    }
}
