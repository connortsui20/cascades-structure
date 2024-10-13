# `optd` Project Rewrite

This document details the design of the new `optd` optimizer.

# Cascades

The `optd` optimizer is based on the cost-based Cascades query optimization framework.

Many of the terms used in this document will be based on terms and concepts defined in the article
_Extensible query optimizers in practice_, specifically in Chapter 2.

# Overarching Goals

The main goal of the `optd` project is to build a query optimizer that has the following properties,
listed in no particular order of importance:

-   **Standalone**: The `optd` optimizer is a standalone component. This is in contrast to other query optimizers that are built directly into end-to-end database management systems. The goal of building a standalone optimizer is to allow DBMS developers to "plug-and-play" `optd` into their own systems.
-   **Extensible**: Developers that use the `optd` optimizer should be able to customize the behavior of `optd` by adding their own relational operators and rules (among other things). The internal optimization engine and search algorithm of `optd` should be agnostic to the exact behavior of the expressions it is optimizing.
-   **Parallel**: In order to extract the most performance out of modern concurrent hardware (CPUs and storage devices), the search algorithm of `optd` should support parallel query plan search. A secondary goal is to limit both global contention and any other forms of OS-provided blocking as much as possible.
-   **Persistent**: Queries often repeat, and different query plans often have similar structure. By persisting state between optimizations of query plans, we can leave "breadcrumbs" of any decisions made for future queries to utilize, which should prevent duplicate work over time. An added benefit of this is that query optimization can be stopped and restarted at any time, and can be durable against crashes.

## Standalone

TODO Substrait
TODO Datafusion?
TODO Cost Model?

## Extensible

3 types of expressions in the expression tree:
Logical Expression / Relational Node
Physical Expression / Physical Operator
Predicate / Scalar / SQL Expression

Note that each of these expressions can have children of any expression type.

A developer using `optd` should be able to easily add their own relations, nodes, and predicates. Internally, we can use Rust enums (via the `enum_dispatch` crate) to ensure a correct type system, and then add a `Box<dyn Trait>` variant that allows external developers to add their own types.Am
We have 2 types of groups / equivalence classes:
Cascades / Relational / Operator Group
Predicate Group

A Cascades Group is a set of all logically and physically equivalent expressions.

A Predicate Group is a set of all equivalent predicate / SQL expressions. It might seem like a waste to store multiple equivalent Predicate expressions, since we generally use heuristic constant folding in these types of expressions. We will explain later why we want to record these, but at a high level the expression (A && B) might be cheap at some point in time, but it can become expensive and (B && A) might be the better option.

## Parallel

One of the main challenges in creating a parallel search algorithm is managing dependent state between tasks and preventing duplicate / unnecessary work across tasks.

It will often be the case that one worker will be optimizing an expression that requires the optimization of a subexpression / child, and some other worker is in the process of optimizing said su expression. The worst way to handle this is to simply put the entire thread that the worker is running on to sleep, as this reduces the parallelism and efficiency of the system.

TODO Merging of groups

TODO Promise and Guidance???

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

### Former Readme

optd (pronounced as op-dee) is a database optimizer framework. It is a cost-based optimizer that searches the plan space using the rules that the user defines and derives the optimal plan based on the cost model and the physical properties.

The primary objective of optd is to explore the potential challenges involved in effectively implementing a cost-based optimizer for real-world production usage. optd implements the Columbia Cascades optimizer framework based on Yongwen Xu's master's thesis. Besides cascades, optd also provides a heuristics optimizer implementation for testing purpose.

The other key objective is to implement a flexible optimizer framework which supports adaptive query optimization (aka. reoptimization) and adaptive query execution. optd executes a query, captures runtime information, and utilizes this data to guide subsequent plan space searches and cost model estimations. This progressive optimization approach ensures that queries are continuously improved, and allows the optimizer to explore a large plan space.

Currently, optd is integrated into Apache Arrow Datafusion as a physical optimizer. It receives the logical plan from Datafusion, implements various physical optimizations (e.g., determining the join order), and subsequently converts it back into the Datafusion physical plan for execution.

optd is a research project and is still evolving. It should not be used in production. The code is licensed under MIT.
