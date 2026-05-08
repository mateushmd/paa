use ::std::collections::{ hash_set, HashSet };

pub struct Graph {
    list: Vec<HashSet<usize>>
}

impl Graph {
    pub fn new(node_count: usize) -> Self {
        Graph { list: vec![HashSet::with_capacity(node_count); node_count] }
    }

    pub fn new_node(&mut self) -> usize {
        self.list.push(HashSet::with_capacity(self.list.len()));
        self.list.len() - 1
    }

    pub fn is_neighbor(&self, node: usize, neighbor: usize) -> bool {
        self.list[node].contains(&neighbor)
    }

    pub fn add_neighbor(&mut self, node: usize, neighbor: usize) -> bool {
        self.list[node].insert(neighbor)
    }
}
