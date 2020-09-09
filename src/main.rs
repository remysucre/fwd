use egg::*;
use std::time::Instant;

define_language! {
    enum Lang {
        Symbol(Symbol),
        "f" = F(Id),
    }
}

fn main() {
    let w = 2000;
    let d = 100;
    let mut egraph: EGraph<Lang, ()> = EGraph::new(());
    let mut xs = vec![];
    for i in 1..w {
        let x = format!("x_{}", i);
        let mut id = egraph.add_expr(&x.parse().unwrap());
        xs.push(id);
        for _j in 1..d {
            // let f = format!("f_{}", j);
            // id = egraph.add(Language::from_op_str(&f, vec![id]).unwrap());
            id = egraph.add(Language::from_op_str("f", vec![id]).unwrap());
        }
    }

    let start = Instant::now();
    for i in 1..w-1 {
        egraph.union(xs[0], xs[i]);
        // egraph.rebuild();
    }
    egraph.rebuild();
    println!("{}", start.elapsed().as_millis());
}
