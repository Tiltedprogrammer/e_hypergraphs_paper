## sd-visualiser

This is a framework for translating multiple front-ends (like llvm, sd-lang etc.) into string diagrams for cartesian closed categories and then rendering these string diagrams nicely be again translating them into some term representation.

The design is abstracted over concrete representation of nodes and edges that form a hypergraph similar to how associated data families in Haskell.

A node is either an `Operation` or a `Thunk` and both of those have inputs and outputs in terms of `InputPort` and `Output` port with the idea being that output ports are eventually connected to input ports. The same applies to a hypergraph as well.

In a traditional e-graph we have `E-nodes` and `E-classes` with the former referencing the latter as children, or rather the `id` of the latter. That is, each `e-class` is defined by the following data

```rust

struct EClass<L> {
    id : Id, // id of the e-class, i.e. pub struct Id(u32);
    nodes : Vec<L> // e-nodes of the e-class
}

```

and the `e-graph` is then given by the following data

```rust

struct EGraph<L : Language> {
    unionfind: UnionFind, //union find over e-class id's, given two id's it can tell if they are from the same e-class
    /// Stores the original node represented by each non-canonical id
    nodes: Vec<L>,
    /// Stores each enode's `Id`, not the `Id` of the eclass.
    /// Enodes in the memo are canonicalized at each rebuild, but after rebuilding new
    /// unions can cause them to become out of date.
    memo: HashMap<L, Id>, // this is hashconsing
    classes: HashMap<Id, EClass<L, N::Data>>,
}

```

In particular, `hashconsing` is the key to efficiency and the provider of sharing, if e-node is already in the e-graph we return its `e-class`'s id.