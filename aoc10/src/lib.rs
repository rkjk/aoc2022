use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::io;
use std::io::Write;

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

#[derive(Debug, Copy, Clone)]
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
    x: i32,
    crt: Vec<Vec<bool>>,
}

impl Session {
    pub fn new() -> Self {

        Session {
            states: vec![State::new(0, 1)],
            x: 1,
            crt: vec![vec![false; 40]; 6],
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

    fn get_part2(&mut self) {
        let get_coords = |cycle_num: usize| -> (usize, usize) {
            ((cycle_num - 1) / 40 as usize, ((cycle_num - 1) % 40) as usize)
        };
        let get_x_coords = |x: i32| -> i32 {
            match x <= 0 {
                true => x,
                false => x % 40
            }
        };
        self.crt[0][0] = true;
        self.crt[0][1] = true;
        let mut cur_cycle = self.states[1].get_cycle();
        let mut cur_x = self.states[1].get_x();
        let mut i = 0;
        for state in self.states[1..].iter() {
            let State {cycle_number: nex_cycle, x: nex_x} = state;
            for cyc in cur_cycle+1..*nex_cycle+1 {
                let (crt_row_num, crt_col_num) = get_coords(cyc);
                let x_col_num = get_x_coords(cur_x);
                for v in [x_col_num - 1, x_col_num, x_col_num + 1].into_iter() {
                    if v < 0 {
                        continue;
                    }
                    if crt_col_num == v as usize {
                        self.crt[crt_row_num][crt_col_num] = true;
                        break;
                    }
                }
            }
            cur_cycle = *nex_cycle;
            cur_x = *nex_x;
        }
    }

    pub fn print_crt(&self) {
        for i in 0..6 {
            for j in 0..40 {
                match self.crt[i][j] {
                    false => print!("."),
                    true => print!("#"),
                };
                io::stdout().flush().unwrap();
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn it_works() {
        let inp = parse_input(read_input("in.test").unwrap());
        let mut session = Session::new();
        session.run_through(&inp);
        //session.print_states();
        println!("test 1: {}", session.get_part1());
        session.get_part2();
        session.print_crt();
    }

    #[test]
    fn actual() {
        use std::time::Instant;
        let now = Instant::now();
        let inp = parse_input(read_input("in.1").unwrap());
        let mut session = Session::new();
        session.run_through(&inp);
        //session.print_states();
        session.get_part2();
        let elapsed = now.elapsed();
        println!("Part 1: {}", session.get_part1());
        session.print_crt();
        println!("Elapsed: {:.2?}", elapsed);
    }
}
