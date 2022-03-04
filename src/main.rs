use std::collections::HashMap;
mod tmr_system;
use tmr_system::{InCorrectPipe, IsPipe, Pipe, Sink, Source, TMRSystem};

fn main() {
    let s_1 = Source {
        vec: vec![0.1, 1.0, 2.0, 3.5],
        curr: None,
        cycle: 0,
    };
    let s_2 = Source {
        vec: vec![0.1, 1.0, 2.6, 3.5],
        curr: None,
        cycle: 0,
    };
    let s_3 = Source {
        vec: vec![0.0, 1.0, 2.0, 3.5],
        curr: None,
        cycle: 0,
    };
    let sources = [s_1, s_2, s_3];


    let p1_1 = Pipe {
        input: None,
        curr: None,
    };
    let p1_2 = Pipe {
        input: None,
        curr: None,
    };
    let p1_3 = Pipe {
        input: None,
        curr: None,
    };
    let pipe_triplet_1: [Box<dyn IsPipe>; 3] = [Box::new(p1_1), Box::new(p1_2), Box::new(p1_3)];

    let p2_1 = Pipe {
        input: None,
        curr: None,
    };
    let mut error_map_2_2 = HashMap::new();
    error_map_2_2.insert(3 as usize, 4.0);
    let p2_2 = InCorrectPipe {
        input: None,
        curr: None,
        error_map: error_map_2_2,
    };
    let mut error_map_2_3 = HashMap::new();
    error_map_2_3.insert(3 as usize, 4.0);
    let p2_3 = InCorrectPipe {
        input: None,
        curr: None,
        error_map: error_map_2_3,
    };
    let pipe_triplet_2: [Box<dyn IsPipe>; 3] = [Box::new(p2_1), Box::new(p2_2), Box::new(p2_3)];

    let pipes = vec![pipe_triplet_1, pipe_triplet_2];


    let sink = Sink {
        input: None,
        output_vec: Vec::new(),
    };
    

    let mut tmr_sys = TMRSystem {
        sources,
        pipes,
        sink,
    };
    let output = tmr_sys.execute();
    println!("{:?}", output);
}
