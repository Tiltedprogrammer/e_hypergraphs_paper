use std::ops::Index;

use egg::EGraph;
use egg::{rewrite as rw, *};
use fxhash::FxHashSet as HashSet;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

define_language! {
    enum CommutativeComonoids {
        "mu" = Mu, // mu : 1 -> 2
        "i" = I, // i : 1 -> 0
        ";" = Compose([Id;2]),
        "parallel" = Parallel([Id;2]),
        "id_0" = Identity0,
        "id_1" = Identity1,
        "id_2" = Identity2,
        "id_3" = Identity3,
        "id_4" = Identity4,
        "sym_0_1" = Sym0_1,
        "sym_1_0" = Sym1_0,
        "sym_1_1" = Sym1_1,
        "sym_1_2" = Sym1_2,
        "sym_2_1" = Sym2_1,
        "sym_2_2" = Sym2_2,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Ty {
    dom: usize,
    cod: usize,
}

#[derive(Default)]
struct TypeAnalysis;

impl Analysis<CommutativeComonoids> for TypeAnalysis {
    type Data = Ty;

    fn make(
        egraph: &mut EGraph<CommutativeComonoids, TypeAnalysis>,
        enode: &CommutativeComonoids,
        id: Id,
    ) -> Self::Data {
        use CommutativeComonoids::*;

        let get = |id: &Id| egraph[*id].data;

        match enode {
            Mu => Ty { dom: 1, cod: 2 },
            I => Ty { dom: 1, cod: 0 },

            Identity0 => Ty { dom: 0, cod: 0 },
            Identity1 => Ty { dom: 1, cod: 1 },
            Identity2 => Ty { dom: 2, cod: 2 },
            Identity3 => Ty { dom: 3, cod: 3 },
            Identity4 => Ty { dom: 4, cod: 4 },

            Sym0_1 => Ty { dom: 1, cod: 1 },
            Sym1_0 => Ty { dom: 1, cod: 1 },
            Sym1_1 => Ty { dom: 2, cod: 2 },
            Sym1_2 => Ty { dom: 3, cod: 3 },
            Sym2_1 => Ty { dom: 3, cod: 3 },
            Sym2_2 => Ty { dom: 4, cod: 4 },

            Compose([f, g]) => match (get(f), get(g)) {
                (t1, t2) => Ty {
                    dom: t1.dom,
                    cod: t2.cod,
                },
            },

            Parallel([f, g]) => match (get(f), get(g)) {
                (t1, t2) => Ty {
                    dom: t1.dom + t2.dom,
                    cod: t1.cod + t2.cod,
                },
            },
        }
    }

    fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
        if to == &from {
            DidMerge(false, false)
        } else {
            DidMerge(true, true)
        }
    }
}

// typed version

define_language! {
    enum Arith {
        "+" = Add([Id;2]),
        "*" = Mul([Id;2]),
        "<<" = Shift([Id;2]),
        "/" = Div([Id;2]),
        Num(i32),
        Symbol(egg::Symbol),
    }
}

fn rulesArith() -> Vec<Rewrite<Arith, ()>> {
    vec![
        rw!("times-2-is-shift-left"; "(* ?a 2)" => "(<< ?a 1)"),
        rw!("times-2-is-shift-right"; "(* 2 ?a)" => "(<< ?a 1)"),
        rw!("times-2-is-shift-right"; "(* 2 ?a)" => "(<< ?a 1)"),
        rw!("div-mul-assoc"; "(/ (* ?a ?b) ?c)" => "(/ ?a (* ?b ?c))"),
        rw!("div-id-left"; "(/ ?a ?a)" => "(1)"),
        rw!("div-id-right"; "(/ ?)" => "(1)"),
        rw!("div-id"; "(/ ?a ?a)" => "(1)"),
    ]
}

type Log = Arc<Mutex<HashMap<String, u64>>>;

#[derive(Debug, Clone)]
struct LogApplier {
    log: Log,
    pattern: Pattern<CommutativeComonoids>,
}

impl Applier<CommutativeComonoids, TypeAnalysis> for LogApplier {
    fn apply_one(
        &self,
        egraph: &mut EGraph<CommutativeComonoids, TypeAnalysis>,
        matched_id: Id,
        subst: &Subst,
        searcher_ast: Option<&PatternAst<CommutativeComonoids>>,
        rule_name: Symbol,
    ) -> Vec<Id> {
        // println!("Applied rule {}", rule_name);
        let result = self
            .pattern
            .apply_one(egraph, matched_id, subst, searcher_ast, rule_name);
        if result.len() > 0 {
            let mut map = self.log.lock().unwrap();
            map.entry(rule_name.to_string())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        result
    }
}

fn do_types_match(
    v1: Var,
    v2: Var,
) -> impl Fn(&mut EGraph<CommutativeComonoids, TypeAnalysis>, Id, &Subst) -> bool {
    move |egraph, _, subst| {
        let id1 = egraph.find(subst[v1]);
        let id2 = egraph.find(subst[v2]);
        egraph[id1].data.cod == egraph[id2].data.dom
    }
}

fn check_cod_type(
    v1: Var,
    t: usize,
) -> impl Fn(&mut EGraph<CommutativeComonoids, TypeAnalysis>, Id, &Subst) -> bool {
    move |egraph, _, subst| {
        let id1 = egraph.find(subst[v1]);
        egraph[id1].data.cod == t
    }
}

fn check_dom_type(
    v1: Var,
    t: usize,
) -> impl Fn(&mut EGraph<CommutativeComonoids, TypeAnalysis>, Id, &Subst) -> bool {
    move |egraph, _, subst| {
        let id1 = egraph.find(subst[v1]);
        egraph[id1].data.dom == t
    }
}

fn both_dom(
    v1: Var,
    n1: usize,
    v2: Var,
    n2: usize,
) -> impl Fn(&mut EGraph<CommutativeComonoids, TypeAnalysis>, Id, &Subst) -> bool {
    move |egraph, _, subst| {
        let id1 = egraph.find(subst[v1]);
        let id2 = egraph.find(subst[v2]);
        egraph[id1].data.dom == n1 && egraph[id2].data.dom == n2
    }
}

fn nat_types(
    nf: Var,
    ng: Var,
    n: usize,
    m: usize,
) -> impl Fn(&mut EGraph<CommutativeComonoids, TypeAnalysis>, Id, &Subst) -> bool {
    move |egraph, _, subst| {
        let f = egraph.find(subst[nf]);
        let g = egraph.find(subst[ng]);

        let tf = &egraph[f].data;
        let tg = &egraph[g].data;

        tf.dom == n && tf.cod == n && tg.dom == m && tg.cod == m
    }
}

fn rules_comonoid(log: Log) -> Vec<Rewrite<CommutativeComonoids, TypeAnalysis>> {
    let mut rules: Vec<Rewrite<CommutativeComonoids, TypeAnalysis>> = Vec::new();
    rules.extend(vec![
        rw!("assoc-compose-left"; "(; ?x (; ?y ?z))" => { LogApplier {log : log.clone(), pattern: "(; (; ?x ?y) ?z)".parse().unwrap()} }),
        rw!("assoc-compose-right"; "(; (; ?x ?y) ?z)" => { LogApplier {log : log.clone(), pattern: "(; ?x (; ?y ?z))".parse().unwrap()} })
        ]);

    // identities of 1
    rules.extend(vec![
        rw!("id_1-left-compose-left"; "(; id_1 ?x)" => {LogApplier {log : log.clone(), pattern: "(?x)".parse().unwrap()} }
        if check_dom_type(var("?x"), 1)
    ),
        rw!("id_1-right-compose-left"; "(; ?x id_1)" => {LogApplier {log : log.clone(), pattern: "(?x)".parse().unwrap()} }
        if check_cod_type(var("?x"), 1)
    ),
        rw!("id_1-left-compose-right"; "(?x)" => {LogApplier {log : log.clone(), pattern: "(; id_1 ?x)".parse().unwrap()} }
        if check_dom_type(var("?x"), 1)
    ),
        rw!("id_1-right-compose-right"; "(?x)" => {LogApplier {log : log.clone(), pattern: "(; ?x id_1)".parse().unwrap()} }
        if check_cod_type(var("?x"), 1)
    )
    ]);
    // identities of 2
    rules.extend(vec![
        rw!("id_2-left-compose-left"; "(; id_2 ?x)" => {LogApplier {log : log.clone(), pattern: "(?x)".parse().unwrap()} }
        if check_dom_type(var("?x"), 2)
    ),
        rw!("id_2-right-compose-left"; "(; ?x id_2)" => {LogApplier {log : log.clone(), pattern: "(?x)".parse().unwrap()} }
        if check_cod_type(var("?x"), 2)
    ),
        rw!("id_2-left-compose-right"; "(?x)" => {LogApplier {log : log.clone(), pattern: "(; id_2 ?x)".parse().unwrap()} }
        if check_dom_type(var("?x"), 2)
    ),
        rw!("id_2-right-compose-right"; "(?x)" => {LogApplier {log : log.clone(), pattern: "(; ?x id_2)".parse().unwrap()} }
        if check_cod_type(var("?x"), 2)
    )
    ]);

    // identities of 3
    rules.extend(vec![
        rw!("id_3-left-compose-left"; "(; id_3 ?x)" => {LogApplier {log : log.clone(), pattern: "(?x)".parse().unwrap()} }
        if check_dom_type(var("?x"), 3)
    ),
        rw!("id_3-right-compose-left"; "(; ?x id_3)" => {LogApplier {log : log.clone(), pattern: "(?x)".parse().unwrap()} }
        if check_cod_type(var("?x"), 3)
    ),
        rw!("id_3-left-compose-right"; "(?x)" => {LogApplier {log : log.clone(), pattern: "(; id_3 ?x)".parse().unwrap()} }
        if check_dom_type(var("?x"), 3)
    ),
        rw!("id_3-right-compose-right"; "(?x)" => {LogApplier {log : log.clone(), pattern: "(; ?x id_3)".parse().unwrap()} }
        if check_cod_type(var("?x"), 3)
    )
    ]);

    // identities of 4
    rules.extend(vec![
        rw!("id_4-left-compose-left"; "(; id_4 ?x)" => {LogApplier {log : log.clone(), pattern: "(?x)".parse().unwrap()} }
        if check_dom_type(var("?x"), 4)
    ),
        rw!("id_4-right-compose-left"; "(; ?x id_4)" => {LogApplier {log : log.clone(), pattern: "(?x)".parse().unwrap()} }
        if check_cod_type(var("?x"), 4)
    ),
        rw!("id_4-left-compose-right"; "(?x)" => {LogApplier {log : log.clone(), pattern: "(; id_4 ?x)".parse().unwrap()} }
        if check_dom_type(var("?x"), 4)
    ),
        rw!("id_4-right-compose-right"; "(?x)" => {LogApplier {log : log.clone(), pattern: "(; ?x id_4)".parse().unwrap()} }
        if check_cod_type(var("?x"), 4)
    )
    ]);

    rules.extend(vec![
        rw!("parallel-bifunctor-left"; "(parallel (; ?x ?y) (; ?z ?w))" => {LogApplier {log : log.clone(), pattern : "(; (parallel ?x ?z) (parallel ?y ?w))".parse().unwrap()} }),
        rw!("parallel-bifunctor-right"; "(; (parallel ?x ?z) (parallel ?y ?w))" => {LogApplier {log : log.clone(), pattern : "(parallel (; ?x ?y) (; ?z ?w))".parse().unwrap()} }
            if (|egraph, w, subst | do_types_match(var("?x"), var("?y"))(egraph, w, subst) && do_types_match(var("?z"), var("?w"))(egraph, w, subst)) as fn(&mut EGraph<CommutativeComonoids, TypeAnalysis>, Id, &Subst) -> bool
        )
    ]);

    rules.extend(vec![
    // idn ; s ≡ s ≡ s ; idn    
    rw!("sym-inverse-1-left";"(; sym_1_1 sym_1_1)" => { LogApplier { log : log.clone(), pattern : "(parallel id_1 id_1)".parse().unwrap()}}),
    rw!("sym-inverse-1-right";"(parallel id_1 id_1)" => { LogApplier { log : log.clone(), pattern : "(; sym_1_1 sym_1_1)".parse().unwrap()}}),
    rw!("sym-naturality-right"; "(; (parallel ?f ?g) sym_1_1)" => {LogApplier {log : log.clone(), pattern: "(; sym_1_1 (parallel ?g ?f))".parse().unwrap()} }
        if (|egraph, w, subst | check_cod_type(var("?f"), 1)(egraph, w, subst) 
                              && check_cod_type(var("?g"), 1)(egraph, w, subst)
                            && check_dom_type(var("?f"), 1)(egraph, w, subst)
                        && check_dom_type(var("?g"), 1)(egraph, w, subst)) as fn(&mut EGraph<CommutativeComonoids, TypeAnalysis>, Id, &Subst) -> bool
    ),
    rw!("sym-naturality-left"; "(; sym_1_1 (parallel ?g ?f))" => {LogApplier {log : log.clone(), pattern: "(; (parallel ?f ?g) sym_1_1)".parse().unwrap()} }
        if (|egraph, w, subst | check_cod_type(var("?f"), 1)(egraph, w, subst) 
                              && check_cod_type(var("?g"), 1)(egraph, w, subst)
                            && check_dom_type(var("?f"), 1)(egraph, w, subst)
                        && check_dom_type(var("?g"), 1)(egraph, w, subst)) as fn(&mut EGraph<CommutativeComonoids, TypeAnalysis>, Id, &Subst) -> bool
    ),
    // comonoid laws
    rw!("counit-left"; "(; mu (parallel i id_1))" => { LogApplier {log : log.clone(), pattern : "(id_1)".parse().unwrap()}}),
    // this one follows by commutativity
    // rw!("counit-right"; "(; mu (parallel id i))" => { LogApplier {log : log.clone(), pattern : "(id)".parse().unwrap()}}),
    rw!("comult-commutative-right"; "(; mu sym_1_1)" => {LogApplier {log : log.clone(), pattern : "(mu)".parse().unwrap()}}),
    rw!("comult-commutative-left"; "(mu)" => {LogApplier {log : log.clone(), pattern : "(; mu sym_1_1)".parse().unwrap()}})
    ]);

    rules.extend(vec![
    rw!("comult-coassoc-left";
        "(; mu (parallel mu id_1))" =>
        { LogApplier { log: log.clone(), pattern: "(; mu (parallel id_1 mu))".parse().unwrap() } }
    ),
    rw!("comult-coassoc-right";
        "(; mu (parallel id_1 mu))" =>
        { LogApplier { log: log.clone(), pattern: "(; mu (parallel mu id_1))".parse().unwrap() } }
    )
    ]);

    rules.extend(vec![
    rw!("parallel-assoc-left";
        "(parallel ?x (parallel ?y ?z))" => {
            LogApplier { log: log.clone(), pattern: "(parallel (parallel ?x ?y) ?z)".parse().unwrap() }
        }
    ),
    rw!("parallel-assoc-right";
        "(parallel (parallel ?x ?y) ?z)" => {
            LogApplier { log: log.clone(), pattern: "(parallel ?x (parallel ?y ?z))".parse().unwrap() }
        }
    ),
    ]);

    rules.extend(vec![
        rw!("parallel-id0-left";
            "(parallel id_0 ?x)" => {
                LogApplier { log: log.clone(), pattern: "(?x)".parse().unwrap() }
            }
        ),
        rw!("parallel-id0-right";
            "(parallel ?x id_0)" => {
                LogApplier { log: log.clone(), pattern: "(?x)".parse().unwrap() }
            }
        ),
        rw!("parallel-id0-left-inv";
            "(?x)" => {
                LogApplier { log: log.clone(), pattern: "(parallel id_0 ?x)".parse().unwrap() }
            }
        ),
        rw!("parallel-id0-right-inv";
            "(?x)" => {
                LogApplier { log: log.clone(), pattern: "(parallel ?x id_0)".parse().unwrap() }
            }
        ),
    ]);

    rules.extend(vec![
    rw!("id_2_def";
        "(id_2)" => {
            LogApplier { log: log.clone(), pattern: "(parallel id_1 id_1)".parse().unwrap() }
        }
    ),
    rw!("id_2_def_inv";
        "(parallel id_1 id_1)" => {
            LogApplier { log: log.clone(), pattern: "(id_2)".parse().unwrap() }
        }
    ),

    rw!("id_3_def";
        "(id_3)" => {
            LogApplier { log: log.clone(), pattern: "(parallel id_1 (parallel id_1 id_1))".parse().unwrap() }
        }
    ),
    rw!("id_3_def_inv";
        "(parallel id_1 (parallel id_1 id_1))" => {
            LogApplier { log: log.clone(), pattern: "(id_3)".parse().unwrap() }
        }
    ),

    rw!("id_4_def";
        "(id_4)" => {
            LogApplier { log: log.clone(), pattern: "(parallel id_2 id_2)".parse().unwrap() }
        }
    ),
    rw!("id_4_def_inv";
        "(parallel id_2 id_2)" => {
            LogApplier { log: log.clone(), pattern: "(id_4)".parse().unwrap() }
        }
    ),
    ]);

    rules.extend(vec![
        rw!("sym_2_1_def";
            "(sym_2_1)" => {
                LogApplier {
                    log: log.clone(),
                    pattern: "(; (parallel sym_1_1 id_1) (parallel id_1 sym_1_1))".parse().unwrap()
                }
            }
        ),
        rw!("sym_2_1_def_inv";
            "(; (parallel sym_1_1 id_1) (parallel id_1 sym_1_1))" => {
                LogApplier {
                    log: log.clone(),
                    pattern: "(sym_2_1)".parse().unwrap()
                }
            }
        ),
    ]);

    rules.extend(vec![
        rw!("sym_1_2_def";
            "(sym_1_2)" => {
                LogApplier {
                    log: log.clone(),
                    pattern: "(; (parallel id_1 sym_1_1) (parallel sym_1_1 id_1))".parse().unwrap()
                }
            }
        ),
        rw!("sym_1_2_def_inv";
            "(; (parallel id_1 sym_1_1) (parallel sym_1_1 id_1))" => {
                LogApplier {
                    log: log.clone(),
                    pattern: "(sym_1_2)".parse().unwrap()
                }
            }
        ),
    ]);

    rules.extend(vec![
        rw!("sym_2_2_def";
            "(sym_2_2)" => {
                LogApplier {
                    log: log.clone(),
                    pattern:
                        "(; (parallel sym_1_2 id_1)
                            (parallel id_1 sym_2_1))"
                            .parse().unwrap()
                }
            }
        ),
        rw!("sym_2_2_def_inv";
            "(; (parallel sym_1_2 id_1)
                (parallel id_1 sym_2_1))" => {
                LogApplier {
                    log: log.clone(),
                    pattern: "(sym_2_2)".parse().unwrap()
                }
            }
        ),
    ]);

    rules.extend(vec![
        rw!("sym_2_1_inv";
            "(; sym_2_1 sym_1_2)" => {
                LogApplier { log: log.clone(), pattern: "(id_3)".parse().unwrap() }
            }
        ),
        rw!("sym_1_2_inv";
            "(; sym_1_2 sym_2_1)" => {
                LogApplier { log: log.clone(), pattern: "(id_3)".parse().unwrap() }
            }
        ),
        rw!("sym_2_2_inv";
            "(; sym_2_2 sym_2_2)" => {
                LogApplier { log: log.clone(), pattern: "(id_4)".parse().unwrap() }
            }
        ),
    ]);

    rules.extend(vec![
        rw!("sym_0_1_id_1";
            "(sym_0_1)" => {
                LogApplier { log: log.clone(), pattern: "(id_1)".parse().unwrap() }
            }
        ),
        rw!("sym_1_0_id_1";
            "(sym_1_0)" => {
                LogApplier { log: log.clone(), pattern: "(id_1)".parse().unwrap() }
            }
        ),
    ]);

    rules.extend(vec![

        // (sym_1_1) ; (f ⊗ g)  ==>  (g ⊗ f) ; sym_{cod(g), cod(f)}

        // 1,1 → 1,1
    rw!("sym-nat-1-1";
        "(; sym_1_1 (parallel ?f ?g))" =>
        { LogApplier { log: log.clone(), pattern: "(; (parallel ?g ?f) sym_1_1)".parse().unwrap() } }
        if (|egraph, id, subst|
            check_dom_type(var("?f"),1)(egraph,id,subst) &&
            check_dom_type(var("?g"),1)(egraph,id,subst) &&
            check_cod_type(var("?f"),1)(egraph,id,subst) &&
            check_cod_type(var("?g"),1)(egraph,id,subst)
        ) as fn(&mut EGraph<_,_>, Id, &Subst) -> bool
    ),

    rw!("sym-nat-1-1-inv";
        "(; (parallel ?g ?f) sym_1_1)" =>
        { LogApplier { log: log.clone(), pattern: "(; sym_1_1 (parallel ?f ?g))".parse().unwrap() } }
        if (|egraph, id, subst|
            check_dom_type(var("?f"),1)(egraph,id,subst) &&
            check_dom_type(var("?g"),1)(egraph,id,subst) &&
            check_cod_type(var("?f"),1)(egraph,id,subst) &&
            check_cod_type(var("?g"),1)(egraph,id,subst)
        ) as fn(&mut EGraph<_,_>, Id, &Subst) -> bool
    ),

        // 1,0 → 0,1
    rw!("sym-nat-1-0";
    "(; sym_1_1 (parallel ?f ?g))" =>
    { LogApplier { log: log.clone(), pattern: "(; (parallel ?g ?f) sym_0_1)".parse().unwrap() } }
    if (|egraph, id, subst|
        check_dom_type(var("?f"),1)(egraph,id,subst) &&
        check_dom_type(var("?g"),1)(egraph,id,subst) &&
        check_cod_type(var("?f"),1)(egraph,id,subst) &&
        check_cod_type(var("?g"),0)(egraph,id,subst)
    ) as fn(&mut EGraph<_,_>, Id, &Subst) -> bool
),

rw!("sym-nat-1-0-inv";
    "(; (parallel ?g ?f) sym_0_1)" =>
    { LogApplier { log: log.clone(), pattern: "(; sym_1_1 (parallel ?f ?g))".parse().unwrap() } }
    if (|egraph, id, subst|
        check_dom_type(var("?f"),1)(egraph,id,subst) &&
        check_dom_type(var("?g"),1)(egraph,id,subst) &&
        check_cod_type(var("?f"),1)(egraph,id,subst) &&
        check_cod_type(var("?g"),0)(egraph,id,subst)
    ) as fn(&mut EGraph<_,_>, Id, &Subst) -> bool
),


rw!("sym-nat-0-1";
    "(; sym_1_1 (parallel ?f ?g))" =>
    { LogApplier { log: log.clone(), pattern: "(; (parallel ?g ?f) sym_1_0)".parse().unwrap() } }
    if (|egraph, id, subst|
        check_dom_type(var("?f"),1)(egraph,id,subst) &&
        check_dom_type(var("?g"),1)(egraph,id,subst) &&
        check_cod_type(var("?f"),0)(egraph,id,subst) &&
        check_cod_type(var("?g"),1)(egraph,id,subst)
    ) as fn(&mut EGraph<_,_>, Id, &Subst) -> bool
),

rw!("sym-nat-0-1-inv";
    "(; (parallel ?g ?f) sym_1_0)" =>
    { LogApplier { log: log.clone(), pattern: "(; sym_1_1 (parallel ?f ?g))".parse().unwrap() } }
    if (|egraph, id, subst|
        check_dom_type(var("?f"),1)(egraph,id,subst) &&
        check_dom_type(var("?g"),1)(egraph,id,subst) &&
        check_cod_type(var("?f"),0)(egraph,id,subst) &&
        check_cod_type(var("?g"),1)(egraph,id,subst)
    ) as fn(&mut EGraph<_,_>, Id, &Subst) -> bool
),

rw!("sym-nat-1-2";
    "(; sym_1_1 (parallel ?f ?g))" =>
    { LogApplier { log: log.clone(), pattern: "(; (parallel ?g ?f) sym_2_1)".parse().unwrap() } }
    if (|egraph, id, subst|
        check_dom_type(var("?f"),1)(egraph,id,subst) &&
        check_dom_type(var("?g"),1)(egraph,id,subst) &&
        check_cod_type(var("?f"),1)(egraph,id,subst) &&
        check_cod_type(var("?g"),2)(egraph,id,subst)
    ) as fn(&mut EGraph<_,_>, Id, &Subst) -> bool
),

rw!("sym-nat-1-2-inv";
    "(; (parallel ?g ?f) sym_2_1)" =>
    { LogApplier { log: log.clone(), pattern: "(; sym_1_1 (parallel ?f ?g))".parse().unwrap() } }
    if (|egraph, id, subst|
        check_dom_type(var("?f"),1)(egraph,id,subst) &&
        check_dom_type(var("?g"),1)(egraph,id,subst) &&
        check_cod_type(var("?f"),1)(egraph,id,subst) &&
        check_cod_type(var("?g"),2)(egraph,id,subst)
    ) as fn(&mut EGraph<_,_>, Id, &Subst) -> bool
),

rw!("sym-nat-2-1";
    "(; sym_1_1 (parallel ?f ?g))" =>
    { LogApplier { log: log.clone(), pattern: "(; (parallel ?g ?f) sym_1_2)".parse().unwrap() } }
    if (|egraph, id, subst|
        check_dom_type(var("?f"),1)(egraph,id,subst) &&
        check_dom_type(var("?g"),1)(egraph,id,subst) &&
        check_cod_type(var("?f"),2)(egraph,id,subst) &&
        check_cod_type(var("?g"),1)(egraph,id,subst)
    ) as fn(&mut EGraph<_,_>, Id, &Subst) -> bool
),

rw!("sym-nat-2-1-inv";
    "(; (parallel ?g ?f) sym_1_2)" =>
    { LogApplier { log: log.clone(), pattern: "(; sym_1_1 (parallel ?f ?g))".parse().unwrap() } }
    if (|egraph, id, subst|
        check_dom_type(var("?f"),1)(egraph,id,subst) &&
        check_dom_type(var("?g"),1)(egraph,id,subst) &&
        check_cod_type(var("?f"),2)(egraph,id,subst) &&
        check_cod_type(var("?g"),1)(egraph,id,subst)
    ) as fn(&mut EGraph<_,_>, Id, &Subst) -> bool
),

rw!("sym-nat-2-2";
    "(; sym_1_1 (parallel ?f ?g))" =>
    { LogApplier { log: log.clone(), pattern: "(; (parallel ?g ?f) sym_2_2)".parse().unwrap() } }
    if (|egraph, id, subst|
        check_dom_type(var("?f"),1)(egraph,id,subst) &&
        check_dom_type(var("?g"),1)(egraph,id,subst) &&
        check_cod_type(var("?f"),2)(egraph,id,subst) &&
        check_cod_type(var("?g"),2)(egraph,id,subst)
    ) as fn(&mut EGraph<_,_>, Id, &Subst) -> bool
),

rw!("sym-nat-2-2-inv";
    "(; (parallel ?g ?f) sym_2_2)" =>
    { LogApplier { log: log.clone(), pattern: "(; sym_1_1 (parallel ?f ?g))".parse().unwrap() } }
    if (|egraph, id, subst|
        check_dom_type(var("?f"),1)(egraph,id,subst) &&
        check_dom_type(var("?g"),1)(egraph,id,subst) &&
        check_cod_type(var("?f"),2)(egraph,id,subst) &&
        check_cod_type(var("?g"),2)(egraph,id,subst)
    ) as fn(&mut EGraph<_,_>, Id, &Subst) -> bool
)

    ]);

    return rules;
    // id0 ⊗ s ≡ s ⊗ id0
    // (s ⊗ t) ⊗ u ≡ (s ⊗ t) ⊗ u
    // (s ; u) ⊗ (t ; v) ≡ (s ⊗ t) ; (u ⊗ v)
    // (idm ⊗ idn) ≡ idm+n
    // (σm,n ⊗ ido); (idn ⊗ σm,o) ≡ σm,n+o
    // σm,n; σn,m ≡ idm+n
    // (s ⊗ idm) ; σn,m = σn,m ; (idm ⊗ s
}

define_language! {
    enum Lambda {
        Bool(bool),
        Num(i32),

        "var" = Var(Id),

        "+" = Add([Id; 2]),
        "=" = Eq([Id; 2]),

        "app" = App([Id; 2]),
        "lam" = Lambda([Id; 2]),
        "let" = Let([Id; 3]),
        "fix" = Fix([Id; 2]),

        "if" = If([Id; 3]),

        Symbol(egg::Symbol),
    }
}

impl Lambda {
    fn num(&self) -> Option<i32> {
        match self {
            Lambda::Num(n) => Some(*n),
            _ => None,
        }
    }
}

type EGraphLambda = egg::EGraph<Lambda, LambdaAnalysis>;

#[derive(Default)]
struct LambdaAnalysis;

#[derive(Debug)]
struct Data {
    free: HashSet<Id>,
    constant: Option<(Lambda, PatternAst<Lambda>)>,
}

fn eval(egraph: &EGraphLambda, enode: &Lambda) -> Option<(Lambda, PatternAst<Lambda>)> {
    let x = |i: &Id| egraph[*i].data.constant.as_ref().map(|c| &c.0);
    match enode {
        Lambda::Num(n) => Some((enode.clone(), format!("{}", n).parse().unwrap())),
        Lambda::Bool(b) => Some((enode.clone(), format!("{}", b).parse().unwrap())),
        Lambda::Add([a, b]) => Some((
            Lambda::Num(x(a)?.num()?.checked_add(x(b)?.num()?)?),
            format!("(+ {} {})", x(a)?, x(b)?).parse().unwrap(),
        )),
        Lambda::Eq([a, b]) => Some((
            Lambda::Bool(x(a)? == x(b)?),
            format!("(= {} {})", x(a)?, x(b)?).parse().unwrap(),
        )),
        _ => None,
    }
}

impl Analysis<Lambda> for LambdaAnalysis {
    type Data = Data;
    fn merge(&mut self, to: &mut Data, from: Data) -> DidMerge {
        let before_len = to.free.len();
        // to.free.extend(from.free);
        to.free.retain(|i| from.free.contains(i));
        // compare lengths to see if I changed to or from
        DidMerge(
            before_len != to.free.len(),
            to.free.len() != from.free.len(),
        ) | merge_option(&mut to.constant, from.constant, |a, b| {
            assert_eq!(a.0, b.0, "Merged non-equal constants");
            DidMerge(false, false)
        })
    }

    fn make(egraph: &mut EGraphLambda, enode: &Lambda, id: Id) -> Data {
        let f = |i: &Id| egraph[*i].data.free.iter().cloned();
        let mut free = HashSet::default();
        match enode {
            Lambda::Var(v) => {
                free.insert(*v);
            }
            Lambda::Let([v, a, b]) => {
                free.extend(f(b));
                free.remove(v);
                free.extend(f(a));
            }
            Lambda::Lambda([v, a]) | Lambda::Fix([v, a]) => {
                free.extend(f(a));
                free.remove(v);
            }
            _ => enode.for_each(|c| free.extend(&egraph[c].data.free)),
        }
        let constant = eval(egraph, enode);
        Data { constant, free }
    }

    fn modify(egraph: &mut EGraphLambda, id: Id) {
        if let Some(c) = egraph[id].data.constant.clone() {
            if egraph.are_explanations_enabled() {
                egraph.union_instantiations(
                    &c.0.to_string().parse().unwrap(),
                    &c.1,
                    &Default::default(),
                    "analysis".to_string(),
                );
            } else {
                let const_id = egraph.add(c.0);
                egraph.union(id, const_id);
            }
        }
    }
}

fn var(s: &str) -> Var {
    s.parse().unwrap()
}

fn is_not_same_var(v1: Var, v2: Var) -> impl Fn(&mut EGraphLambda, Id, &Subst) -> bool {
    move |egraph, _, subst| egraph.find(subst[v1]) != egraph.find(subst[v2])
}

fn is_const(v: Var) -> impl Fn(&mut EGraphLambda, Id, &Subst) -> bool {
    move |egraph, _, subst| egraph[subst[v]].data.constant.is_some()
}

fn rules() -> Vec<Rewrite<Lambda, LambdaAnalysis>> {
    vec![
        // open term rules
        rw!("if-true";  "(if  true ?then ?else)" => "?then"),
        rw!("if-false"; "(if false ?then ?else)" => "?else"),
        rw!("if-elim"; "(if (= (var ?x) ?e) ?then ?else)" => "?else"
            if ConditionEqual::parse("(let ?x ?e ?then)", "(let ?x ?e ?else)")),
        rw!("add-comm";  "(+ ?a ?b)"        => "(+ ?b ?a)"),
        rw!("add-assoc"; "(+ (+ ?a ?b) ?c)" => "(+ ?a (+ ?b ?c))"),
        rw!("eq-comm";   "(= ?a ?b)"        => "(= ?b ?a)"),
        // subst rules
        rw!("fix";      "(fix ?v ?e)"             => "(let ?v (fix ?v ?e) ?e)"),
        rw!("beta";     "(app (lam ?v ?body) ?e)" => "(let ?v ?e ?body)"),
        rw!("let-app";  "(let ?v ?e (app ?a ?b))" => "(app (let ?v ?e ?a) (let ?v ?e ?b))"),
        rw!("let-add";  "(let ?v ?e (+   ?a ?b))" => "(+   (let ?v ?e ?a) (let ?v ?e ?b))"),
        rw!("let-eq";   "(let ?v ?e (=   ?a ?b))" => "(=   (let ?v ?e ?a) (let ?v ?e ?b))"),
        rw!("let-const";
            "(let ?v ?e ?c)" => "?c" if is_const(var("?c"))),
        rw!("let-if";
            "(let ?v ?e (if ?cond ?then ?else))" =>
            "(if (let ?v ?e ?cond) (let ?v ?e ?then) (let ?v ?e ?else))"
        ),
        rw!("let-var-same"; "(let ?v1 ?e (var ?v1))" => "?e"),
        rw!("let-var-diff"; "(let ?v1 ?e (var ?v2))" => "(var ?v2)"
            if is_not_same_var(var("?v1"), var("?v2"))),
        rw!("let-lam-same"; "(let ?v1 ?e (lam ?v1 ?body))" => "(lam ?v1 ?body)"),
        rw!("let-lam-diff";
            "(let ?v1 ?e (lam ?v2 ?body))" =>
            { CaptureAvoid {
                fresh: var("?fresh"), v2: var("?v2"), e: var("?e"),
                if_not_free: "(lam ?v2 (let ?v1 ?e ?body))".parse().unwrap(),
                if_free: "(lam ?fresh (let ?v1 ?e (let ?v2 (var ?fresh) ?body)))".parse().unwrap(),
            }}
            if is_not_same_var(var("?v1"), var("?v2"))),
    ]
}

struct CaptureAvoid {
    fresh: Var,
    v2: Var,
    e: Var,
    if_not_free: Pattern<Lambda>,
    if_free: Pattern<Lambda>,
}

impl Applier<Lambda, LambdaAnalysis> for CaptureAvoid {
    fn apply_one(
        &self,
        egraph: &mut EGraphLambda,
        eclass: Id,
        subst: &Subst,
        searcher_ast: Option<&PatternAst<Lambda>>,
        rule_name: Symbol,
    ) -> Vec<Id> {
        let e = subst[self.e];
        let v2 = subst[self.v2];
        let v2_free_in_e = egraph[e].data.free.contains(&v2);
        if v2_free_in_e {
            let mut subst = subst.clone();
            let sym = Lambda::Symbol(format!("_{}", eclass).into());
            subst.insert(self.fresh, egraph.add(sym));
            self.if_free
                .apply_one(egraph, eclass, &subst, searcher_ast, rule_name)
        } else {
            self.if_not_free
                .apply_one(egraph, eclass, subst, searcher_ast, rule_name)
        }
    }
}

fn commutative_comonoid_example() -> Result<(), Box<dyn std::error::Error>> {
    let x = "(parallel (; (; mu sym_1_1) (parallel i id_1)) (; mu (parallel id_1 id_1 ) ) )"
        .parse::<RecExpr<CommutativeComonoids>>()?;
    let y = "(; (parallel mu mu) (parallel (; sym_1_1 (parallel i id_1)) (parallel id_1 id_1)))"
        .parse::<RecExpr<CommutativeComonoids>>()?;
    let z = "(parallel id_1 mu)".parse::<RecExpr<CommutativeComonoids>>()?;
    let mut egraph = egg::EGraph::<CommutativeComonoids, TypeAnalysis>::default();
    let x_id = egraph.add_expr(&x);
    egraph.dot().to_pdf("dot_comonoid.pdf")?;

    let log: Log = Arc::new(Mutex::new(HashMap::new()));

    let runner = Runner::default()
        .with_hook(|runner| {
            println!("Egraph is this big: {}", runner.egraph.total_size());
            Ok(())
        })
        .with_egraph(egraph)
        .run(rules_comonoid(log.clone()).iter());

    let mut egraph = runner.egraph;
    let y_id = egraph.add_expr(&y);
    let z_id = egraph.add_expr(&z);
    let x_id_canonicalised = egraph.find(x_id);

    println!("{}", x_id_canonicalised == y_id);
    println!("{}", x_id_canonicalised == z_id);

    let map = log.lock().unwrap();
    for (key, value) in map.iter() {
        println!("{}: {}", key, value);
    }

    Ok(())
}

fn main() {
    // we can do the same thing with an EGraph
    // let mut egraph: EGraph = Default::default();

    // let start : RecExpr<Lambda> = "(let g (lam x (app f x))
    //     (+ (app g 42) (app g 32)))".parse().unwrap();

    // let id = "(app (lam z (lam y (app (lam x x) y))) z) ".parse().unwrap();
    // let start = "(app (lam x x) y)".parse().unwrap();
    // let y = "y".parse().unwrap();
    // let x = "x".parse().unwrap();
    // let app_x_2 = "(app x 2)".parse().unwrap();
    // let app_y_2 = "(app y 2)".parse().unwrap();

    // let expr_1 = egraph.add_expr(&id);
    // let expr_2 = egraph.add_expr(&start);
    // let expr_3 = egraph.add_expr(&y);
    // let expr_4 = egraph.add_expr(&x);
    // let expr_5 = egraph.add_expr(&app_x_2);
    // let expr_6 = egraph.add_expr(&app_y_2);

    // egraph.union(expr_4, expr_3);
    // egraph.rebuild();

    // let runner = Runner::default().with_egraph(egraph).run(rules().iter());

    // egraph.rebuild();

    // let a = egraph.add(Lambda::Symbol(Symbol::from("x")));
    // let add = egraph.add(Lambda::Add([a,a]));
    // let lambda_add = egraph.add(Lambda::Lambda([a,add]));
    // let f = egraph.add(Lambda::Symbol(Symbol::from("f")));
    // let x42 = egraph.add(Lambda::Num(42));
    // let app = egraph.add(Lambda::App([f,x42]));
    // let b = egraph.add(Lambda::Let([f,lambda_add,app]));

    // let result = eval(&egraph,&Lambda::Let([f,add,app]));

    // match result {
    //     Some(x) => {
    //         match x.0 {
    //             Lambda::Num(n) => {println!("{}",n);},
    //             Lambda::Let(n) => {println!("Let!")}
    //             _ => {println!("Error2")}
    //         }
    //     },
    //     _ => {println!("Error")}
    // }

    // print!("{}",start.to_string());

    // let d = runner.egraph.dot();
    // d.to_pdf("dot_4.pdf").unwrap();

    // println!("{:?}",runner.stop_reason);
    let x = commutative_comonoid_example();
    match x {
        Ok(expr) => {}
        Err(err) => {
            print!("{:?}", err);
        }
    }
}
