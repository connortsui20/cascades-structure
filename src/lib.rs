use enum_dispatch::enum_dispatch;
use rules::Rule;
use std::sync::Arc;

mod expression;
mod rules;

use expression::logical::*;
use expression::physical::*;

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

impl Expression {
    /// Transforms the given input according to a rule, if possible.
    pub fn transform<R: Rule>(self: &Arc<Expression>, rule: R) -> Option<Arc<Expression>> {
        rule(self)
    }
}

/// The trait defining shared behavior between all types of `Expression`s.
///
/// TODO do we need a data() method that returns an optional `Value` type here?
#[enum_dispatch]
pub trait Relation {
    fn children(&self) -> Vec<Arc<Expression>>;
}
