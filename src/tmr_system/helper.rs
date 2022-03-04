pub mod helper {
    pub fn get_valid_value_from_tripple_input(input: Option<[Option<f64>; 3]>) -> Option<f64> {
        if let Some(i) = input {
            if i[0] == i[1] || i[0] == i[2] {
                return i[0];
            } else if i[1] == i[2] {
                return i[1];
            }
        }
        return None;
    }
}



