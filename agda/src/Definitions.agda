module Definitions where

open import Data.List using (List; []; _∷_)
import Data.List.NonEmpty as List⁺
open List⁺ using (List⁺) renaming (_∷_ to mkList⁺)
open import Data.Nat
open import Relation.Binary.PropositionalEquality
------------------------------------------------------------------------
-- Binary multigraphs

record Multigraph : Set₁ where
  field
    Obj   : Set
    Edge₂ : Obj → Obj → Obj → Set

data Edge₁ : ℕ → ℕ → ℕ → Set where
    _+'_ : ∀ {m n} → Edge₁ m n (m + n)
    _*'_ : ∀ {m n} → Edge₁ m n (m * n)

G₁ : Multigraph
G₁ = record { Obj   = ℕ ; Edge₂ = Edge₁}

-- Alternative: single object N, +/* are generators N,N → N
data Obj₁ : Set where
  N : Obj₁

data Edge₁' : Obj₁ → Obj₁ → Obj₁ → Set where
  plus  : Edge₁' N N N
  times : Edge₁' N N N

G₁' : Multigraph
G₁' = record { Obj = Obj₁ ; Edge₂ = Edge₁' }
------------------------------------------------------------------------
-- The common-context (cartesian) syntax

module Syntax (G : Multigraph) where
  open Multigraph G

  infixr 25 _⊗_

  data Type : Set where
    base : Obj → Type
    𝟙    : Type
    _⊗_  : Type → Type → Type

  Context : Set
  Context = List Type

  ----------------------------------------------------------------------
  -- Typed De Bruijn variables

  infix 4 _∈_

  data _∈_ (A : Type) : Context → Set where
    here  : ∀ {Γ} → A ∈ (A ∷ Γ)
    there : ∀ {B Γ} → A ∈ Γ → A ∈ (B ∷ Γ)

  ----------------------------------------------------------------------
  -- Join-free 'raw' terms
  --
  -- In match⊗ s t, the body t lives in B ∷ A ∷ Γ.  Thus De Bruijn
  -- index zero denotes the second displayed binder and index one the
  -- first: this is the nameless version of match(s , xy.t).

  data JFTerm (Γ : Context) : Type → Set where
    var     : ∀ {A} → A ∈ Γ → JFTerm Γ A
    edge₂   : ∀ {A B C}
            → Edge₂ A B C
            → JFTerm Γ (base A)
            → JFTerm Γ (base B)
            → JFTerm Γ (base C)
    unit    : JFTerm Γ 𝟙
    pair    : ∀ {A B}
            → JFTerm Γ A
            → JFTerm Γ B
            → JFTerm Γ (A ⊗ B)
    match⊗  : ∀ {A B C}
            → JFTerm Γ (A ⊗ B)
            → JFTerm (B ∷ A ∷ Γ) C
            → JFTerm Γ C
    match𝟙  : ∀ {A}
            → JFTerm Γ 𝟙
            → JFTerm Γ A
            → JFTerm Γ A

  ----------------------------------------------------------------------
  -- General terms
  --
  -- A term is already in distributive form: an ordered, non-empty
  -- list of join-free summands.  At this stage order and multiplicity
  -- are observable.  Associativity is represented by list
  -- concatenation; commutativity and idempotence will belong to the
  -- later term equality, not to this datatype.

  Terms : Context → Type → Set
  Terms Γ A = List⁺ (JFTerm Γ A)

  singleton : ∀ {Γ A} → JFTerm Γ A → Terms Γ A
  singleton = List⁺.[_]

  infixr 5 _⊞_

  _⊞_ : ∀ {Γ A} → Terms Γ A → Terms Γ A → Terms Γ A
  _⊞_ = List⁺._⁺++⁺_

  ----------------------------------------------------------------------
  -- Canonical distributive extensions of binary constructors
  --
  -- productWith uses row-major order: for each entry of the first
  -- list, traverse the whole second list.  This choice matters.  With
  -- ordered lists, distributing a binary rule locally in its first
  -- premise and then its second can produce a different order from
  -- distributing in the opposite order.  The later commuting
  -- conversion should therefore expand a whole rule instance into
  -- this canonical order.

--   productWith :
--       ∀ {a b c} {A : Set a} {B : Set b} {C : Set c}
--       → (A → B → C)
--       → List⁺ A
--       → List⁺ B
--       → List⁺ C
--   productWith f xs ys =
--     List⁺.concatMap (λ x → List⁺.map (f x) ys) xs
  productWith :
    ∀ {a b c} {A : Set a} {B : Set b} {C : Set c}
    → (A → B → C)
    → List⁺ A
    → List⁺ B
    → List⁺ C

  productWith {A = A} {C = C} f (mkList⁺ x xs) ys =
    go x xs
    where
      go : A → List A → List⁺ C
      go x [] =
        List⁺.map (f x) ys

      go x (x′ ∷ xs) =
        List⁺._⁺++⁺_
          (List⁺.map (f x) ys)
          (go x′ xs)

  edge₂ᵀ :
      ∀ {Γ A B C}
      → Edge₂ A B C
      → Terms Γ (base A)
      → Terms Γ (base B)
      → Terms Γ (base C)
  edge₂ᵀ e = productWith (edge₂ e)

  pairᵀ :
      ∀ {Γ A B}
      → Terms Γ A
      → Terms Γ B
      → Terms Γ (A ⊗ B)
  pairᵀ = productWith pair

  match⊗ᵀ :
      ∀ {Γ A B C}
      → Terms Γ (A ⊗ B)
      → Terms (B ∷ A ∷ Γ) C
      → Terms Γ C
  match⊗ᵀ = productWith match⊗

  match𝟙ᵀ :
      ∀ {Γ A}
      → Terms Γ 𝟙
      → Terms Γ A
      → Terms Γ A
  match𝟙ᵀ = productWith match𝟙

  ----------------------------------------------------------------------
  -- Typing derivations
  --
  -- A derivation is a non-empty list of last-rule instances.  This is
  -- join introduction in flattened form: its order and duplicates are
  -- retained, but there is no binary join tree whose associativity
  -- would later have to be added to permutation equivalence.
  --
  -- A last-rule instance can itself have general derivations as
  -- premises.  This retains the intended non-uniqueness: a rule can
  -- consume a multi-summand premise directly, or its fully distributed
  -- instances can occur as separate entries of the outer derivation.

  mutual
    data RuleDerivation (Γ : Context) : (A : Type) → (Terms Γ A) → Set where
      varᵈ :
          ∀ {A} (i : A ∈ Γ)
          → RuleDerivation Γ A (singleton (var i))

      edge₂ᵈ :
          ∀ {A B C M N}
          → (e : Edge₂ A B C)
          → Derivation Γ (base A) M
          → Derivation Γ (base B) N
          → RuleDerivation Γ (base C) (edge₂ᵀ e M N) 

      unitᵈ :
          RuleDerivation Γ 𝟙 (singleton unit)

      pairᵈ :
          ∀ {A B M N}
          → Derivation Γ A M
          → Derivation Γ B N
          → RuleDerivation Γ (A ⊗ B) (pairᵀ M N)

      match⊗ᵈ :
          ∀ {A B C M N}
          → Derivation Γ (A ⊗ B) M
          → Derivation (B ∷ A ∷ Γ) C N
          → RuleDerivation Γ C (match⊗ᵀ M N)

      match𝟙ᵈ :
          ∀ {A M N}
          → Derivation Γ 𝟙 M
          → Derivation Γ A N
          → RuleDerivation Γ A (match𝟙ᵀ M N)

    data Derivation (Γ : Context) : (A : Type) → Terms Γ A → Set where

        oneᵈ :
            ∀ {A M}
            → RuleDerivation Γ A M
            → Derivation Γ A M

        consᵈ :
            ∀ {A M N}
            → RuleDerivation Γ A M
            → Derivation Γ A N
            → Derivation Γ A (M ⊞ N)

  data JFDerivation (Γ : Context) : (A : Type) → JFTerm Γ A → Set where

      varⁿ :
          ∀ {A} (i : A ∈ Γ) →
          JFDerivation Γ A (var i)

      edge₂ⁿ :
          ∀ {A B C} {M : JFTerm Γ (base A)}
                      {N : JFTerm Γ (base B)}
          → (e : Edge₂ A B C)
          → JFDerivation Γ (base A) M
          → JFDerivation Γ (base B) N
          → JFDerivation Γ (base C) (edge₂ e M N)

      unitⁿ :
          JFDerivation Γ 𝟙 unit

      pairⁿ :
          ∀ {A B M N}
          → JFDerivation Γ A M
          → JFDerivation Γ B N
          → JFDerivation Γ (A ⊗ B) (pair M N)
    
      match⊗ⁿ :
              ∀ {A B C M N}
              → JFDerivation Γ (A ⊗ B) M
              → JFDerivation (B ∷ A ∷ Γ) C N
              → JFDerivation Γ C (match⊗ M N)

      match𝟙ⁿ :
          ∀ {A M N}
          → JFDerivation Γ 𝟙 M
          → JFDerivation Γ A N
          → JFDerivation Γ A (match𝟙 M N)

  data NFDerivation (Γ : Context) (A : Type) : Terms Γ A → Set where

      oneⁿ :
          ∀ {M}
          → JFDerivation Γ A M
          → NFDerivation Γ A (singleton M)

      consⁿ :
          ∀ {M N Ns}
          → JFDerivation Γ A M
          → NFDerivation Γ A (mkList⁺ N Ns)
          → NFDerivation Γ A (mkList⁺ M (N ∷ Ns))

  mapNFDerivation :
    ∀ {Γ Δ A B}
    → (F : JFTerm Γ A → JFTerm Δ B)
    → (∀ {M}
       → JFDerivation Γ A M
       → JFDerivation Δ B (F M))
    → ∀ {P}
    → NFDerivation Γ A P
    → NFDerivation Δ B (List⁺.map F P)

  mapNFDerivation F f (oneⁿ x) =
    oneⁿ (f x)

  mapNFDerivation F f (consⁿ x xs) =
    consⁿ (f x) (mapNFDerivation F f xs)
  
  appendNFDerivation :
    ∀ {Γ A M N}
    → NFDerivation Γ A M
    → NFDerivation Γ A N
    → NFDerivation Γ A (M ⊞ N)

  appendNFDerivation (oneⁿ x) ys =
    consⁿ x ys

  appendNFDerivation (consⁿ x xs) ys =
    consⁿ x (appendNFDerivation xs ys)

  mutual
      normalise : forall {Γ A M} → Derivation Γ A M → NFDerivation Γ A M
      normalise (oneᵈ x) = normaliseRule x
      normalise (consᵈ x d) = let xⁿ = normaliseRule x in
                                 let dⁿ = normalise d in
                              appendNFDerivation xⁿ dⁿ

      normaliseRule : forall {Γ A M} → RuleDerivation Γ A M → NFDerivation Γ A M
      normaliseRule (varᵈ i) = oneⁿ (varⁿ i)
      normaliseRule {Γ = Γ} (edge₂ᵈ {A = A₁} {B = B₁} {C = C₁} e x x₁) = let l = normalise x in
                                      let r = normalise x₁ in helper l r where
                                        helper :
                                            ∀ {M N}
                                            → NFDerivation Γ (base A₁) M
                                            → NFDerivation Γ (base B₁) N
                                            → NFDerivation Γ (base C₁) (edge₂ᵀ e M N)

                                        helper (oneⁿ {M = m} x) r =
                                            mapNFDerivation
                                                (edge₂ e m)
                                                (edge₂ⁿ e x)
                                                r

                                        helper (consⁿ {M = m} x l) r =
                                            appendNFDerivation
                                                (mapNFDerivation
                                                (edge₂ e m)
                                                (edge₂ⁿ e x)
                                                r)
                                                (helper l r)
      normaliseRule unitᵈ = oneⁿ unitⁿ
      normaliseRule {Γ = Γ} (pairᵈ {A = A₁} {B = B₁} x x₁) = let l = normalise x in
                                                             let r = normalise x₁ in helper l r where 
                                        helper :
                                            ∀ {M N}
                                            → NFDerivation Γ (A₁) M
                                            → NFDerivation Γ (B₁) N
                                            → NFDerivation Γ (A₁ ⊗ B₁) (pairᵀ M N)
                                        helper (oneⁿ {M = m} x) r =
                                            mapNFDerivation
                                                (pair m)
                                                (pairⁿ x)
                                                r

                                        helper (consⁿ {M = m} x l) r =
                                            appendNFDerivation
                                                (mapNFDerivation
                                                (pair m)
                                                (pairⁿ x)
                                                r)
                                                (helper l r)


      normaliseRule {Γ = Γ} (match⊗ᵈ {A = A₁} {B = B₁} {C = C₁} x x₁) = let l = normalise x in
                                                             let r = normalise x₁ in helper l r where 
                                        helper :
                                            ∀ {M N}
                                            → NFDerivation Γ (A₁ ⊗ B₁) M
                                            → NFDerivation (B₁ ∷ A₁ ∷ Γ) (C₁) N
                                            → NFDerivation Γ (C₁) (match⊗ᵀ M N)
                                        helper (oneⁿ {M = m} x) r =
                                            mapNFDerivation
                                                (match⊗ m)
                                                (match⊗ⁿ x)
                                                r

                                        helper (consⁿ {M = m} x l) r =
                                            appendNFDerivation
                                                (mapNFDerivation
                                                (match⊗ m)
                                                (match⊗ⁿ x)
                                                r)
                                                (helper l r)
      normaliseRule {Γ = Γ} (match𝟙ᵈ {A = A₁} x x₁) = let l = normalise x in
                                             let r = normalise x₁ in helper l r where 
                                        helper :
                                            ∀ {M N}
                                            → NFDerivation Γ (𝟙) M
                                            → NFDerivation Γ (A₁) N
                                            → NFDerivation Γ (A₁) (match𝟙ᵀ M N)
                                        helper (oneⁿ {M = m} x) r =
                                            mapNFDerivation
                                                (match𝟙 m)
                                                (match𝟙ⁿ x)
                                                r

                                        helper (consⁿ {M = m} x l) r =
                                            appendNFDerivation
                                                (mapNFDerivation
                                                (match𝟙 m)
                                                (match𝟙ⁿ x)
                                                r)
                                                (helper l r)
  data _∼_ {Γ : Context} {A : Type} {M : Terms Γ A} : Derivation Γ A M → Derivation Γ A M → Set where
      reflᵈ : (d : Derivation Γ A M) → d ∼ d

module Example where
    open Syntax G₁'

    example₁ : Derivation ((base N) ∷ []) (base N) (edge₂ᵀ plus (singleton (var here)) (singleton (var here)))
    example₁ = let context = (base N) ∷ [] in
               let 
                   var' : Derivation ((base N) ∷ []) (base N) (singleton (var here))
                   var' =  oneᵈ (varᵈ (here)) in 
               let π = oneᵈ (edge₂ᵈ plus var' var') in π
    

    example₂ : Derivation ((base N) ∷ []) (base N) ((edge₂ᵀ plus (singleton (var here)) (singleton (var here))) ⊞ (edge₂ᵀ plus (singleton (var here)) (singleton (var here))))
    example₂ = let context = (base N) ∷ [] in
               let 
                   var' : Derivation ((base N) ∷ []) (base N) (singleton (var here))
                   var' =  oneᵈ (varᵈ (here)) in 
               let π = (edge₂ᵈ plus var' var') in
               let π⊞π = consᵈ π (oneᵈ π) in π⊞π

    example₃ : Derivation ((base N) ∷ []) (base N) ((edge₂ᵀ plus (singleton (var here)) (singleton (var here))) ⊞ (edge₂ᵀ plus (singleton (var here)) (singleton (var here))))
    example₃ = let context = (base N) ∷ [] in
               let 
                   var' : RuleDerivation ((base N) ∷ []) (base N) (singleton (var here))
                   var' =  (varᵈ (here)) in 
               let var'⊞var' = consᵈ var' (oneᵈ var') in oneᵈ (edge₂ᵈ plus var'⊞var'  (oneᵈ var'))

    checkExample₂Example₃ : normalise example₂ ≡ normalise example₃
    checkExample₂Example₃ = refl