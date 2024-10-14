# The `optd` Query Optimizer

**10/13/2024**

The `optd` optimizer is based on the cost-based Cascades query optimization framework. Many of the terms used in this document will be based on terms and concepts defined in the Microsoft article _Extensible query optimizers in practice_ (specifically in Chapter 2).

The purpose of this document is to record the high-level goals and potential contributions of the `optd` query optimizer project. Most of the ideas detailed in this document were discussed during the first handful of `optd` meetings in Fall 2024.

Note that at the time of writing, `optd` already has a prototype implementation. However, this document purposefully does not reference the code of the prototype. We will make the assumption that nothing has been implemented, and this project will be started from scratch.

Not all of the goals we describe in this document may be possible. However, by recording all of our goals in one place, we hope to create a unified vision for the `optd` project that will allow us to pave a clearer path.

# Overarching Goals

The main goal of the `optd` project is to build a query optimizer that has the following properties, listed in no particular order of importance:

-   **Standalone**: The `optd` optimizer should be a standalone component. This is in contrast to other query optimizers that are built directly into end-to-end database management systems. The goal of building a standalone optimizer is to allow future DBMS developers to "plug-and-play" `optd` into their own systems, either as an embedded library or as a service.
-   **Extensible**: Developers that use the `optd` optimizer should be able to customize the behavior of `optd` by adding their own relational operators and rules (among other things). The exact behavior of the execution operators that `optd` is optimizing should be opaque to the internal optimization engine.
-   **Parallel**: In order to extract the most performance out of modern concurrent hardware (CPUs and storage devices), the query optimization algorithm of `optd` should support efficient parallel search and optimization.
-   **Persistent**: Queries often repeat, and different query plans often still have similar structure. By persisting state between the optimization of query plans, we can leave a trail of "breadcrumbs" for any of the decisions made for future queries to utilize.

We will explain in depth what each of these goals entail.

## Standalone

The `optd` optimizer should be its own modularized component that DBMS developers can easily "plug in" to their own system. Due to its standalone nature, `optd` should use a standard representation of query plans as the bridge between `optd` and the external execution engine.

`optd` will use **Substrait** as its query plan representation by default. This can potentially be extended to support other representations in the future, but we will use Substrait by default.

TODO justification on why we want to use Substrait

TODO more information on plans for integration with Substrait

Additionally, `optd` will use **DataFusion** as its default execution engine as well as use DataFusion's SQL parser and binder.

TODO more information on plans for integration with DataFusion

TODO plans for eventually removing the DataFusion dependency in the future

Finally, `optd` should ship as either a fully functioning standalone service that exists in its own separate process (or even its own compute node), or a fully embeddable library that lives in the same process as the execution engine. Additionally, we will want to invest in the ease of integration for third-party developers. Getting started with `optd` should be as simple as importing a single library and copy-and-pasting tutorial code from a documentation site.

## Extensible

Due to `optd`'s standalone nature, it must support the customization and extensibility of its rules and operators without requiring an external developer to fork the entire repository to make changes. A developer using `optd` should be able to easily add their own relations, execution engine operators, and SQL expressions.

Since most execution engines are going to share similar behavior and operators (every query execution engine needs a scan operator), we will provide these standard relations and corresponding rules into `optd` by default. `optd` should work out of the box without any setup or customization. We will then additionally support customization by allowing developers to add their own custom operators and rules that might be specific to their own execution engine.

The core search algorithm of query optimization itself should not need to know the exact behavior of the operators, relations, or rules it is using for optimization, regardless of if they are provided by `optd` (proprietary) or provided by a third-party developer. This means that the `optd` search engine must rely on dynamic dispatch for the manipulation of external third-party types.

However, this does not mean we must solely switch to an object-oriented model and give up on Rust's powerful type system (abstract data types via `enum`s). Internally, we can use Rust `enum`s (via the `enum_dispatch` crate) to ensure correctness over the proprietary operators and rules that we write ourselves. Then, we can add a `Box<dyn Trait>` variant that allows external developers to additionally add their own types. This would allow compile-time guarantees for the all of the proprietary rules and operators in the standalone crate, while also allowing for the flexibility of trait objects. Note that this might seem like redundant work, but it will allow for faster velocity of change for the core components of `optd` while ensuring correctness throughout development.

TODO Cost model extensibility?

Echoing the end of the [standalone](#standalone) section, since we expect developers to add their own custom operators and rules, high quality documentation, tutorials, and examples of how to use `optd` is an absolute requirement.

### Internal Extensibility

Ideally, all of the high-level goals that have been stated in the section should be true not just for the public layer, but also the private library implementation itself. The internal library should be modular, extensible, have excellent documentation, and have a robust testing framework. We expect many people will be working on the `optd` project over the years to come, not all at the same time. We want to reduce the activation energy needed to get started on a pull request from someone new to the codebase, and since this project is open-source, potentially even someone outside of the CMU DB group.

## Parallel

One of the main challenges in creating a parallel search algorithm is managing dependent state between tasks and preventing duplicate / unnecessary work across tasks.

It will often be the case that one worker will be optimizing an expression that requires the optimization of a sub-expression / child, and some other worker is in the process of optimizing said sub-expression. The worst way to handle this is to simply put the entire thread that the worker is running on to sleep, as this reduces the parallelism and efficiency of the system.

Another challenge is the fact that the memo table (used for dynamic programming in the Cascades query optimization framework) is a single point of global contention. Almost every operation and task in the Cascades framework must read _and_ write to the memo table in a thread-safe manner. This means that surrounding the memo table with a single global lock would eliminate almost all parallelism, and performance would be measured by the speed at which threads can acquire and release a lock.

A potential solution to both of these problems is to use an asynchronous and cooperative model of parallel execution. In an asynchronous runtime, all parallelism is handled in userspace, and tasks that "block" waiting for other tasks (e.g., on a mutex or on I/O) don't actually halt the threads they are running on. Instead, they yield to another task that is located on the same thread. In other words, using an asynchronous runtime like `tokio` could eliminate the need for a complicated task dependency graph in the task scheduler (such as the one used by Orca) as a task that is "waiting" is no longer the same as a task that is "blocked" or sleeping.

Contention is still a problem in an asynchronous setting, as there is only so much a runtime can do to alleviate all tasks attempting to access global data structures in a thread-safe manner with locks. Thus, the memo table itself needs to support parallel access and manipulation. At a minimum, the memo table should support finer granularity of locking in the memo table. In the future, it is likely possible to implement the entire memo table in a lock-free manner (copy-on-write, append-only, compare-and-swap semantics, etc.). Using an asynchronous runtime certainly helps, as the cost of "blocking" is not the same as when using traditional synchronous threads.

## Persistent

One of the main research contributions of `optd` will be leaving a record of all decisions made by the optimizer by persisting optimization state which would act as a trail of breadcrumbs.

As far as we are aware, all current query optimizers are completely volatile. In other words, once a query plan has been optimized, the system forgets all of the steps it took to reach the final fully-optimized query plan.

This persisted state theoretically should prevent duplicate optimization work between queries over time. An added benefit of this is that query optimization can be stopped and restarted at any time, and can be durable against crashes. This behavior can be the default, allowing for `optd` to behave as an embeddable library if needed.

TODO Use of DBMS backend
TODO SeaORM
TODO breadcrumbs
TODO event logs
TODO durability guarantees?

While `optd` should have the goal of remembering all decisions made by default, we should also be able to support `optd` as an embeddable library that does not persist any state. We can simply remove all stored data after every query and use an in-memory DBMS as a backend server.

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
