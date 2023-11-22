#[derive(Debug)]
struct Node;

struct Edge;

struct MyGraph;

trait Graph {
    type N;
    type E;

    fn has_edge(&self, _: &Self::N, _: &Self::N) -> bool;
    fn edges(&self, _: &Self::N) -> Vec<Self::E>;
}

impl Graph for MyGraph {
    type N = Node;
    type E = Edge;

    fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
        true
    }

    fn edges(&self, n: &Node) -> Vec<Edge> {
        Vec::new()
    }
}

fn main() {
    println!("Hello, world!");
}
