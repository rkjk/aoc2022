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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Dir {
    U,
    D,
    L,
    R
}

type Coord = (usize, usize);

#[derive(Debug, Eq, Hash, Clone)]
struct Blizzard {
    pub pos: Coord,
    pub dir: Dir
}

impl PartialEq for Blizzard {
    fn eq(&self, other: &Blizzard) -> bool {
        self.pos == other.pos && self.dir == other.dir
    }
}

impl Blizzard {
    pub fn new(init_pos: Coord, dir: Dir) -> Self {
        Blizzard {
            pos: init_pos,
            dir: dir
        }
    }

    /// uwall and lwall are row = 0 and col = 0 respectively
    pub fn update_loc(&mut self, rwall: usize, dwall: usize) {
        let (r, c) = self.pos;
        let (mut nr, mut nc) = match self.dir {
            Dir::U => (r - 1, c),
            Dir::D => (r + 1, c),
            Dir::L => (r, c - 1),
            Dir::R => (r, c + 1)
        };
        if nr == 0 {
            nr = dwall - 1;
        }
        if nr == dwall {
            nr = 1;
        }
        if nc == 0 {
            nc = rwall - 1;
        }
        if nc == rwall {
            nc = 1;
        }
        self.pos = (nr, nc);
    }
}

/// Return Start coord, end coord, and set of blizzard locations
/// Coords are in the usual matrix row-col i.e top left is 0,0. First row is 0,x, Second column is (y, 1)
fn parse_input(inp: Vec<String>) -> (Vec<Blizzard>, usize, usize) {
    let mut list = Vec::new();
    let (nrows, ncols) = (inp.len(), inp[0].len());
    for (i, val) in inp.into_iter().enumerate() {
        for (j, c) in val.chars().enumerate() {
            match c {
                '>' => { list.push(Blizzard::new((i, j), Dir::R)); },
                '<' => { list.push(Blizzard::new((i, j), Dir::L)); },
                'v' => { list.push(Blizzard::new((i, j), Dir::D)); },
                '^' => { list.push(Blizzard::new((i, j), Dir::U)); },
                _ => (),
            }
        }
    }
    (list, nrows, ncols)
}

struct Session {
    nrows: usize,
    ncols: usize,
    blizzards: Vec<Vec<Blizzard>>, // Holds all blizzards per iteration
    blizzard_locations: Vec<HashSet<Coord>> // Holds all unique blizzard locations per iteration
}

impl Session {
    pub fn new(blizzards: Vec<Blizzard>, nrows: usize, ncols: usize) -> Self {
        let mut set = HashSet::new();
        for v in blizzards.iter() {
            set.insert(v.pos);
        }
        Session {
            blizzards: vec![blizzards],
            nrows: nrows,
            ncols: ncols,
            blizzard_locations: vec![set]
        }
    }

    fn update_all_locs(&mut self) {
        let (rwall, dwall) = (self.ncols - 1, self.nrows - 1);
        let mut clone = self.blizzards[self.blizzards.len() - 1].clone();
        let mut set = HashSet::new();
        for val in clone.iter_mut() {
            val.update_loc(rwall, dwall);
            set.insert(val.pos);
        }
        self.blizzards.push(clone);
        self.blizzard_locations.push(set);
    }

    fn generate_new_blizzards(&mut self, iteration: usize) {
        if iteration < self.blizzards.len() {
            return;
        }
        for i in self.blizzards.len()..iteration+1 {
            self.update_all_locs();
        }
    }

    fn check_validity(&self, cur_pos: Coord, cur_it: usize) -> bool {
        self.blizzard_locations[cur_it].iter().all(|v| *v != cur_pos)
    }

    fn get_new_positions(&self, pos: Coord) -> Vec<Coord> {
        let (r, c) = pos;
        let mut vec = Vec::new();
        if r >= 1 {
            vec.push((r - 1, c));
        }
        if r < self.nrows - 1 {
            vec.push((r + 1, c))
        }
        if c >= 1 {
            vec.push((r, c - 1));
        }
        if c < self.ncols - 1 {
            vec.push((r, c + 1));
        }
        vec
    }

    pub fn find_shortest_path(&mut self, start: Coord, end: Coord, start_iter: usize) -> usize {
        //println!("start: {:?} end: {:?}", start, end);
        let IT_MAX = 750;
        let mut shortest_time = usize::MAX;
        let mut visited = HashSet::new();
        let mut q = VecDeque::from(vec![(start, start_iter)]); // Cur-coordinate, cur-iteration
        while !q.is_empty() {
            let (cur_loc, cur_it) = q.pop_front().unwrap();
            if cur_it > IT_MAX {
                continue;
            }
            // If you have reached the end
            //if cur_loc == end {
            //    shortest_time = min(shortest_time, cur_it);
            //    continue;
            //}
            if visited.contains(&(cur_loc, cur_it)) {
                continue;
            }
            // Generate blizzard locations for this iteration
            self.generate_new_blizzards(cur_it);
            // Add to visited
            //println!("cur_loc: {:?} cur_it: {}", cur_loc, cur_it);
            visited.insert((cur_loc, cur_it));
            // Check if this is a valid position without any blizzards
            if !self.check_validity(cur_loc, cur_it) {
                //println!("here");
                continue;
            }
            for (r, c) in self.get_new_positions(cur_loc) {
                if (r, c) == end {
                    shortest_time = min(shortest_time, cur_it + 1);
                    continue;
                }
                if r == 0 || r == self.nrows - 1 || c == 0 || c == self.ncols - 1 {
                    continue;
                }
                q.push_back(((r, c), cur_it + 1));
            }
            // Wait
            q.push_back((cur_loc, cur_it + 1));
        }
        shortest_time
    }

    pub fn orchestrate(&mut self, start: Coord, end: Coord) -> usize {
        let val1 = self.find_shortest_path(start, end, 0);
        let val2 = self.find_shortest_path(end, start, val1);
        self.find_shortest_path(start, end, val2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (blizzards, nrows, ncols) = parse_input(read_input("in.test").unwrap());
        //println!("blizzards: {:?}", blizzards);
        let mut session = Session::new(blizzards, nrows, ncols);
        let part1 = session.find_shortest_path((0, 1), (nrows - 1, ncols - 2), 0);
        let part2 = session.orchestrate((0, 1), (nrows - 1, ncols - 2));
        println!("Test 1: {}", part1);
        println!("Test 2: {}", part2);
    }

    #[test]
    fn actual() {
        use std::time::Instant;
        let now = Instant::now();
        let (blizzards, nrows, ncols) = parse_input(read_input("in.1").unwrap());
        //println!("blizzards: {:?}", blizzards);
        let mut session = Session::new(blizzards, nrows, ncols);
        let part1 = session.find_shortest_path((0, 1), (nrows - 1, ncols - 2), 0);
        let part2 = session.orchestrate((0, 1), (nrows - 1, ncols - 2));
        let elapsed = now.elapsed();
        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2);
        println!("Elapsed: {:?}", elapsed);
    }
}
