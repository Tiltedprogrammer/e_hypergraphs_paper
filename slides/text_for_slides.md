Slide #1

Hello everyone. First of all, I really appreciate the opportunity to present remotely because I did not get a visa so I could not attend the workshop in person. My name is Aleksei and I am doing my PhD at the University of Birmingham. I will present a joint work with Dan Ghica and Chris Barret on something we called Equivalence Hypergraphs which are a generalisation of e-graphs for monoidal theories. I want to highlight that the work is purely theoretical and hence we do not have any experiments, implementation or benchmarks. But it is not that bad! I hope the work will be insightful for some of you and hopefully it can foster some discussion. I will also appreciate any feedback. We essentially built some theoretical constructions and I will show how these constructions apply to e-graphs and why would anyone need them.

Slide #2

First I will recall that e-graphs are primarily used for algebraic theories and explain what these are, then I will explain what monoidal theories are, why would anyone be interested in monoidal theories and therefore why would anyone need e-graphs for such theories. 

Then I will sketch why e-graphs when applied in their current formulation to monoidal theories struggle and how we can try to rectify this

Then I will very briefly and informally explain the underlying theory of e-graphs for monoidal theories that we have constructed

Then I will demonstrate two examples: one example is about showing that e-graphs arise when our generalisation is restricted to so-called Cartesian monoidal theories. Another example is illustration of how our theory can be useful in building e-graphs for languages with bindings.

Slide #3

Observe that e-graphs are traditionally used for algebraic theories. These are the theories where generators have exactly one output and a single sort of composition. So for example consider a theory of commutative monoids where you have a multiplication, a unit and possibly some other constants. Both multiplication and the unit have co-arity of 1 while their arities differ: the multiplication has arity 2 and the unit has artity 0. The equations are standard associativity, unitality and commutativity.

Slide #4

If we allow ourselves to vary the co-arities we get to so-called monoidal theories that are used e.g. to describe behaviour of processes, e.g. quantum processes.

Slide #5

So, in constrast consider a co-algebraic signature where we have different generators with different co-arities.
In partiular, we have identity generators with matching arities and co-arities, we have symmetries which swap their inputs to produce outputs and some other domain-specific generators, for example co-multiplication and co-unit.

Such theories consist of family of terms which are constructed from the generators by joining them sequentially and in parallel, that is we have two types of composition. Seqential composition composes terms with matching co-arities and arities while parallel composition composes arbitrary terms.

An example of such a term can be seen below.

Slide #6

However, unlike algebraic theories where the equations were domain-specific, in monoidal theories we have built-in equations that prescribe that id's are left and right identities for a composition, identity whose arity and co-arity are zeroes are parallel composition identities and some other equations that require the generators and compositions to be coherent.

A symmetric monoidal theory is a family of terms which are constructed out of a co-algebraic signature using parallel and sequential compositions modulo the built-in equations.

So by using the same equations as for the theory of commutative monoids for our new generators we get a theory of commutative comonoids.

Such SMTs represent a syntax and one provides a semantical meaning for this syntax by building a functor from an induced symmetric monoidal category to some symmetric monoidal category of interest, for example to a category of finite vector spaces over a field k.

Slide #7

These theories find their application in different areas. For example one can build quantum circuits using these theories, or define operational semantics of digital circuits, encode lambda calculus. So all these areas would benefit from equality saturation, for example to find the smallest quantum circuits or digital circuits.

Slides #8

It is possible to use e-graphs to encode such theories and perform equality saturation, for example by using the encoding below, i.e. by treating all the generators as constant and combine them by using the two compositions. However, the fact that we need to apply built-in equations lead to an explosion of the number of equivalence classes so it would be beneficial to somehow avoid these equations.

And a reasonable question is can we avoid them?
The answer is yes and this is where string diagrams come into play.

Slide #9

SMTs come with a two-dimensional syntax called string diagrams which represent equivalence
classes of terms quotiented by built-in equations which are a convenient tool for graphical
reasoning

Here is how the generators of a theory are represented using this graphical syntax which consitsts of wires and boxes. Identity is a single wire, symmetry is crossed wires, sequential composition connects the wires and parallel composition puts the boxes with their respective wires side-by-side. A box has as many input wires as its arity and as many output wires as its co-arity. Traditionally, the inputs flow from top to bottom, as opposed to an AST representation. Strings diagrams are invariant with respect to sliding of the boxes and with respect to double crossing of wires, i.e. double crossing is identity. That is, this two-dimentional syntax absorbs the equations of interest within its notation.

Slide #10

In particular note how bi-functoriality law gets absorbed
by the notation, some other equations can be also absorbed by using geometrical properties of string diagrams.

Slide #11

Another benefit of string diagrams is that particular equations represented much clearer and it is not immediately obvious what is the term-equivalent (or rather an equivalence class of terms) of a particular equation as in the case of Z-spider fusion rule of ZX-calculus.

Slide #12

Notably, string diagrams can be represented as concrete combinatorial structures, as hypergraphs where the laws of the symmetric monoidal category gets absorbed by the notion of isomorphic hypergraphs. The application of string-diagrammatic equations is implemented as graph rewriting.

Slide #13

So again, the answer is yes and we can get rid of the laws of summetric monoidal category or rather sweep them under the carpet if we use hypergraphs as a representation of terms.

The work next formalises e-graphs for symmetric monoidal theories using string diagrams as a representation for terms.

Slide #14

First, we introduce an auxilary operator to a signature that we denote with '+' that takes a formal join of n terms with the same types. We will call SMTs equipped with such '+' enriched if this '+' further satisfies specific equations.

These joins will denote the equivalence classes of terms and we will graphically denote such joins as 'hierarchical boxes'.

Slide #15

Much like equivalences in e-graphs should satisfy the congruence property, we have a notion of congruence for formal joins. These also include the requirements that '+' is a commutative associative and idempotent operator.

Slide #16

The equations look as follows. Intuitively, the top left equation means that if g_1 is equivalent to g_n then f composed with g_1 is equivalent to f composed with g_n. The other rules are similar.
The bottom rules specify the idempotence associativity and commutativity of '+'

Slide #17

Then the non-destructive feature of rewriting (and equality saturation) can be expressed as follows. Suppose we have a rewrite rule l -> r. And a target string diagram we want to apply the rewrite to. By using idempotence we create a copy of 'l' and then by rewriting this copy we avoid losing the information about 'l'. If we were to apply the same rule again to the resulting string diagram we would end up with the same result: we would introduce a copy of 'l' and end up with two copies of 'r' which can be reduced to just 'r'.

Slide #18

Such extended string diagrams also have a combinatorial incarnation as an equivalence hypegraph which consists of vertices and edges and some edges may be hierarchical, that is, they may contain equivalent e-hypergraphs. The interfaces indicate which vertices are inputs and which are outputs. The e-hypergraph on the right represent a symmetry inside a hierarchical edge.

Slide #19

Now equivalence classes contain equivalent hypergraphs rather than sub-trees in contrast with plain e-graphs.

The main technical result of the our work is that these e-hypergraphs are sound and complete for representing equivalence classes of terms in an enriched SMT

This allows us to reason about this structures using graph rewriting which is defined in terms of double pushout rewriting which is a convenient framework for reasoning about rewriting systems for graph-like objects.
In particular, it makes it easier to reason about confluence, decide if two rules can be applied in parallel etc.

Note that these e-hypergraphs absorb the laws of symmetric monoidal category, equal terms modulo these laws become isomorphic hypergraphs, but they do not absorb structural rules for '+'.

Slide #20

Now I want to show why we can say that this construction is a generalisation of plain e-graphs. E-graphs appear to be a special case of e-hypergraphs when the underlying theory is Cartesian. That is, the theory comes with a copy and delete generators that satisfy some equations: the commutative comonoid ones plus a naturality of copy and delete.

Such theories essentially contain only terms with co-arity of 1, the same that e-graphs support well.

Slide #21

In such a setting for a given e-graph we can construct an equivalent e-hypergraphs.
Consider an example e-graph with two equivalent terms. Every constant on the right becomes nested into a hierarchical box and node 'a' get shared explicitly by using the duplication generator. Nodes for '*' and '<<' get placed into a hierarchical box as well. Note the ports on the box and deletion generators. Every generator is connected to its respective port, for example if we supply 1 to the left component of the box, it get discrarded as well as one of the copies of 'a'. If we move the nodes inside the box by using the equations, we end up with a box that contains two equivalent terms.

A general conversion can be sketched below.

Slide #22

There is a more elaborate example that mimics the addition of a subterm into an e-graph: we add a bit shift and then merge e-classes --- this can be represented as a series of rewrites applied to a string diagrammatic counterpart.

First we deduplicate '2' to identify a regex.
Then we perform a non-destructive rewrite and modify the nested string diagrams to be able to factor out 'a', '2' and '1' out of the box. This transformation produces an equivalent string diagram because of the equations for deletion. Obviously we can always perform such a factorisation. Then we factor these things out and notice that the resulting string diagram is equivalent to the resulting e-graph.

Slide #23

---

Slide #24

Another example is how string diagrams facilitate the support for bindings. Compare two e-graphs representations of binding and lambdas: one using names and the other using indices. String diagrammatic equivalent is on the right. Variables become just wires and lambdas are represented as 'bubbles'. Note also how an unused argument is deleted.

Slide #25

Thus, by enriching a Cartesian Closed SMT we can build e-hypergraphs for lambda calculus.

Slide #26

Consider an example that shows e-graphs for lambda calculus with explicit substitution and its string diagrammatic counterpart. In the top figure a beta reduction is computed via a recursive explicit substitution of e-classes. In the bottom figure the bubble represents as before a lambda abstraction and the grey half circle represents an application. A beta reduction is then a single rewriting step that connects the argument to the dangling wire of the abstraction and discards the 'bubble'. In particular note how the substitution of 1 into 'y + y' was avoided as well as the creation of auxilary nodes.

This is just a sketch of string diagrams (and hence e-hypergraphs) for enriched Cartesian Closed SMTs. There are enough technical details so that e-hypergraphs for such theories (closed ones) do not immediately follow from our construction so fully formalising this is future work.

Slides #27

e have presented a generalisation of e-graphs for
SMTs which could broaden the area of application of
e-graphs to
• Quantum processes
• Probabilistic programming
• Digital circuits

I've seen quite a few talks about e-graphs for digital circuits in the program so e-hypergraphs may be just another way of doing it.

As a bonus we have a framework for reasoning about
e-graphs in terms of graph rewriting, although we did not formulate any statements about the latter.

So the practical value of the work is something that should be judged/discussed by the community. We are open for collaboration :)