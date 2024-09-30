use crate::{rules::Rule, Expression, Group, Memo};
use scc::Stack;
use std::sync::{atomic::Ordering, Arc, RwLock};

/// The different types of tasks in the Cascades framework.
pub enum Task {
    OptimizeGroup {
        expr: Arc<Group>,
        limit: usize,
    },
    ExploreGroup {
        expr: Arc<Group>,
        limit: usize,
    },
    ExploreExpression {
        expr: Arc<Expression>,
        limit: usize,
    },
    OptimizeExpression {
        expr: Arc<Expression>,
        limit: usize,
    },
    OptimizeInputs {
        expr: Arc<Expression>,
        limit: usize,
    },
    ApplyRule {
        expr: Arc<Expression>,
        limit: usize,
        rule: Arc<dyn Rule>,
        promise: usize,
    },
}

/// The "global" state we need to keep track of during search in the Cascades framework.
///
/// TODO:
/// Note that all of the fields need to be serializable if we want to implement leaving breadcrumbs.
pub struct SearchEngine {
    memo: Arc<RwLock<Memo>>,
    tasks: Stack<Task>,
}

impl SearchEngine {
    /// The top-level function that optimizes a query plan.
    ///
    /// TODO: Parallelism.
    pub fn optimize(&self, query: Arc<Group>) {
        self.tasks.push(Task::OptimizeGroup {
            expr: query,
            limit: usize::MAX,
        });

        while let Some(task) = self.tasks.pop() {
            match task.as_ref().as_ref() {
                Task::OptimizeGroup { expr, limit } => self.optimize_group(expr, *limit),
                Task::ExploreGroup { expr, limit } => self.explore_group(expr, *limit),
                Task::ExploreExpression { expr, limit } => self.explore_expression(expr, *limit),
                Task::OptimizeExpression { expr, limit } => self.optimize_expression(expr, *limit),
                Task::OptimizeInputs { expr, limit } => self.optimize_inputs(expr, *limit),
                Task::ApplyRule {
                    expr,
                    limit,
                    rule,
                    promise,
                } => self.apply_rule(expr, *limit, rule, *promise),
            }
        }

        todo!("Return the fully optimized plan")
    }

    /// Derives the best physical plan for a group / equivalence class and places it in the memo
    /// table.
    pub fn optimize_group(&self, group: &Arc<Group>, limit: usize) {
        if group.explored.load(Ordering::Acquire) {
            todo!("Begin optimizing the expressions inside this group");
        } else {
            todo!("Loop through all expressions in this group and call `optimize_expression`");
        }
    }

    /// Generates alternative equivalent logical expressions for the group.
    pub fn explore_group(&self, group: &Arc<Group>, limit: usize) {
        // TODO: Why do we store true here now and not later???
        group.explored.store(true, Ordering::Release);

        todo!("Loop through all expressions in this group and call `explore_expression`")
    }

    /// Generates alternative equivalent logical expressions for the expression, pushing `ApplyRule`
    /// tasks onto the stack.
    pub fn explore_expression(&self, expr: &Arc<Expression>, limit: usize) {
        todo!()
    }

    /// Derives the best physical plan for an expression and places it in the memo table.
    pub fn optimize_expression(&self, expr: &Arc<Expression>, limit: usize) {
        todo!()
    }

    /// Applies a rule to the given expression, updates the memo table, and adds new expressions to
    /// explore if new expressions are created.
    pub fn apply_rule(
        &self,
        expr: &Arc<Expression>,
        limit: usize,
        rule: &Arc<dyn Rule>,
        promise: usize,
    ) {
        todo!()
    }

    /// Iterates over the inputs / children of an expression and optimizes them.
    pub fn optimize_inputs(&self, expr: &Arc<Expression>, limit: usize) {
        todo!()
    }
}
