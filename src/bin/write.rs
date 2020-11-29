extern crate graph_map;

use std::io::{BufWriter, Write};
use std::fs::File;
use graph_map::GraphMMap;

fn main() {
    println!("usage: write <source> <out>");

    let filename = std::env::args().skip(1).next().unwrap();
    let outname = std::env::args().skip(2).next().unwrap();

    println!("{} -> {}", filename, outname);
    
    let graph = GraphMMap::new(&filename);
    let mut writer = BufWriter::new(File::create(&outname).unwrap());

    let mut line = String::new();
    for node in 0 .. graph.nodes() {
        for &edge in graph.edges(node) {
            line = format!("{}\t{}\n", node, edge);
            writer.write_all(&line.into_bytes()).unwrap();
        }
    }
}
