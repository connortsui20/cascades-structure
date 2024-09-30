use enum_dispatch::enum_dispatch;
use rules::Rule;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::AtomicUsize;
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

/// A `Guidance` object that tracks the possible transformations that can be applied to an
/// `Expression` tree.
///
/// TODO:
/// This `Guidance` type will have to support concurrent access and modification so that there is
/// only one worker applying a transformation at a time.
pub struct Guidance {
    // TODO create an atomic bitmap instead.
    pub bitmap: Arc<[AtomicU8]>,
    pub cost_limit: AtomicUsize,
}

impl Expression {
    /// Transforms the given input according to a rule, if possible.
    pub fn transform<R: Rule>(self: &Arc<Expression>, rule: R) -> Option<Arc<Expression>> {
        rule(self)
    }

    /// Given an expression, returns an iterator over the possible and available transformations.
    ///
    /// TODO:
    /// Should we store the `Guidance` inside the `Expression` tree or separately?
    pub fn moves<I>(self: &Arc<Expression>, _guidance: &Guidance) -> I
    where
        I: Iterator<Item = Arc<Expression>>,
    {
        todo!("Return an iterator over the possible transformations")
    }
}

/// The trait defining shared behavior between all types of `Expression`s.
///
/// TODO do we need a data() method that returns an optional `Value` type here?
#[enum_dispatch]
pub trait Relation {
    fn children(&self) -> Vec<Arc<Expression>>;
}
