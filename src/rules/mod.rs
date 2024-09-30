use crate::Expression;
use std::sync::Arc;

pub mod transformation;
pub mod implementation;

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

pub type StaticRule = fn(&Arc<Expression>) -> Option<Arc<Expression>>;

impl Rule for StaticRule {}
