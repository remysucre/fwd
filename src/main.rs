use egg::*;
use std::time::Instant;

define_language! {
    enum Lang {
        Symbol(Symbol),
        Other(Symbol, Vec<Id>),
    }
}

fn main() {
    let mut xs = vec![];
    for &w in &[2000, 4000, 6000, 8000, 10000] {
        let mut egraph: EGraph<Lang, ()> = EGraph::new(());
        for i in 1..w {
            let x = format!("x_{}", i);
            let id = egraph.add_expr(&x.parse().unwrap());
            xs.push(id);
            let f = format!("f_{}", i);
            egraph.add(Language::from_op_str(&f, vec![id]).unwrap());
        }

        let start = Instant::now();
        for i in 1..w - 1 {
            egraph.union(xs[0], xs[i]);
            // egraph.rebuild();
        }
        egraph.rebuild();
        println!("{}", start.elapsed().as_millis());
    }
}

fn rules(n: usize) -> Vec<Rewrite<Lang, ()>> {
    let mut rules = vec![];
    for i in 1..n - 1 {
        let r = Rewrite::new(
            format!("id_{}", i),
            format!("id_{}", i),
            "x_1".parse::<Pattern<Lang>>().unwrap(),
            format!("x_{}", i + 1).parse::<Pattern<Lang>>().unwrap(),
        )
        .unwrap();
        rules.push(r);
    }
    rules
}
