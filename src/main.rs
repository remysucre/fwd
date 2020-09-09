use egg::*;
use std::time::{Instant, Duration};

define_language! {
    enum Lang {
        Symbol(Symbol),
        "f" = F(Id),
    }
}

fn go(upward: bool) {
    let mut w = 100;
    loop {
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
        for i in 1..w - 1 {
            egraph.union(xs[0], xs[i]);
            if upward {
                egraph.rebuild();
            }
        }
        if !upward {
            egraph.rebuild();
        }

        let time = start.elapsed();
        println!("{}, {}", w, time.as_millis());
        w += 100;
        if time > Duration::from_secs(2) || w >= 10000{
            break
        }
    }
}

fn main() {
    go(true);
    go(false);
}
