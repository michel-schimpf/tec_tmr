use std::{collections::HashMap};

fn get_valid_value_from_tripple_input(input: Option<[Option<f64>;3]>) ->Option<f64> {
    if let Some(i) = input {
        if i[0] == i[1] || i[0] == i[2] {
            return i[0];
        } else if i[1] == i[2] {
            return i[1];
        } 
    } 
    return None;
}

fn get_max_cycles(sources: &[Source;3]) -> usize {
    let mut sources_sizes = [sources[0].vec.len(), sources[1].vec.len(), sources[2].vec.len()];
    sources_sizes.sort();
    sources_sizes[1].clone()
}

struct Source{
    vec: Vec<f64>,
    curr: Option<f64>,
    cycle: usize,
}

impl Source {
    fn update(&mut self,) {
        match self.vec.get(self.cycle){
            None => {self.curr = None},
            Some(&x) =>  { self.curr = Some(x)}
        }
    }
    fn get_output(&mut self) -> Option<f64>{
        self.curr
    }
}

trait IsPipe {
    fn update(&mut self,cycle: usize);
    fn set_input(&mut self,input: Option<[Option<f64>;3]>, );
    fn get_output(&mut self) -> Option<f64>;
}

struct Pipe {
    input: Option<[Option<f64>;3]>,
    curr: Option<f64>,
}

impl IsPipe for Pipe {
    fn update(&mut self, _cycle: usize ){
        self.curr = get_valid_value_from_tripple_input(self.input);
    }
    fn set_input(&mut self,input: Option<[Option<f64>;3]>, ) {
        self.input = input;
    }
    fn get_output(&mut self) -> Option<f64>{
        self.curr
    }
}

struct InCorrectPipe {
    input: Option<[Option<f64>;3]>,
    curr: Option<f64>,
    error_map: HashMap<usize,f64>,
}
impl IsPipe for InCorrectPipe {
    fn update(&mut self,cycle: usize ){
        match self.error_map.get(&cycle){
            None => {
                self.curr = get_valid_value_from_tripple_input(self.input);
            },
            Some(&x) => {
                self.curr = Some(x);
            }
        }
    }
    fn set_input(&mut self,input: Option<[Option<f64>;3]>, ) {
        self.input = input;
    }
    fn get_output(&mut self) -> Option<f64>{
        self.curr
    }
}
struct Sink {
    input: Option<[Option<f64>;3]>,
    output_vec: Vec<Option<f64>>,
}

impl Sink {
    fn update(&mut self){
        self.output_vec.push( get_valid_value_from_tripple_input(self.input));
    }
}


struct TMRSystem {
    sources: [Source;3],
    pipes: Vec<[Box<dyn IsPipe>;3]>,
    sink: Sink,
}

impl TMRSystem {
    fn execute(&mut self) ->  Vec<Option<f64>> {
        let max_cycles = get_max_cycles(&self.sources);
        for c in 0..max_cycles{
            let mut outputs: [Option<f64>; 3] = Default::default();
            let mut i =0;
            for mut s in &mut self.sources {
                s.cycle = c;
                s.update();
                outputs[i] = s.get_output();
                i+=1;
            }
            
            let mut pipe_outputs: [Option<f64>; 3] = outputs; //if no pipe this will not change so sink takes source outputs
            for p_triplet in &mut self.pipes {
                i =0;
                for p in p_triplet{
                    p.set_input(Some(outputs));
                    p.update(c);
                    pipe_outputs[i] = p.get_output();
                    i +=1;
                }
                outputs = pipe_outputs;
            }
            self.sink.input = Some(pipe_outputs);
            self.sink.update();

        }
        self.sink.output_vec.clone()
    }
}


fn main() {
    let s_1 = Source { vec: vec![0.1,1.0,2.0,3.5], curr: None, cycle: 0};
    let s_2 = Source { vec: vec![0.1,1.0,2.6,3.5], curr: None, cycle: 0};
    let s_3 = Source { vec: vec![0.0,1.0,2.0,3.5], curr: None, cycle: 0};
    let sources = [s_1,s_2,s_3];
    let p1_1 = Pipe{input: None, curr: None};
    let p1_2 = Pipe{input: None, curr: None};
    let p1_3 = Pipe{input: None, curr: None};
    let pipe_triplet_1:[Box<dyn IsPipe>;3] = [Box::new(p1_1),Box::new(p1_2),Box::new(p1_3)];
    let p2_1 = Pipe{input: None, curr: None};
    let mut error_map_2_2 = HashMap::new();
    error_map_2_2.insert(3 as usize, 4.0);
    let p2_2 = InCorrectPipe{input: None, curr: None, error_map: error_map_2_2};
    let mut error_map_2_3 = HashMap::new();
    error_map_2_3.insert(3 as usize, 4.0);
    let p2_3 = InCorrectPipe{input: None, curr: None, error_map: error_map_2_3};
    let pipe_triplet_2:[Box<dyn IsPipe>;3] = [Box::new(p2_1),Box::new(p2_2),Box::new(p2_3)];

    let pipes = vec![pipe_triplet_1,pipe_triplet_2];
    let sink = Sink{input: None, output_vec:Vec::new()};

    let mut tmr_sys =  TMRSystem { sources, pipes, sink };
    let output = tmr_sys.execute();
    println!("{:?}",output);

}




