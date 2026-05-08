#[derive(Debug)]
pub struct Interval {
    pub s: isize,
    pub f: isize,
}

impl Default for Interval {
    fn default() -> Self {
        Self { s: 0, f: 0} 
    }
}

impl Clone for Interval {
    fn clone(&self) -> Self {
        Self { s: self.s, f: self.f } 
    }
}
