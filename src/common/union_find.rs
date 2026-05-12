pub struct UnionFind(Vec<usize>);

impl UnionFind {
    fn new(s: usize) -> Self {
        UnionFind((0..s).collect())
    }

    fn union(&mut self, a: usize, b: usize) {
        self.0[b] = a;
    }

    fn find(&self, mut a: usize) -> usize {
        while self.0[a] != a {
            a = self.0[a];
        }
        a
    }
}
