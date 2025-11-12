Slide #1

Hello everyone. First of all, I really appreciate the opportunity to present remotely and big thanks to the organisers for putting the effort because I did not get a visa so I could not attend the workshop in person. My name is Aleksei and I am doing my PhD at the University of Birmingham. I will present a joint work with Dan Ghica and Chris Barret on something we called Equivalence Hypergraphs (or e-hypergraphs) which are a generalisation of e-graphs for monoidal theories. I want to highlight that the work is purely theoretical and hence we do not have any experiments, implementation or benchmarks. But I hope the work will be insightful for some of you and hopefully it can foster some discussion. I will also appreciate any feedback. We essentially built some theoretical constructions and what connections these constructions have with e-graphs and why would anyone need these constructions.

Slide #2

First I will recall that e-graphs are primarily used for so-called algebraic theories and explain what these are, then I will explain what monoidal theories are, why would anyone be interested in monoidal theories and therefore why would anyone need e-graphs for such theories. 

Then I will sketch why e-graphs when applied in their current formulation to monoidal theories struggle and how we can try to rectify this / why e-graphs face difficulties when applied to monoidal theories

Then I will very briefly and informally explain the underlying theory of e-graphs for monoidal theories that we have constructed

Then I will demonstrate two examples: one example is about showing that e-graphs arise when our generalisation is restricted to so-called Cartesian monoidal theories. Another example is illustration of how our theory can be useful in building e-graphs for languages with bindings.

Slide #3

Observe that e-graphs are traditionally used for algebraic theories. Theories are sets of terms with equations and signature is a set of generators that you use to build terms in a theory. A signature is algebraic if every generator has exactly one output, or I will also say co-arity to refer to the number of outputs. This 'restriction' comes unnoticed because we usually operate with abstract syntax trees.

So for example consider a theory of commutative monoids where I have explicitly annotated the arities and co-arities. We have a multiplication, a unit and possibly some other constants. Both multiplication and the unit have co-arity of 1 while their arities differ: the multiplication has arity 2 and the unit has artity 0. The equations are standard associativity, unitality and commutativity.

Say that there are more general theories that represent the interaction between processes, e.g. quantum processes.

However, there are theories where we do not limit ourselves with generators of co-aritiy 1. These are used to describe the behaviour and interaction between proccesses, e.g. quantum processes.

<!-- Slide #4

If we allow ourselves to vary the co-arities we get to so-called monoidal theories that are used e.g. to describe behaviour of processes, e.g. quantum processes. -->

Slide #5

So, in constrast consider a co-algebraic signature where we have different generators with different co-arities.
In partiular, we have identity generators with matching arities and co-arities, we have symmetries which swap their inputs to produce outputs and some other domain-specific generators, for example co-multiplication and co-unit.

Also consider a set of terms that are constructed out of the generators by using (;) and (\otimes) making sure the types match. 

An example of such a term can be seen below.

Slide #6

However, unlike algebraic theories where the equations were domain-specific, terms constructed in this signature are subject to some built-in equations.
-- talk about equations --
<!-- 
monoidal theories we have built-in equations that prescribe that id's are left and right identities for a composition, identity whose arity and co-arity are zeroes are parallel composition identities and some other equations that require the generators and compositions to be coherent. -->

A symmetric monoidal theory is a family of terms which are constructed out of a co-algebraic signature using parallel and sequential compositions modulo the built-in equations. If we omit the symmetry generators and the corresponding rules we end up with a theory that is just monoidal.

So by using the same equations as for the theory of commutative monoids for our new generators we get a theory of commutative comonoids.

Slide #6.5

Such SMTs represent a syntax and one provides a semantical meaning for this syntax by building a functor from an induced symmetric monoidal category to some symmetric monoidal category of interest, for example to a category of finite vector spaces over a field k.

Slide #7

These theories find their application in different areas. For example one can build quantum circuits using these theories (and derive particularly nice small circuits by using equations), or define operational semantics of digital circuits, encode lambda calculus. So all these areas would benefit from equality saturation because once we have equations we have a problem of their ordering, for example to find the smallest quantum circuits or digital circuits.

Slides #8

It is possible to use e-graphs to encode such theories and perform equality saturation, for example by using the encoding below, However, this approach does not scale very well. Consider an example of an e-graph for a term we had previously...

After we saturate the graph with only the laws of a symmetric monoidal category (i.e. without comonoid laws) we end up with something like this which I had to crop because it did not fit the slide.

So a reasonable question is can we avoid these equations? Spoiler: yes.

The answer is yes and this is where string diagrams come into play.

Slide #9

String diagrams are a two-dimentional syntax for SMTs which represent equivalence classes of terms modulo the built-in equations. String diagrams are constructed inductively out of wires and boxes: identity is encoded as a single wire, symmetry is as crossed wires, every generator becomes a box and we can compose string diagrams sequentially and in parallel. There was also an id_0 which is encoded as empty space, which is very convenient as you do not need to draw it. Such diagrams are traditionally drawn from top to bottom as opposed to an abstract syntax tree. 

Such diagrams absorb some of the equations within its notation and other equations are sort of absorbed because we treat string diagrams that are obtained from each other via topological deformations as the same: i.e. we can freely slide the boxes along the wires or cross wires twice to stay within the same diagram.

Slide #10

In particular note how bi-functoriality law gets absorbed
by the notation, some other equations can be also absorbed by using geometrical properties of string diagrams.

Slide #11

Another benefit of string diagrams is that particular equations represented much clearer and it is not immediately obvious what is the term-equivalent (or rather an equivalence class of terms) of a particular equation as in the case of Z-spider fusion rule of ZX-calculus.

Slide #11.1

Say that boxes become hyperedges and wires become vertices of hypergraphs.

Slide #11.2

Let's revisit the previous example and encode the generators of commutative comonoid as follows. The string diagrammatic version of the term on the left will look like this, that it is a parallel composition of two diagrams. And it is already saturated with respect to built-in equations. However it is not saturated with respect to e.g. equations of commutative comonoid and I will show later how to encode this.
Note that I put saturated in quotes because we can apply the equation to untangle the wires and move the dot above but this step dissapears if we use hypergraphs.

<!-- Slide #12

Notably, string diagrams can be represented as concrete combinatorial structures, as hypergraphs where the laws of the symmetric monoidal category gets absorbed by the notion of isomorphic hypergraphs. The application of string-diagrammatic equations is implemented as graph rewriting. -->

Slide #13

So again, the answer is yes and we can get rid of the laws of symmetric monoidal category or rather sweep them under the carpet if we use hypergraphs as a representation of terms.

The work next formalises e-graphs for symmetric monoidal theories using string diagrams as a representation for terms.

Slide #14

First, we introduce an auxilary operator to a signature that we denote with '+' that takes a formal join of n terms with the same types. We will call SMTs equipped with such '+' enriched if this '+' further satisfies specific equations.

These joins will denote the equivalence classes of terms and we will graphically denote such joins as 'hierarchical boxes'.

Slide #15

Much like equivalences in e-graphs should satisfy the congruence property, we have a notion of congruence for formal joins. These also include the requirements that '+' is a commutative associative and idempotent operator.

Slide #16

The equations look as follows. Intuitively, the top left equation means that if g_1 is equivalent to g_n then f composed with g_1 is equivalent to f composed with g_n. The other rules are similar.
The bottom rules specify the idempotence associativity and commutativity of '+' which equips a set with this '+' with a structure of a semilattice.

An SMT equipped with such a plus gives rise to so-called enriched PROP for which the models are monoidal categories enriched over a category of semilattices. So this '+' has a clear categorical semantics.

Slide #17

Then the non-destructive feature of rewriting (and equality saturation) can be expressed as follows. Suppose we have a rewrite rule l -> r. And a target string diagram we want to apply the rewrite to. By using idempotence we create a copy of 'l' and then by rewriting this copy we avoid losing the information about 'l'. If we were to apply the same rule again to the resulting string diagram we would end up with the same result: we would introduce a copy of 'l' and end up with two copies of 'r' which can be reduced to just 'r'.

Slide #18

Such extended string diagrams also have a combinatorial incarnation as an equivalence hypegraph which consists of vertices and edges and some edges may be hierarchical, that is, they may contain equivalent e-hypergraphs. The interfaces indicate which vertices are inputs and which are outputs. The e-hypergraph on the right represent a symmetry inside a hierarchical edge.

Slide #19

So a key difference with plain e-graphs is that e-classes in e-hypergraphs contain equivalent hypergraphs rather than subtrees.

Now equivalence classes contain equivalent hypergraphs rather than sub-trees in contrast with plain e-graphs.

The main technical result of the our work is that these e-hypergraphs are sound and complete for representing equivalence classes of terms in an enriched SMT

This allows us to reason about this structures using graph rewriting which is defined in terms of double pushout rewriting which is a convenient framework for reasoning about rewriting systems for graph-like objects.
In particular, it makes it easier to reason about confluence, decide if two rules can be applied in parallel etc.

Note that these e-hypergraphs absorb the laws of symmetric monoidal category, equal terms modulo these laws become isomorphic hypergraphs, but they do not absorb structural rules for '+'.

Slide #20

Now I want to show why we can say that this construction is a generalisation of plain (acyclic) e-graphs. E-graphs appear to be a special case of e-hypergraphs when the underlying theory is Cartesian. That is, the theory comes with a copy and delete generators that satisfy some equations: the commutative comonoid ones plus a naturality of copy and delete.

Such theories essentially contain only terms with co-arity of 1, the same that e-graphs support well.

Slide #21

In such a setting for a given e-graph we can construct an equivalent e-hypergraph by recursively translating e-classes.
Consider an example e-graph with two equivalent terms.

Every trivial e-class is represented as it is: note the explicit copy of node 'a'. However, non-trivial e-class is represented a bit differently: we introduce a hierarchical box with a dashed line between each component and with an input for each for each of the components. To make this well-typed we add a deletion generator to discard the redundant inputs. For example, if we were to extract e.g. the bitshift out of this box, we would replace the box with its right component and 'reconnect' the wires. Node 2 and one copy of 'a' would then be deleted with a residual of just a << 1.


A general conversion can be sketched below.

Slide #22

There is a more elaborate example that mimics the addition of a subterm into an e-graph: we add a bit shift and then merge e-classes --- this can be represented as a series of rewrites applied to a string diagrammatic counterpart.

First we deduplicate '2' to identify a redex.
Then we perform a non-destructive rewrite and modify the nested string diagrams to be able to factor out 'a', '2' and '1' out of the box. This transformation produces an equivalent string diagram because of the equations for deletion. Obviously we can always perform such a factorisation. Then we factor these things out and notice that the resulting string diagram is equivalent to the resulting e-graph.

Slide #23

---

Slide #24

Another example is how string diagrams facilitate the support for bindings. Compare two e-graphs representations of binding and lambda-abstractions: one using names and the other using indices. String diagrammatic equivalent is on the right. Variables become just wires and lambdas are represented as 'bubbles'. Note also how an unused argument is deleted.

So string diagrams are automatically quotiented by alpha-equivalence

Slide #25

Thus, by enriching a Cartesian Closed SMT we can build e-hypergraphs for lambda calculus.

Slide #26

Consider an example that shows e-graphs for lambda calculus with explicit substitution and its string diagrammatic counterpart. In the top figure a beta reduction is computed via a recursive explicit substitution of e-classes. In the bottom figure the bubble represents as before a lambda abstraction and the grey half circle represents an application. A beta reduction is then a single rewriting step that connects the argument to the dangling wire of the abstraction and discards the 'bubble'. In particular note how the substitution of 1 into 'y + y' was avoided as well as the creation of auxilary nodes. Also note that we do not need to recompute any indices during such a rewrite.

This is just a sketch of string diagrams (and hence e-hypergraphs) for enriched Cartesian Closed SMTs. There are enough technical details so that e-hypergraphs for such theories (closed ones) do not immediately follow from our construction so fully formalising this is future work.

Slides #27

We have presented a generalisation of e-graphs for
SMTs which could broaden the area of application of
e-graphs to
• Quantum processes
• Probabilistic programming
• Digital circuits

I've seen quite a few talks about e-graphs for digital circuits in the program so e-hypergraphs may be just another way of doing it.

As a bonus we have a framework for reasoning about
e-graphs in terms of graph rewriting, although we did not formulate any statements about the latter.

So the practical value of the work is something that should be judged/discussed by the community. We are open for collaboration :)