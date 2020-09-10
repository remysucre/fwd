use egg::*;
use std::time::{Instant, Duration};

define_language! {
    enum Lang {
        Symbol(Symbol),
        "f" = F(Id),
    }
}

fn go(upward: bool) {
    let mut w = 3000;
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
        let mut rbt = 0;
        for i in 1..w - 1 {
            egraph.union(xs[i-1], xs[i]);
            if upward {
                let rs = Instant::now();
                egraph.rebuild();
                // println!("{}", rs.elapsed().as_nanos());
                rbt += rs.elapsed().as_nanos();
            }
        }
        // println!("{}", rbt / (w-2) as u128 );
        if !upward {
            egraph.rebuild();
        }

        let time = start.elapsed();
        println!("{}, {}", w, time.as_millis());
        w += 1000;
        if time > Duration::from_secs(1) || w >= 200000 {
            break
        }
        // println!("hashcons: {}", egraph.total_size());
        // println!("nodes: {}", egraph.total_number_of_nodes());
        // println!("classes: {}", egraph.number_of_classes());
        // break
    }
}

fn main() {
    go(true);
    go(false);
}
