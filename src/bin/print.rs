extern crate graph_map;
use graph_map::GraphMMap;

fn main() {
    // println!("usage: print <source>");

    let filename = std::env::args().skip(1).next().unwrap();
    let graph = GraphMMap::new(&filename);

    use std::io::Write;
    let mut writer = std::io::BufWriter::new(std::io::stdout());

    for node in 0 .. graph.nodes() {
        for &edge in graph.edges(node) {
            write!(&mut writer, "{}\t{}\n", node, edge).expect("Write failed");
        }
    }

    std::io::stdout().flush().expect("Flush failed");
}
