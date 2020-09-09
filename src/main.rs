use egg::*;

define_language! {
    enum Lang {
        Symbol(Symbol),
        Other(Symbol, Vec<Id>),
    }
}

fn main() {
    let n = 1000;
    let m = 10000;
    let mut egraph: EGraph<Lang, ()> = EGraph::new(());
    for i in 1..n {
        let x = format!("x_{}", i);
        let mut id = egraph.add_expr(&x.parse().unwrap());
        for j in 1..m {
           let f = format!("f_{}", j);
            id = egraph.add(Language::from_op_str(&f, vec![id]).unwrap());
        }
    }
    // egraph.dot().to_png("input.png").unwrap();
    let runner = Runner::default()
        .with_node_limit(1000_000_000)
        .with_time_limit(std::time::Duration::from_secs(60))
        .with_egraph(egraph)
        .run(&rules(n));
    // runner.egraph.dot().to_png("output.png").unwrap();
    runner.print_report();
}

fn rules(n: usize) -> Vec<Rewrite<Lang, ()>> {
    let mut rules = vec![];
    for i in 1..n {
        let r = Rewrite::new(
            format!("id_{}", i),
            format!("id_{}", i),
            format!("x_{}", i).parse::<Pattern<Lang>>().unwrap(),
            format!("x_{}", i+1).parse::<Pattern<Lang>>().unwrap(),
        ).unwrap();
        rules.push(r);
    }
    rules
}
