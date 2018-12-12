# graph-map

A library for working with mmap'd graph data.

## Usage

**Prepare**

Given a plain-text representation of a graph, such as the following
from
[snap.stanford.edu/data/soc-LiveJournal1.html](https://snap.stanford.edu/data/soc-LiveJournal1.html)

```
# Directed graph (each unordered pair of nodes is saved once): soc-LiveJournal1.txt 
# Directed LiveJournal friednship social network
# Nodes: 4847571 Edges: 68993773
# FromNodeId    ToNodeId
0       1
0       2
0       3
0       4
0       5
```

run

``` shell
cargo run --release --bin parse <your_graph.txt> <out>
```

This will create two files `out.targets` and `out.offsets` containing
binary representations of the graph's edges and nodes respectively.

**Read**

After the above steps you should end up with something like the
following:

```
+ ~/data/
  + your_graph.offsets
  + your_graph.targets
```

Reading the graph is a simple matter of

``` rust
extern crate graph_map;
use graph_map::GraphMMap;

fn main () {

    // ! Note that we simply drop the 'targets' / 'offsets' prefix
    let graph = GraphMMap::new("~/data/your_graph");
    
    for node in 0 .. graph.nodes() {
        for &edge in graph.edges(node) {
            println!("{}\t{}", node, edge);
        }
    }
}
```

