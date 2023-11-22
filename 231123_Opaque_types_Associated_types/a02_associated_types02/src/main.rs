use std::fmt::Display;

trait Graph {
    type N: Display;
    type E;

    fn has_edge(&self, _: &Self::N, _: &Self::N) -> bool;
    fn edges(&self, _: &Self::N) -> Vec<Self::E>;
}

fn distance<G: Graph>(graph: &G, start: &G::N, end: &G::N) {}

fn main() {
    println!("Hello, world!");
}
