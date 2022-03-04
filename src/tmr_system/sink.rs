use crate::tmr_system::helper::helper::get_valid_value_from_tripple_input;

pub struct Sink {
    pub input: Option<[Option<f64>; 3]>,
    pub output_vec: Vec<Option<f64>>,
}

impl Sink {
    pub fn update(&mut self) {
        self.output_vec
            .push(get_valid_value_from_tripple_input(self.input));
    }
}
