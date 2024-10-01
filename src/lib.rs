//! Some high-level thoughts:
//!
//! - Having very strict types may make things verbose, but it ensures that things are not incorrect
//!   and generally easier to work with.
//! - There are quite a few design decisions that should be made with respect to the specific data
//!   structures used (Memo table keys and values, granularity of locks, where to store guidance,
//!   where to store the different types of rules)
//! - I think it is possible for us to implement efficient parallel search _without_ a task
//!   dependency graph like the one described in Orca, **only if** we use an asynchronous runtime.
//!   The concern with not having a dependency graph is that tasks that get scheduled might get
//!   blocked on other tasks and spend most of their time waiting for the tasks they are dependent
//!   to finish. However, in an asynchronous environment, there is not blocking, and the runtime can
//!   figure out which task the current task is dependent on and go help it out.

use arc_swap::ArcSwapOption;
use dashmap::DashMap;
use enum_dispatch::enum_dispatch;
use rules::Rule;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize};
use std::sync::{Arc, RwLock};

mod engine;
mod expression;
mod rules;

use expression::logical::*;
use expression::physical::*;

/// A Cascades expression, which is either a logical expression or a physical expression.
///
/// We use the [`enum_dispatch`] macro to generate enum bindings for all possible variants. See the
/// documentation in the modules [`logical`] and [`physical`] for more information.
///
/// Note that the only reason why this is not using [`enum_dispatch`] is that it would make
/// constructing one of these way too verbose.
#[derive(Debug)]
pub enum Expression {
    Logical(LogicalExpression),
    Physical(PhysicalExpression),
}

impl Relation for Expression {
    fn children(&self) -> Vec<Arc<Expression>> {
        match self {
            Expression::Logical(logical_expression) => logical_expression.children(),
            Expression::Physical(physical_expression) => physical_expression.children(),
        }
    }

    fn physical_properties(&self) -> Vec<PhysicalProperties> {
        match self {
            Expression::Logical(logical_expression) => logical_expression.physical_properties(),
            Expression::Physical(physical_expression) => physical_expression.physical_properties(),
        }
    }
}

impl Expression {
    /// Checks if the pattern matches the given expression.
    pub fn check_pattern<R: Rule>(self: &Arc<Expression>, rule: R) -> bool {
        rule(self).is_some()
    }

    /// Given an expression, returns an iterator of the possible logical transformations this
    /// expression can take on, ordered by their promise values.
    ///
    /// TODO:
    /// Should we store the `Guidance` inside the `Expression` tree or in the memo table?
    pub fn transformation_moves(
        self: &Arc<Expression>,
        guidance: &Guidance,
    ) -> Vec<(Arc<dyn Rule>, usize)> {
        todo!("Return an iterator over the possible transformations")
    }

    /// Given an expression, returns an iterator of the possible physical and logical
    /// transformations this expression can take on, ordered by their promise values.
    ///
    /// TODO:
    /// Should we store the `Guidance` inside the `Expression` tree or in the memo table?
    pub fn all_moves(self: &Arc<Expression>, _guidance: &Guidance) -> Vec<(Arc<dyn Rule>, usize)> {
        todo!("Return an iterator over the possible transformations")
    }

    /// Returns the group / equivalence class of the current expression.
    pub fn group(self: &Arc<Expression>, memo: &Arc<Memo>) -> Arc<Group> {
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
#[derive(Default)]
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
    ///
    /// TODO:
    /// Might even want to put locking on each individual expression within this equivalence class.
    expressions: RwLock<Vec<Arc<Expression>>>,

    /// Since `Guidance` should be thread-safe, we don't need to protect it with a lock.
    guides: Vec<Guidance>,

    /// By storing this in an atomic `ArcSwapOption`, we can ensure atomic changes to both the
    /// expression and the cost associated with that expression.
    winner: ArcSwapOption<Winner>,

    /// A flag that represents if exploration of this group has finished.
    explored: AtomicBool,
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
