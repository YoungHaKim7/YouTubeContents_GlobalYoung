// trait Graph<N, E> {
trait Graph {
    type N;
    type E;

    fn has_edge(&self, _: &Self::N, _: &Self::N) -> bool;
    fn edges(&self, _: &Self::N) -> Vec<Self::E>;
}

// fn distance<N, E, G: Graph<N, E>>(graph: &G, start: &N, end: &N) {}

fn distance<G: Graph>(graph: &G, start: &G::N, end: &G::N) {}

fn main() {
    println!("Hello, world!");
}
