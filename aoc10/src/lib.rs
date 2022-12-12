use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::HashMap;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

struct Data {
    num_cycles: usize,
    val: i32
}

impl Data {
    pub fn new(num_cycles: usize, val: i32) -> Self {
        Data {
            num_cycles: num_cycles,
            val: val
        }
    }

    pub fn get_val(&self) -> i32 {
        self.val
    }
}

enum Op {
    Noop(Data),
    Addx(Data)
}

impl Op {
    pub fn new(val: String) -> Self {
        let v: Vec<&str> = val.split(" ").collect();
        if v.len() == 1 {
            return Op::Noop(Data::new(1, 0));
        }
        Op::Addx(Data::new(2, v[1].parse::<i32>().unwrap()))
    }
}


fn parse_input(inp: Vec<String>) -> Vec<Op> {
    inp.into_iter().map(|val| Op::new(val)).collect()
}

#[derive(Debug)]
struct State {
    cycle_number: usize,
    x: i32
}

impl State {
    pub fn new(cycle_number: usize, x: i32) -> Self {
        State {
            cycle_number: cycle_number,
            x: x
        }
    }

    pub fn get_cycle(&self) -> usize {
        self.cycle_number
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_signal_strength(&self) -> i32 {
        self.cycle_number as i32 * self.x
    }
}

struct Session {
    states: Vec<State>,
    x: i32
}

impl Session {
    pub fn new() -> Self {
        Session {
            states: vec![State::new(0, 1)],
            x: 1
        }
    }

    pub fn run_through(&mut self, instructions: &Vec<Op>) {
        for instr in instructions {
            let State {cycle_number: cur_cycle, x: cur_x} = self.states[self.states.len() - 1];
            match instr {
                Op::Noop(_) => self.states.push(State::new(cur_cycle + 1, cur_x)),
                Op::Addx(v) => self.states.push(State::new(cur_cycle + 2, cur_x + v.get_val()))
            }
        }
    }

    pub fn print_states(&self) {
        println!("States: {:?}", self.states);
    }

    pub fn get_part1(&self) -> i32 {
        let mut res = vec![0];
        for val in self.states.iter() {
            if val.get_cycle() /  20 == res.len() {
                res.push(0);
            }
            let ind = res.len() - 1;
            res[ind] = val.get_x();
        }
        let mut product = 0;
        let mut coeff = 0;
        for (i, val) in res.into_iter().enumerate() {
            coeff += 20;
            if coeff > 220 {
                break;
            }
            if coeff % 40 == 0 {
                continue;
            }
            product += coeff * val;
        }
        product
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let inp = parse_input(read_input("in.test").unwrap());
        let mut session = Session::new();
        session.run_through(&inp);
        println!("test 1: {}", session.get_part1());
    }

    #[test]
    fn actual() {
        let inp = parse_input(read_input("in.1").unwrap());
        let mut session = Session::new();
        session.run_through(&inp);
        println!("Part 1: {}", session.get_part1());
    }
}
