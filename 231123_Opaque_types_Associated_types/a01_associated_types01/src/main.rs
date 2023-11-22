trait Graph<N, E> {
    fn has_edge(&self, _: &N, _: &N) -> bool;
    fn edges(&self, _: &N) -> Vec<E>;
}

/// .
fn distance<N, E, G: Graph<N, E>>(graph: &G, start: &N, end: &N) {}

fn main() {
    println!("Hello, world!");
}
