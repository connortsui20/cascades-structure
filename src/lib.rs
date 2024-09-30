use arc_swap::ArcSwapOption;
use dashmap::DashMap;
use enum_dispatch::enum_dispatch;
use rules::Rule;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::sync::RwLock;

mod engine;
mod expression;
mod rules;

use expression::logical::*;
use expression::physical::*;

/// A Cascades expression, which is either a logical expression or a physical expression.
///
/// We use the [`enum_dispatch`] macro to generate enum bindings for all possible variants. See the
/// documentation in the modules [`logical`] and [`physical`] for more information.
#[derive(Debug)]
pub enum Expression {
    Logical(LogicalExpression),
    Physical(PhysicalExpression),
}

impl Expression {
    /// Transforms the given input according to a rule, if possible.
    pub fn transform<R: Rule>(self: &Arc<Expression>, rule: R) -> Option<Arc<Expression>> {
        rule(self)
    }

    /// Given an expression, returns an iterator of the possible transformations this expression can
    /// take on, ordered by their promise values.
    ///
    /// TODO:
    /// Should we store the `Guidance` inside the `Expression` tree or in the memo table?
    pub fn moves<I>(self: &Arc<Expression>, _guidance: &Guidance) -> I
    where
        I: Iterator<Item = Arc<Expression>>,
    {
        todo!("Return an iterator over the possible transformations")
    }

    /// Returns the group / equivalence class of the current expression.
    pub fn group(self: Arc<Expression>, memo: Arc<Memo>) -> Arc<Group> {
        todo!("Figure out the `GroupKey` of the current `Expression` and find the group in memo")
    }
}

/// The trait defining shared behavior between all types of `Expression`s.
///
/// TODO do we need a data() method that returns an optional `Value` type here?
#[enum_dispatch]
pub trait Relation {
    fn children(&self) -> Vec<Arc<Expression>>;

    fn physical_properties(&self) -> Vec<PhysicalProperties>;
}

/// The different types of physical properties.
pub enum PhysicalProperties {
    Sorted(usize),
    Partitioned(usize),
    Exchanged(usize),
    RowStored,
    ColumnStored,
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

// The winning / best plan for a given group / equivalence class.
pub struct Winner {
    expression: Arc<Expression>,
    cost: usize,
}

/// The representation of an equivalence class in the Cascades framework.
///
/// TODO:
/// - Assuming we have a way to quickly look up a group and get access to the list of equivalent
///   expressions as well as the current winner, should we store guidance and promise inside the
///   expressions themselves (literally in the [`Expression`] tree) or can we store them right next
///   to each other in the memo table?
pub struct Group {
    /// The equivalent expressions that belong to this group / equivalence class.
    expressions: Arc<RwLock<Vec<Arc<Expression>>>>,

    /// An alternative if we want to store the `Guidance` objects right next to where the
    /// expressions are getting stored.
    /// Might even want to put locking on each individual expression within this equivalence class.
    expressions_alternative: Arc<RwLock<Vec<(Guidance, Arc<Expression>)>>>,

    /// By storing this in an atomic `ArcSwapOption`, we can ensure atomic changes to both the
    /// expression and the cost associated with that expression.
    winner: ArcSwapOption<Winner>,
}

/// The lookup key for a `Group`.
///
/// TODO:
/// - How do to store and lookup groups efficiently? By ID or hashing? Or some other type of representation?
pub struct GroupKey {
    // is this the right representation?
    id: usize,
}

/// The memoization table used for dynamic programming in the Cascades framework.
///
/// TODO:
/// - The memo table _needs_ to allow parallel access and mutation, which means it will need a very
///   fine level of granular locking. What probably makes the most sense is storing 1 rwlock for
///   every group / equivalence class, and having guidance be implemented via atomic types is (using
///   a lot of compare-and-swaps + fetch_update) is likely sufficient.
pub struct Memo {
    /// A concurrent hash table mapping [`GroupKey`]s to [`Group`]s.
    table: DashMap<GroupKey, Arc<Group>>,
}
