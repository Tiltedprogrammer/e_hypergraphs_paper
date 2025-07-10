Slide 4: Say that e-nodes are functional symbols with e-classes as children

Slide 16:
The original work on string diagram rewriting interprets morphisms of a syntactic category S(\Sigma) as cospans in the category of hypergraphs where the feet are ordered discrete hypergraphs and the carrier is an arbitrary hypergraph: a hypergraph is a graph where edges are allowed to connect more than 2 vertices. On the left you can see an example of such a cospan that encodes a morphism at the bottom (symmetry composed with the tensor of e_1 and e_2), note how the mapping of the interfaces distinguishes symmetry from identity (at the bottom). Also note that we get the same cospan for e_2 tensor e_1 composed with symmetry, that is such representation absorbs the SMC laws through a notion of cospan isomorphism which makes this representation very appealing for syntactic reasoning as it allows performing rewriting modulo SMC laws for free. The cospans form morphisms of a corresponding category of hypergraphs with interfaces where the composition is defined by pushout along the common interface.

We generalise the notion of hypergraph to an e-hypergraph where the latter has additionally two relations that make it possible to define dashed edges. The first relation induces an ordering on edges and vertices while the second relation distinguishes the equivalent components from each other. For example, we have that e_1 is a predecessor of node v_3 and v_3 is related to v_4 according to the smile relation.

Since boxes can contain sub-morphisms we need additional interfaces to encode them. From the internal interfaces you can see that the left component of the box represents a symmetry morphism. We distinguish between internal and external interfaces as composition does not care about nested morphisms. The adjusted notion of isomorphism of such extended cospans does not absorb all the equations brought by free enrichment, but absorbs commutativity and associativity of '+'.

Whereas all pushouts exist in the category on the left, only some of them exist in the category on the right.
However, the ones that do exist are sufficient for our purposes. We can note that interpretation is not a functor yet as the thing on the right is not a semilattice-enriched category yet, we will fix it shortly.

Slide 17:
Rewriting of morphisms is then formalised by double pushout rewriting where a rewrite rule is given by a span L <-> R and the target graph is given by G. Intuitively it removes the occurence of L within G and then fills the hole with R.

On the right is a modified DPO square that accounts for extended interfaces. It also contains the additional morphisms to define the embedding of internal and external interfaces into the resulting e-hypergraph H.

Slide 20:
The key result of the original work on string diagram rewriting is that categories S(\Sigma) and HypI(\Sigma) are equivalent in a sense that there is a full and faithful functor between them.

To get a similar result for S^(+) we first turn EHypI into a semilattice enriched category by defining a plus of two cospans and introduce structural DPO rewrites to make this plus obey the rules of a semilattice enriched category. By quotienting the morphisms, we get the following result. The equivalence on the right is essentially soundness and completeness of rewriting: if one morphism can be rewritten into another in S(\Sigma), we can also rewrite its image under the interpretation functor.
While the representation does not absorb these structural rules, we believe that rewriting modulo these rules is not hard (conceptually, no complexity yet).

Finally, we are ready to interpret. (use idempotence)

So we can revisit the example from the beginning.

The intuition of rewriting is preserved in our extended notion with the addition of an extra step to construct the new interfaces for the resulting e-hypergraph. 
