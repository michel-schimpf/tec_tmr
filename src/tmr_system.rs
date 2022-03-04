pub use pipes::{Pipe,IsPipe,InCorrectPipe};
mod pipes;
mod helper;
pub use source::Source;
mod source;
mod sink;
pub use sink::Sink;


pub struct TMRSystem {
    pub sources: [Source; 3],
    pub pipes: Vec<[Box<dyn IsPipe>; 3]>,
    pub sink: Sink,
}

impl TMRSystem {
    pub fn execute(&mut self) -> Vec<Option<f64>> {
        let max_cycles = get_max_cycles(&self.sources);
        for c in 0..max_cycles {
            let mut outputs: [Option<f64>; 3] = Default::default();
            let mut i = 0;
            for mut s in &mut self.sources {
                s.cycle = c;
                s.update();
                outputs[i] = s.get_output();
                i += 1;
            }

            let mut pipe_outputs: [Option<f64>; 3] = outputs; //if no pipe this will not change so sink takes source outputs
            for p_triplet in &mut self.pipes {
                i = 0;
                for p in p_triplet {
                    p.set_input(Some(outputs));
                    p.update(c);
                    pipe_outputs[i] = p.get_output();
                    i += 1;
                }
                outputs = pipe_outputs;
            }
            self.sink.input = Some(pipe_outputs);
            self.sink.update();
        }
        self.sink.output_vec.clone()
    }
}


fn get_max_cycles(sources: &[Source; 3]) -> usize {
    let mut sources_sizes = [
        sources[0].vec.len(),
        sources[1].vec.len(),
        sources[2].vec.len(),
    ];
    sources_sizes.sort();
    sources_sizes[1].clone()
}