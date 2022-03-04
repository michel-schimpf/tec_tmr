use std::collections::HashMap;
use crate::tmr_system::helper::helper::get_valid_value_from_tripple_input;

pub trait IsPipe {
    fn update(&mut self, cycle: usize);
    fn set_input(&mut self, input: Option<[Option<f64>; 3]>);
    fn get_output(&mut self) -> Option<f64>;
}

pub struct Pipe {
    pub input: Option<[Option<f64>; 3]>,
    pub curr: Option<f64>,
}

impl IsPipe for Pipe {
    fn update(&mut self, _cycle: usize) {
        self.curr = get_valid_value_from_tripple_input(self.input);
    }
    fn set_input(&mut self, input: Option<[Option<f64>; 3]>) {
        self.input = input;
    }
    fn get_output(&mut self) -> Option<f64> {
        self.curr
    }
}

pub struct InCorrectPipe {
    pub input: Option<[Option<f64>; 3]>,
    pub curr: Option<f64>,
    pub error_map: HashMap<usize, f64>,
}
impl IsPipe for InCorrectPipe {
    fn update(&mut self, cycle: usize) {
        match self.error_map.get(&cycle) {
            None => {
                self.curr = get_valid_value_from_tripple_input(self.input);
            }
            Some(&x) => {
                self.curr = Some(x);
            }
        }
    }
    fn set_input(&mut self, input: Option<[Option<f64>; 3]>) {
        self.input = input;
    }
    fn get_output(&mut self) -> Option<f64> {
        self.curr
    }
}
