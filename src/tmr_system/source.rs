pub struct Source {
    pub vec: Vec<f64>,
    pub curr: Option<f64>,
    pub cycle: usize,
}

impl Source {
    pub fn update(&mut self) {
        match self.vec.get(self.cycle) {
            None => self.curr = None,
            Some(&x) => self.curr = Some(x),
        }
    }
    pub fn get_output(&mut self) -> Option<f64> {
        self.curr
    }
}