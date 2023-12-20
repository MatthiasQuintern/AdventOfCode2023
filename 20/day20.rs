use std::io::{self, BufRead};
use std::convert::TryInto;
// use num::Integer;
use std::collections::{HashMap, VecDeque};

type Pulse = u8;
const LOW: Pulse = 0;
const HIGH: Pulse = 1;

trait Module {
    fn get_pulse(&mut self, from: u16, pulse: Pulse) -> Option<Pulse>;
    fn add_input(&mut self, input: u16) {}
    fn get_connections(&self) -> &Vec<u16>;
}
#[derive(Clone)]
struct FlipFlop {
    name: u16,
    modules: Vec<u16>,
    state: Pulse,
}
impl Module for FlipFlop {
    fn get_pulse(&mut self, from: u16, pulse: Pulse) -> Option<Pulse> {
        if pulse == LOW {
            self.state ^= HIGH;
            return Some(self.state);
        }
        None
    }
    fn get_connections(&self) -> &Vec<u16> { return &self.modules; }
}

#[derive(Clone)]
struct Conjunction {
    name: u16,
    modules: Vec<u16>,
    state: Pulse,
    last_inputs: HashMap<u16, Pulse>,
}
impl Module for Conjunction {
    fn get_pulse(&mut self, from: u16, pulse: Pulse) -> Option<Pulse> {
        self.last_inputs.iter().for_each(|(k, v)| print!("{}: {},", k, v));
        println!();
        if !self.last_inputs.contains_key(&from) {
            panic!("Conjunction got unexpected impulse from {}", from);
        }
        *self.last_inputs.get_mut(&from).unwrap() = pulse;
        if self.last_inputs.iter().all(|(k,v)| *v == HIGH) { self.state = LOW; }
        else { self.state = HIGH; }
        return Some(self.state)
    }
    fn add_input(&mut self, input: u16) {
        self.last_inputs.insert(input, LOW);
    }
    fn get_connections(&self) -> &Vec<u16> { return &self.modules; }
}

#[derive(Clone)]
struct Start {
    name: u16,
    modules: Vec<u16>,
    state: Pulse,
}
impl Module for Start {
    fn get_pulse(&mut self, from: u16, pulse: Pulse) -> Option<Pulse> {
        self.state = pulse;
        return Some(self.state)
    }
    fn get_connections(&self) -> &Vec<u16> { return &self.modules; }
}


fn make_index(s: &str) -> u16 {
    println!("{}", s);
    if s.len() == 2 {
        let mut t: u16 = 0; 
        s.as_bytes().iter().enumerate().for_each(|(i, c)| t |= (*c as u16) << i*8);
        return t;
    }
    return 0
}
fn from_index(i: u16) -> char {
    if i == 0 { return 'B' }
    let mut c: u8 = i as u8;
    c |= i.overflowing_shr(8).0 as u8;
    return c as char
}

fn main() {
    let input = "input.txt";
    // let input = "example.txt";
    // let input = "example2.txt";
    let mut lines = read_lines(&input);
    let mut modules = HashMap::<u16, Box<dyn Module>>::new();
    for line in lines.map(|r| r.ok().unwrap()) {
        let dash = line.find('-').unwrap();
        let module = make_index(&line[1..dash-1]);
        println!("{module}");
        let connected_modules = line[dash+2..].split(',').map(|s| make_index(&s[1..])).collect();
        let _ = match line.chars().next().unwrap() {
            '&' => modules.insert(module, Box::new(Conjunction{name:module, modules: connected_modules, state: LOW, last_inputs: HashMap::new()})),
            '%' => modules.insert(module, Box::new(FlipFlop{name:module, modules: connected_modules, state: LOW})),
            _   => modules.insert(module, Box::new(Start{name:module, modules: connected_modules, state: LOW})),
        };
    }
    let keys = modules.keys().map(|k| *k).collect::<Vec<u16>>().clone();
    for n in keys.iter() {
        let connected_modules = modules[n].get_connections().clone().iter().filter(|c| modules.contains_key(c)).map(|c| *c).collect::<Vec<u16>>();
        for c in connected_modules.iter() {
            let m = modules.get_mut(c).unwrap();
            m.add_input(*n);
        }
    }
    let mut modules2 = modules.clone();
    let mut queue: VecDeque::<(u16, Pulse)> = VecDeque::new();
    let mut lows: usize = 0;
    let mut highs: usize = 0;
    for i in 0..1000 {
        println!("{i}");
        queue.push_back((0, LOW));
        lows += 1;
        while !queue.is_empty() {
            let (from, pulse) = queue.pop_front().unwrap();
            if !modules.contains_key(&from) { continue; }
            for m in modules[&from].get_connections().clone().iter() {
                println!("{:6} -{} -> {}", from_index(from), pulse, from_index(*m));
                if pulse == LOW { lows += 1 } else { highs += 1 }
                if !modules.contains_key(m) { continue; }
                if let Some(new_pulse) = modules.get_mut(&m).unwrap().get_pulse(from, pulse) {
                    queue.push_back((*m, new_pulse));
                }
            }
        }
    }
    println!("lows: {}, highs: {}", lows, highs);
    println!("lows * highs (1): {}", lows * highs);

}


fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<std::fs::File>>
where P: AsRef<std::path::Path>, {
    return match std::fs::File::open(filename) {
        Err(why) => panic!("Could not open file. {}", why),
        Ok(file) => std::io::BufReader::new(file).lines()
    };
}
