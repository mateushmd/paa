pub trait AllowedType: Copy  {}

impl AllowedType for u8 {}
impl AllowedType for i8 {}
impl AllowedType for u16 {}
impl AllowedType for i16 {}
impl AllowedType for u32 {}
impl AllowedType for i32 {}
impl AllowedType for u64 {}
impl AllowedType for i64 {}
impl AllowedType for u128 {}
impl AllowedType for i128 {}
impl AllowedType for usize {}
impl AllowedType for isize {}
impl AllowedType for f32 {}
impl AllowedType for f64 {}

pub struct Edge<T: AllowedType> {
    v: usize,
    w: T
}

impl<T: AllowedType> Edge<T> {
    pub fn new(v: usize, w: T) -> Self {
        Edge {
            v: v,
            w: w
        }
    }
}

impl<T: AllowedType> Clone for Edge<T> {
    fn clone(&self) -> Self {
        Edge::new(self.v, self.w) 
    }
}

pub struct Graph<T: AllowedType> {
    list: Vec<Vec<Edge<T>>>
}

impl<T: AllowedType> Graph<T> {
    pub fn new() -> Self {
        Graph { list: Vec::new() }
    }

    pub fn new_node(&mut self) -> usize {
        self.list.push(Vec::new());
        self.list.len() - 1
    }

    pub fn get_weight(&self, node: usize, neighbor: usize) -> Option<&Edge<T>> {
        self.list[node].iter().find(|e| e.v == neighbor)
    }

    pub fn add_neighbor(&mut self, node: usize, neighbor: usize, weight: T) {
        self.list[node].push(Edge::new(neighbor, weight));
    }
}

