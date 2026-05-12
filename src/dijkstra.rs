use crate::{
    common::Graph,
    input
};

pub fn solve() {
    let n = input!("number of nodes: ", usize);

    let mut graph = Graph::<isize>::new();

    for _ in 0..n {
        let nn = input!("number of neighbors: ", usize);

        for j in 0..nn {
            let v = input!(format!("neighbor {j}: "), usize);
            let w = input!(format!("weight of neibor {j}: "), isize);
            graph.add_neighbor(j, v, w);
        }
    }

    let u = input!("starting node: ", usize);
    let v = input!("finishing node: ", usize);

    
}
