# `optd` Project Rewrite

_This document details the design of the new `optd` optimizer._

The `optd` optimizer is based on the cost-based Cascades query optimization framework. Many of the terms used in this document will be based on terms and concepts defined in the article _Extensible query optimizers in practice_, specifically in Chapter 2.

# Overarching Goals

The main goal of the `optd` project is to build a query optimizer that has the following properties,
listed in no particular order of importance:

-   **Standalone**: The `optd` optimizer should be a standalone component. This is in contrast to other query optimizers that are built directly into end-to-end database management systems. The goal of building a standalone optimizer is to allow future DBMS developers to "plug-and-play" `optd` into their own systems.
-   **Extensible**: Developers that use the `optd` optimizer should be able to customize the behavior of `optd` by adding their own relational operators and rules (among other things). The internal optimization engine and search algorithm of `optd` should be agnostic to the behavior of the expressions it is optimizing.
-   **Parallel**: In order to extract the most performance out of modern concurrent hardware (CPUs and storage devices), the query optimization algorithm of `optd` should support parallel search. A secondary goal is to limit both the global contention on data structures as well as any forms of OS-provided blocking as much as possible.
-   **Persistent**: Queries often repeat, and different query plans often have similar structure. By persisting state between optimizations of query plans, we can leave "breadcrumbs" of any decisions made for future queries to utilize, which should prevent duplicate work over time. An added benefit of this is that query optimization can be stopped and restarted at any time, and can be durable against crashes.

## Standalone

The `optd` optimizer should be its own modularized component that DBMS developers can easily add to their own system. Due to its standalone nature, `optd` should use a standard representation of query plans as the bridge between `optd` and the external execution engine.

`optd` will use **Substrait** as its query plan representation by default. This can potentially be extended to support other representations in the future, but we will use Substrait by default.

TODO more information on plans for integration with Substrait

Additionally, `optd` will use **DataFusion** as its default execution engine as well as use DataFusion's SQL parser and binder.

TODO more information on plans for integration with DataFusion

## Extensible

Due to `optd`'s standalone nature, it must support the customization and extensibility of its rules and operators without requiring an external developer to fork the entire repository. A developer using `optd` should be able to easily add their own relations, nodes, and SQL expressions.

The search algorithm of optimization itself should not need to know the exact behavior of operators and relations it is optimizing. It should also be completely agnostic to transformation and implementation rules, regardless of if they are provided by us (first-party) or provided by a third-party developer. This means that the `optd` search engine must rely on dynamic dispatch and trait objects for the manipulation of external types.

However, this does not mean we must give up on Rust's powerful type system (abstract data types via `enum`s). Internally, we can use Rust `enum`s (via the `enum_dispatch` crate) to ensure correctness over the first-party core operators and rules that we write ourselves. Then, we can add a `Box<dyn Trait>` variant that allows external developers to additionally add their own types. This would allow compile-time guarantees for the all of the first-party rules and operators in the standalone crate, while also allowing for the flexibility of trait objects. Note that this might seem like redundant work, but it will allow for faster velocity of change for the core components of `optd` while ensuring correctness throughout development.

TODO Cost model extensibility?

## Parallel

One of the main challenges in creating a parallel search algorithm is managing dependent state between tasks and preventing duplicate / unnecessary work across tasks.

It will often be the case that one worker will be optimizing an expression that requires the optimization of a sub-expression / child, and some other worker is in the process of optimizing said sub-expression. The worst way to handle this is to simply put the entire thread that the worker is running on to sleep, as this reduces the parallelism and efficiency of the system.

Another challenge is the fact that the memo table (used for dynamic programming in the Cascades query optimization framework) is a single point of global contention. Almost every operation and task in the Cascades framework must read _and_ write to the memo table in a thread-safe manner. This means that surrounding the memo table with a single global lock would eliminate almost all parallelism, and performance would be measured by the speed at which threads can acquire and release a lock.

A potential solution to both of these problems is to use an asynchronous and cooperative model of parallel execution. In an asynchronous runtime, all parallelism is handled in userspace, and tasks that "block" waiting for other tasks (e.g., on a mutex or on I/O) don't actually halt the threads they are running on. Instead, they yield to another task that is located on the same thread. In other words, using an asynchronous runtime like `tokio` could eliminate the need for a complicated task dependency graph in the task scheduler (such as the one used by Orca) as a task that is "waiting" is no longer the same as a task that is "blocked" or sleeping.

Contention is still a problem in an asynchronous setting, as there is only so much a runtime can do to alleviate all tasks attempting to access global data structures in a thread-safe manner with locks. Thus, the memo table itself needs to support parallel access and manipulation. The first steps we should take are implementing finer granularity of locking in the memo table. In the future, it is likely possible to implement the entire memo table in a lock-free manner (copy-on-write, append-only, compare-and-swap semantics, etc.). Using an asynchronous runtime certainly helps, as "locking" is not the same as when using traditional synchronous threads.

## Persistent

TODO Use of DBMS backend
TODO SeaORM
TODO breadcrumbs
TODO event logs
TODO durability guarantees?

# Related Work

The original Cascades paper
Microsoft "Extensible query optimizers in practice" article
The Orca query optimizer
CockroachDB's query optimizer

# Design

3 types of expressions in the expression tree:
Logical Expression / Relational Node
Physical Expression / Physical Operator
Predicate / Scalar / SQL Expression

Note that each of these expressions can have children of any expression type.

We have 2 types of groups / equivalence classes:
Cascades / Relational / Operator Group
Predicate Group

A Cascades Group is a set of all logically and physically equivalent expressions.

A Predicate Group is a set of all equivalent predicate / SQL expressions. It might seem like a waste to store multiple equivalent Predicate expressions, since we generally use heuristic constant folding in these types of expressions. We will explain later why we want to record these, but at a high level the expression (A && B) might be cheap at some point in time, but it can become expensive and (B && A) might be the better option.

---

### Former Readme

optd (pronounced as op-dee) is a database optimizer framework. It is a cost-based optimizer that searches the plan space using the rules that the user defines and derives the optimal plan based on the cost model and the physical properties.

The primary objective of optd is to explore the potential challenges involved in effectively implementing a cost-based optimizer for real-world production usage. optd implements the Columbia Cascades optimizer framework based on Yongwen Xu's master's thesis. Besides cascades, optd also provides a heuristics optimizer implementation for testing purpose.

The other key objective is to implement a flexible optimizer framework which supports adaptive query optimization (aka. reoptimization) and adaptive query execution. optd executes a query, captures runtime information, and utilizes this data to guide subsequent plan space searches and cost model estimations. This progressive optimization approach ensures that queries are continuously improved, and allows the optimizer to explore a large plan space.

Currently, optd is integrated into Apache Arrow Datafusion as a physical optimizer. It receives the logical plan from Datafusion, implements various physical optimizations (e.g., determining the join order), and subsequently converts it back into the Datafusion physical plan for execution.

optd is a research project and is still evolving. It should not be used in production. The code is licensed under MIT.
