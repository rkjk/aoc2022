use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::{min, max};

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

/// Assume bottom left is origin in our case
fn parse_input(inp: Vec<String>) -> Elves {
    let mut set = HashSet::new();
    let height = inp.len() - 1;
    for (i, v) in inp.into_iter().enumerate() {
        for (j, c) in v.chars().enumerate() {
            match c {
                '#' => { set.insert((j as Val, (height - i) as Val)); },
                _ => (),
            };
        }
    }
    Elves::new(set)
}

type Val = i32;
type Pos = (Val, Val);

enum Dir {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW
}

impl Dir {
    fn all() -> Vec<Self> {
        vec![Dir::N, Dir::S, Dir::E, Dir::W, Dir::NE, Dir::NW, Dir::SE, Dir::SW]
    }

    fn north() -> Vec<Self> {
        vec![Dir::N, Dir::NE, Dir::NW]
    }

    fn south() -> Vec<Self> {
        vec![Dir::S, Dir::SE, Dir::SW]
    }

    fn east() -> Vec<Self> {
        vec![Dir::E, Dir::SE, Dir::NE]
    }

    fn west() -> Vec<Self> {
        vec![Dir::W, Dir::SW, Dir::NW]
    }
}

#[derive(Debug)]
struct Elves {
    map: HashSet<Pos>
}

impl Elves {
    pub fn new(inp: HashSet<Pos>) -> Self {
        Elves {
            map: inp
        }
    }

    pub fn length(&self) -> usize {
        self.map.len()
    }

    pub fn get_empty(&self) -> usize {
        let (mut minx, mut maxx, mut miny, mut maxy) = (Val::MAX, Val::MIN, Val::MAX, Val::MIN);
        for val in self.map.iter() {
            minx = min(minx, val.0);
            maxx = max(maxx, val.0);
            miny = min(miny, val.1);
            maxy = max(maxy, val.1);
        }
        let total_points = ((maxy - miny + 1) * (maxx - minx + 1)) as usize;
        total_points - self.length()
    }

    pub fn get_map(&self) -> &HashSet<Pos> {
        &self.map
    }

    pub fn remove(&mut self, point: &Pos) {
        self.map.remove(point);
    }

    pub fn insert(&mut self, point: Pos) {
        self.map.insert(point);
    }

    fn get_neighbour(point: &Pos, direction: Dir) -> Pos {
        let &(x, y) = point;
        match direction {
            Dir::N => (x, y + 1),
            Dir::NE => (x + 1, y + 1),
            Dir::NW => (x - 1, y + 1),
            Dir::E => (x + 1, y),
            Dir::W => (x - 1, y),
            Dir::S => (x, y - 1),
            Dir::SE => (x + 1, y - 1),
            Dir::SW => (x - 1, y - 1)
        }
    }

    fn get_neighbours(point: &Pos, directions: Vec<Dir>) -> Vec<Pos> {
        directions.into_iter().map(|d| Elves::get_neighbour(point, d)).collect()
    }

    /*
    pub fn get_all_neighbours(point: &Pos) -> Vec<Pos> {
        Elves::get_neighbours(point, Dir::all())
    }

    pub fn get_north_neighbours(point: Pos) -> Vec<Pos> {
        Elves::get_neighbours(point, Dir::north())
    }

    pub fn get_south_neighbours(point: Pos) -> Vec<Pos> {
        Elves::get_neighbours(point, Dir::south())
    }

    pub fn get_east_neighbours(point: Pos) -> Vec<Pos> {
        Elves::get_neighbours(point, Dir::east())
    }

    pub fn get_west_neighbours(point: Pos) -> Vec<Pos> {
        Elves::get_neighbours(point, Dir::west())
    }
    */

    pub fn is_move_possible(&self, points: &Vec<Pos>) -> bool {
        points.iter().all(|p| !self.map.contains(p))
    }

    pub fn check_end(&self, point: &Pos) -> bool {
        self.is_move_possible(&Elves::get_neighbours(point, Dir::all()))
    }

    pub fn check_north(&self, point: &Pos) -> bool {
        self.is_move_possible(&Elves::get_neighbours(point, Dir::north()))
    }

    pub fn check_south(&self, point: &Pos) -> bool {
        self.is_move_possible(&Elves::get_neighbours(point, Dir::south()))
    }

    pub fn check_east(&self, point: &Pos) -> bool {
        self.is_move_possible(&Elves::get_neighbours(point, Dir::east()))
    }

    pub fn check_west(&self, point: &Pos) -> bool {
        self.is_move_possible(&Elves::get_neighbours(point, Dir::west()))
    }
}

enum State {
    Continue,
    End
}

struct Session {
    cur_pos: Elves,
    cur_dir: VecDeque<Dir>
}

impl Session {
    pub fn new(inp: Elves) -> Self {
        Session {
            cur_pos: inp,
            cur_dir: VecDeque::from(vec![Dir::N, Dir::S, Dir::W, Dir::E]),
        }
    }

    fn run_round(&mut self) -> State {
        let mut new_positions: HashMap<Pos, Vec<Pos>> = HashMap::new();
        let mut count = 0;
        for elf in self.cur_pos.get_map().iter() {
            if self.cur_pos.check_end(elf) {
                count += 1;
                continue;
            }
            for d in self.cur_dir.iter() {
                let new_position = match d {
                    Dir::N => {
                        match self.cur_pos.check_north(elf) {
                            true => Some((elf.0, elf.1 + 1)),
                            false => None,
                        }
                    },
                    Dir::S => {
                        match self.cur_pos.check_south(elf) {
                            true => Some((elf.0, elf.1 - 1)),
                            false => None,
                        }
                    },
                    Dir::W => {
                        match self.cur_pos.check_west(elf) {
                            true => Some((elf.0 - 1, elf.1)),
                            false => None
                        }
                    },
                    Dir::E => {
                        match self.cur_pos.check_east(elf) {
                            true => Some((elf.0 + 1, elf.1)),
                            false => None,
                        }
                    },
                    _ => None,
                };
                match new_position {
                    Some(new_pos) => {
                        new_positions.entry(new_pos)
                            .and_modify(|v| v.push(*elf))
                            .or_insert(vec![*elf]);
                        break;
                    },
                    None => (),
                };
            }
        }
        if count == self.cur_pos.length() {
            return State::End;
        }
        new_positions.retain(|_, v| v.len() == 1);
        for (k, v) in new_positions.into_iter() {
            //println!("Moving {:?} to {:?}", &v[0], k);
            self.cur_pos.remove(&v[0]);
            self.cur_pos.insert(k);
        }
        let first_dir = self.cur_dir.pop_front().unwrap();
        self.cur_dir.push_back(first_dir);
        State::Continue
    }

    pub fn run_rounds(&mut self, num_rounds: usize) {
        for i in 0..num_rounds {
            //println!("Round: {}", i);
            match self.run_round() {
                State::Continue => (),
                State::End => break,
            };
        }
    }

    pub fn count_empty(&self) -> usize {
        self.cur_pos.get_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = parse_input(read_input("in.test").unwrap());
        let mut session = Session::new(input);
        session.run_rounds(10);
        let part1 = session.count_empty();
        println!("Test 1: {}", part1);
    }

    #[test]
    fn actual() {
        let input = parse_input(read_input("in.1").unwrap());
        let mut session = Session::new(input);
        session.run_rounds(10);
        let part1 = session.count_empty();
        println!("Part 1: {}", part1);
    }
}
