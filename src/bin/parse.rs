extern crate rand;

use std::io::{BufRead, BufReader, BufWriter, Write};
use std::fs::File;
use std::slice;
use std::mem;

use std::cmp::{min, max};

use rand::Rng;

fn main() {
    println!("usage: parse <source> <target> [sort|dedup|reorder|undirect|randomize]^*");
    println!("takes sorted input whose lines look like:");
    println!("<src> <dst>");
    println!("will overwrite <target>.offsets and <target>.targets");
    let source = std::env::args().nth(1).unwrap();
    let target = std::env::args().nth(2).unwrap();

    let mut graph = read_edges(&source);
    for instruction in std::env::args().skip(3) {
        match instruction.as_str() {
            "sort" => graph.sort(),                 // sorts the edges by (src, dst).
            "dedup" => graph.dedup(),               // deduplicates edges.
            "deloop" => graph.retain(|&(x,y)| x != y),  // removes self-loops.
            "reorder" => reorder(&mut graph),       // renumbers nodes by undirected degree.
            "undirect" => undirect(&mut graph),     // removes edge direction.
            "randomize" => randomize(&mut graph),   // renumbers nodes randomly.
            unknown => {
                panic!(format!("Unrecognized command: {}", unknown));
            },
        }
    }

    graph.sort();
    graph.dedup();
    digest_graph_vector(&_extract_fragment(graph.iter().map(|x| *x)), &target);
}

/// Reads lines of text into pairs of integers.
fn read_edges(filename: &str) -> Vec<(u32, u32)> {
    let mut graph = Vec::new();
    let file = BufReader::new(File::open(filename).unwrap());
    for readline in file.lines() {
        let line = readline.ok().expect("read error");
        if !line.starts_with('#') {
            let mut elts = line[..].split_whitespace();
            let src: u32 = elts.next().expect("line missing src field").parse().expect("malformed src");
            let dst: u32 = elts.next().expect("line missing dst field").parse().expect("malformed dst");
            graph.push((src, dst));
        }
    }
    graph
}

/// Re-orders graph identifiers by undirected degree.
fn reorder(graph: &mut Vec<(u32, u32)>) {

    let mut degrees = Vec::new();
    for &(src, dst) in graph.iter() {
        while degrees.len() <= src as usize { degrees.push(0); }
        while degrees.len() <= dst as usize { degrees.push(0); }
        degrees[src as usize] += 1;
        degrees[dst as usize] += 1;
    }

    let mut to_sort =
        degrees
            .drain(..)
            .enumerate()
            .filter(|&(_,deg)| deg > 0)
            .map(|(pos,deg)| (deg,pos as u32))
            .collect::<Vec<_>>();

    to_sort.sort_unstable();
    let mut rename = Vec::new();
    for (idx, (_deg, pos)) in to_sort.into_iter().enumerate() {
        while rename.len() <= pos as usize { rename.push(0); }
        rename[pos as usize] = idx as u32;
    }

    for index in 0 .. graph.len() {
        let new_src = rename[graph[index].0 as usize];
        let new_dst = rename[graph[index].1 as usize];
        graph[index] = (min(new_src, new_dst), max(new_src, new_dst));
    }
}

/// Re-orients edges to point from small to large.
fn undirect(graph: &mut Vec<(u32, u32)>) {
    for src_dst in graph.iter_mut() {
        if src_dst.0 > src_dst.1 {
            ::std::mem::swap(&mut src_dst.0, &mut src_dst.1);
        }
    }
}

fn randomize(graph: &mut Vec<(u32, u32)>) {

    let mut rng = ::rand::thread_rng();

    let mut random = Vec::new();
    for index in 0 .. graph.len() {
        while random.len() <= graph[index].0 as usize { random.push(rng.gen::<usize>()); }
        while random.len() <= graph[index].1 as usize { random.push(rng.gen::<usize>()); }
    }

    let mut to_sort =
        random
            .drain(..)
            .enumerate()
            .map(|(pos,deg)| (deg,pos as u32))
            .collect::<Vec<_>>();

    to_sort.sort_unstable();
    let mut rename = Vec::new();
    for (idx, (_deg, pos)) in to_sort.into_iter().enumerate() {
        while rename.len() <= pos as usize { rename.push(0); }
        rename[pos as usize] = idx as u32;
    }

    for index in 0 .. graph.len() {
        graph[index].0 = rename[graph[index].0 as usize];
        graph[index].1 = rename[graph[index].1 as usize];
    }
}

fn _extract_fragment<I: Iterator<Item=(u32, u32)>>(graph: I) -> (Vec<u64>, Vec<u32>) {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for (src, dst) in graph {
        while src + 1 >= nodes.len() as u32 { nodes.push(0); }
        while dst + 1 >= nodes.len() as u32 { nodes.push(0); }

        nodes[src as usize + 1] += 1;
        edges.push(dst);
    }

    for index in 1..nodes.len() {
        nodes[index] += nodes[index - 1];
    }

    return (nodes, edges);
}

fn digest_graph_vector(graph: &(Vec<u64>, Vec<u32>), output_prefix: &str) {
    let mut edge_writer = BufWriter::new(File::create(format!("{}.targets", output_prefix)).unwrap());
    let mut node_writer = BufWriter::new(File::create(format!("{}.offsets", output_prefix)).unwrap());
    node_writer.write_all(unsafe { _typed_as_byte_slice(&graph.0[..]) }).unwrap();

    let mut slice = unsafe { _typed_as_byte_slice(&graph.1[..]) };
    while slice.len() > 0 {
        let to_write = if slice.len() < 1000000 { slice.len() } else { 1000000 };
        edge_writer.write_all(&slice[..to_write]).unwrap();
        slice = &slice[to_write..];
    }
}

unsafe fn _typed_as_byte_slice<T>(slice: &[T]) -> &[u8] {
    slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len() * mem::size_of::<T>())
}
