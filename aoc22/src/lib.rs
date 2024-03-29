use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::{HashSet, VecDeque};
use std::cmp::min;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

#[derive(Debug, Copy, Clone)]
struct Offset {
    pub x: i32,
    pub y: i32
}

impl Offset {
    pub fn new(x: i32, y: i32) -> Self {
        Offset {
            x: x,
            y: y
        }
    }

    pub fn turn_left(&mut self) {
        let (x, y) = (self.x, self.y);
        self.x = y;
        self.y = x;
        self.x *= -1;
    }

    pub fn turn_right(&mut self) {
        let (x, y) = (self.x, self.y);
        self.x = y;
        self.y = x;
        self.y *= -1;
    }

    pub fn get_facing(&self) -> usize {
        match (self.x, self.y) {
            (0, 1) => 0,
            (1, 0) => 1,
            (0, -1) => 2,
            (-1, 0) => 3,
            _ => panic!("unknown direction"),
        }
    }
}


type Instr = (usize, Offset);

fn get_instr(inp: &str) -> Vec<Instr> {
    let mut dir = Offset::new(0, 1); // Pointing right in the beginning
    let mut instr = Vec::new();
    let mut last_seen = 0;
    for (ind, matched) in inp.match_indices(|c: char| c == 'L' || c == 'R') {
        let num = inp[last_seen..ind].parse::<usize>().unwrap();
        instr.push((num, dir));
        match matched {
            "L" => dir.turn_left(),
            "R" => dir.turn_right(),
            _ => panic!("unexpected"),
        };
        last_seen = ind + 1;
    }
    instr.push((inp[last_seen..].parse::<usize>().unwrap(), dir));
    instr
}

#[derive(Debug, Copy, Clone)]
struct Tmp {
    first: Option<usize>,
    last: Option<usize>,
}

impl Tmp {
    pub fn new() -> Self {
        Tmp {
            first: None,
            last: None,
        }
    }

    pub fn fill(&mut self, val: usize) {
        if self.first.is_none() {
            self.first = Some(val);
            
        }
        self.last = Some(val);
    }
}

type Coord = (usize, usize);

#[derive(Debug)]
struct Line {
    pub first: usize,
    pub last: usize,
}

impl Line {
    pub fn new(first: usize, last: usize) -> Self {
        Line {
            first: first,
            last: last
        }
    }
}

fn parse_input(inp: Vec<String>) -> (Vec<Line>, Vec<Line>, HashSet<Coord>){
    let (rmax, cmax) = (inp.len(), inp.iter().map(|t| t.len()).max().unwrap());
    //println!("Max row: {}, Max col: {}", rmax, cmax);
    let mut row_limits = vec![Tmp::new(); rmax];
    let mut col_limits = vec![Tmp::new(); cmax];
    let mut walls = HashSet::new();
    for (i, line) in inp.into_iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    walls.insert((i, j));
                    row_limits[i].fill(j);
                    col_limits[j].fill(i);
                },
                '.' => {
                    row_limits[i].fill(j);
                    col_limits[j].fill(i);
                },
                _ => (),
            }
        }
    }
    /*
    for (i, r) in row_limits.iter().enumerate() {
        if r.first.is_none() || r.last.is_none() {
            println!("row: {}, {:?}", i, r);
        }
    }
    for (i, c) in col_limits.iter().enumerate() {
        if c.first.is_none() || c.last.is_none() {
            //println!("col: {}, {:?}", i, c);
        }
    }
    */
    let rows = row_limits.into_iter().map(|v| Line::new(v.first.unwrap(), v.last.unwrap())).collect();
    let cols = col_limits.into_iter().map(|v| Line::new(v.first.unwrap(), v.last.unwrap())).collect();
    (rows, cols, walls)
}

enum Op {
    Continue,
    Stop
}

struct Session {
    rows: Vec<Line>,
    cols: Vec<Line>,
    walls: HashSet<Coord>,
    instrs: Vec<Instr>,
    cur_pos: Coord
}

impl Session {
    pub fn new(rows: Vec<Line>, cols: Vec<Line>, walls: HashSet<Coord>, instrs: Vec<Instr>) -> Self {
        let cur_pos = (0, rows[0].first);
        Session {
            rows: rows,
            cols: cols,
            walls: walls,
            instrs: instrs,
            cur_pos: cur_pos
        }
    }

    fn process_instruction(&mut self, offset: &Offset) -> Op {
        let (mut r, mut c): (i32, i32) = (
            self.cur_pos.0.try_into().unwrap(),
            self.cur_pos.1.try_into().unwrap());
        let (ru, cu) = self.cur_pos;
        if offset.x == 0 {
            c += offset.y;
            let (rf, rl): (i32, i32) = (
                self.rows[ru].first.try_into().unwrap(),
                self.rows[ru].last.try_into().unwrap()
            );
            if c > rl {
                c = rf;
            } else if c < rf {
                c = rl;
            } else {}
        } else {
            r += offset.x;
            let (cf, cl): (i32, i32) = (
                self.cols[cu].first.try_into().unwrap(),
                self.cols[cu].last.try_into().unwrap()
            );
            if r > cl {
                r = cf;
            } else if r < cf {
                r = cl;
            } else {}
        }
        let np: Coord = (r.try_into().unwrap(), c.try_into().unwrap());
        match self.walls.contains(&np) {
            true => Op::Stop,
            false => {
                self.cur_pos = np;
                Op::Continue
            }
        }
    }

    pub fn process_part1(&mut self) -> usize {
        let instrs = self.instrs.clone();
        for instr in instrs.iter() {
            let (num, offset) = instr;
            for _ in 0..*num {
                match self.process_instruction(&offset) {
                    Op::Continue => (),
                    Op::Stop => break,
                }
            }
        }
        let facing = self.instrs[self.instrs.len() - 1].1.get_facing();
        let (r, c) = self.cur_pos;
        //println!("final row: {}, final col: {}, facing: {}", r, c, facing);
        1000 * (r + 1) + 4 * (c + 1) + facing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (rows, cols, walls) = parse_input(read_input("in.test").unwrap());
        let steps_t = "10R5L5R10L4R5L5";
        let steps = get_instr(steps_t);
        let mut session = Session::new(rows, cols, walls, steps);
        let part1 = session.process_part1();
        println!("Test 1: {}", part1);
    }

    #[test]
    fn actual() {
        let (rows, cols, walls) = parse_input(read_input("in.1").unwrap());
        let steps_t = "3L27L4R39L42R22L2L20R47L27R11R11R19R21R36L1L40L1R27L32L7R16L23R44R8R15L22L22R22R18L44R48R27L5R32R9L38R23R48L47R25L25R27R36R7L26L13L36R37R21R3R22L38L9L43R15L50R18L19R48R6R19L30R4R33L48L42L21R45R4R9R5R4L38R41R9R37R44R39R1L8R49L24L26R33L16L22R34R39L14L28R32L23L13R19L32R34L8L35R15R29L17L3L46L26L37L48R22L6L37R16R46R36L36R22R3R12R24L17R21R26R33R16R5R44R11R2L31R23R26R25R36R27R1R1L12R39L21R49R44L41L35R31L39L3R5R14L26R30R47R6R21L34R11L42R2L20R37L28L25R47R44L9R50R49L16L33R6R7L33L45L38R24R29R29L26L36R9R16L47R27R19L35L26L24L12L50L47L11L22R39L35L12L23L31L20R32L46L46R17L35R26R35L41L30L18R7R23L9R12R21R36R45L45R41R7R29L3R22L27L35L10L11R15L39L25L19L43L18L50R24L12R7L3R28R10L32L19R11L2R6R25R43R22R8R20L31R17R45L47L48L3R20L26L15R8L30L9L34L8R35L37R30R42L31L17L19L20L9R46R5L2L48L5L15R4R13L4L23R44R36R12L12R11L29R4L37L6R12R38R24R6R1L3R50L2L8L26R24R38R24L43L42R48L35R27R10L21L34L11R48R39L4R29R28R42L35R38L8R22R23R50L5R6L25L20L6L18R48R20L33R1R13L39L7L30L48R49R14L35R38R15R25R45L18L30L2L28R25L29L4L35L25L38L33R48R37R4R29L30L13L6R23L9L37L3R17R1R13L10R22R27R14R13L32L3R6L34R49L37L47L10L1L36L27R48R15L45R23L30L47R1L15R4R30R29R40L24L37L30L28R27R36R5R15L5L6R1R15R4R23L39R38R24L46L40R16L32R4R43L17R18R27R29R24L11R38L13L31R48L5R23R4R9R35L25R12R32L39L7L22L14L14R28L1R16L39R15R2R26L34L3R18R29L46R31R47R23L31R28R20R24L18R6R18R33R49L24R11L29L6L44L22L26L1R5L1L45R29L17L31L22R46R39R7L28R4R28L13L18L50R3L17R19R43R16R26R34L4R18R29L41L33R46R16L34L23L34R22L43R22L42R1R23R37R18R36R48L18R3R22R6R27L24L47R44R19R12L41L7R23L5R39R1R1L10R17R19L8L49R38R6R3L39R1R42L21L27L25L32R18L14L5R43L13R13R26R36R44R37L23R17R41L41L22L21L41L26L20L14R22L25R42R28R16L33R2R28R50R47L21L24L5L25R4L46L34R1L2L14R7R10L34R28L34R20R35L43R6L5R32R34R16L22L45R16R9R17R19R46L46L49L22L28L29L46R2R38R2R23R47R36L45R10L46R42R9L23L38L5L50L31L14L17R6L27R28L49R45R50L30R23R3L36L10L36L21R36L25R37R40R21R32R27L3R40R28R49L10L23R36L18R21L18L36R30L44R19R15R18L22L18L13R3R43L18L2L3L46L21L6L25L19R16R43R14L11R41R27R42L30R6L34R9L14R34L49R35R21R41R4R13L3R15L38L19R37L31L9R1R9L19L6L15L9L35R27L27L27L28R26R40L41L6L3L20R39L35R9R19R14L38L13R32L12R26L27R27R42L30R5L24R32L35L3R39R42R2L35L11R33L32R6L31L33L1L9L27L1L12L25R34R13R5L34R46R44R28R29R47R47R41R20L8R40L12R39R46R34L47L42L49L5L7L25R21R20L19L2L14L43R2L15R4R48L48R5L36R36R5L46R23L32R26L29R23L6L50L33R12R27R23L22L2R33R48L7R49L5L10R26L16R48R10R19R38L34R10R42L31R45L22L36R45L40L7L19R9R15L29L7R15L35L32R31R7L17R24L46L11R5L30R27R23R9R43L36R14R26L4L34L15R49L16R20L25L12R9L8L42L39R6R20L18R9L23R15L38R13R24R18L49L47L35L41L41L8R7R10R18L39R13R46L48L49L39L49L50L34R10R30L4L19L16R18L18L12R44L17R34R2R9L11L6L14L42R29R22L8R19R42L49R29L5R44R42R46L30R44R32L45R6R38R17R14R37L47L7R5L9R7R33R10R36L13L35R19R46L32L7L8R27R3L36R39R1L26R11L20L34R38R35L10L38L5L39R17R20L26L45R48R33R20L22L5R36L16L27L28L5L41R22L9L46L20R17R1R15R12R20L24L43L35L50R6R33R5L21R21L32R34L47R13L18L4R2L43R12L30L50R49R39R20L1L45R5L12R30R6R3R7L10R44L44R33R48R14R3R45R35R11R5R10L26R9L13R37R9R34L42L11L1R42R33L4R9L26R6L2L20R39R24R17R24L22R36R4L29R35L10R12L49R9L4R13L11R18R28L49R4L18R48R49R19L28L9L49R34L40L49R31L3R19L11L33R50L33L1R47R2L7R47L34R33L39R8R49R15R47R17R14L19L5R32R24L10L21R13L47L38R23L14R40L47L5L11L24R33R34L45L38L5R41R22L3L11R44L47R5R20L39R21L49L25R28L17R18R36L16L5L12R25L39L8R12R13R18L34R45R37R22L47L8R13R18L15L9L33R28L11L1L30R49L20L27R1R2R3L42L20L44R23L11R17L18R29R31L11L20R11L6L28L3R21L22R36L49R37L39R16R1L9L48R27R32R4L3L24R6R40L28R2R39L50R27L45L15R32R44R14R8R21R5L18L24L45R29L42L46R13L39L31R43R2L27L45L42R48L3L18R20L3R30L25R12L47L23L49R27L21L46L40R21L26L22R22L22L45L18R42L6L18R30R18L23R46R22R8L44L14R26L17R26L45R41R16L46R2R16L50R20L39L24R41R7L19L25L24R29R48R41L30L10L26R3L7R41L40R20R26R40R44R3L37R22R44R17R38L47R35L28L19L23L34L8L12R8R40R43L8R43L42R43R44L30R34L6L50R20L45R25R4R9L39L29L17R1L32L18R10L8R21R29L26L34R26R41R2R49L42R43R3R10R7L14L37L1L29L15L19L23R15R28R20R2R31R44L16R38R19R12R26R42R20R12L35R34R48R13L49L48L35R30R38R21L47L41R6R10R32R46R14L20R34L1R5R7L29L41L26L41R41L41R46R18L28R18L25L20L37R47L17L35L30L49L13L19L42R36L37L36R2R1L36L31R20L36L46L26L25R26L45R18R15L42L50R17L31R30L16R45R47L11R4R29R3R9L23L19R14L15L18L14R44L4L20L20L44R17L43L9R11R42R19L1R10R44L47L46R26R22R6L24R22R34R5R35L17L12L10R42L49L43L50R5L44L24L31R7L6R37L34R7R5R11L35R34L27R18L6R16L7L31L31L50L29R34L47R4R10L48L13L10R37L48L40L32R2R36L26L41L28R38R29L32R50R32L39R30R43L39R40R35R9R34L21L25L6L14L26L42R43R5L39L1L20L37L36R25L39L14L21L37L4L10R11L35L17L47R6L30L20L18R12L10L14R37R38L2R23L17R11L1L12R34L42L24L8R34L40R34R6R25L27R25L21L4R44L41R20R14L30R30L22L40R45L11R26R34R7R40L2R43R16R17R20R35R18L1L2R22R29R44L17L41L50L8R33R2L45R49L48R46R25L23L48L13R49L1L2R18L24R19R3L29R24L20L16L48R12R8L35L4L33R19R34R11L44R40L5L7R23R46L10L15R32R49L24L40L11R26L48L13L22R23L21R45L31R49R43L16L33L39R18R24R34L39R21L45R37R7L17R44L27L34L35L11L48L7L13L41R47L19R27L17R44R29R48L48R2R35L47R46L38L20R5L49R19L17R8L13L11R30R27R18R3L11R43L19L14R37R31R42L11R15R31L1L39R16R42L46R34L48R6R48R23R48L21L19R6R17L18R7L6L22R1L39L20R28R27L10L9L4L31R34R27L29L12R30R9L28R13R37L37R48L10L28L31R33R45R1R21L40R7R10R18L17R5R11L4L48R41L37R18L50L48R30L25R47R29R40L29R4R22R44L30R5L17L43R20R4R17R14L47L20L16L33L31R48L9L30R38L21L7R5R4L13R49L48R41L28L7R30R23L25L37R23L37R34R7L22R43L36R43R8R29R35R33L14R45R2R50L44R21R24L33L39R46L47L13L48L32L22R8L16L5L13L11R35R12R36R11L50L40L26L26L16L44L26L25R26R25L31L15R31R41R7L45L29L37R15L4L36L5L47R10L46R30L1L10R21R14R3L46R7R50L40L3L7R34L34L23L49L7R19L27L32L13R10R26L26R49R41L17L27L50R29L11L31L9R13R16L47R47L40L13R33L20R24L44R14R3R46L25R11L24L26L2R2L23R34L24R9L4R22L38R24L46L6R28R35R42L5L35R11L21R42R9L40R45R22R17R9L20R6L13L27L38L32L16R23L43R33R16R10L16L6R11L9L22R44L23L31R1L20L17L9R12L44R3R49R7L8L12R5R43R33L26L16R33L48R29R26L50R9R43R48R36L30R17R42";
        let steps = get_instr(steps_t);
        let mut session = Session::new(rows, cols, walls, steps);
        let part1 = session.process_part1();
        println!("Part 1: {}", part1);
    }
}
