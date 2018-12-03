extern crate graph_map;
use graph_map::GraphMMap;

fn main() {
    // println!("usage: print <source>");

    let filename = std::env::args().skip(1).next().unwrap();
    let graph = GraphMMap::new(&filename);
    for node in 0 .. graph.nodes() {
        println!("degree[{}]: {}", node, graph.edges(node).len());
        // for &edge in graph.edges(node) {
        //     println!("{}\t{}", node, edge);
        // }
    }
}