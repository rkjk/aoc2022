use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::HashSet;
use std::cmp::{min, max};

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

type Point = (i32, i32);

/// Return all rock positions and the maximum y-coordinate  
/// (x increaes to the right and y increases as we go down)
fn parse_input(inp: Vec<String>) -> (HashSet<Point>, i32) {
    let get_coords = |s: &str| -> Point {
        let tmp: Vec<&str> = s.trim().split(",").collect();
        (tmp[0].parse::<i32>().unwrap(), tmp[1].parse::<i32>().unwrap())
    };
    let mut global_max_y = i32::MIN;
    let mut set = HashSet::new();
    for line in inp.into_iter() {
        let segments: Vec<&str> = line.split("->").collect();
        let (mut xp, mut yp) = get_coords(segments[0]);
        for i in 1..segments.len() {
            let (xn, yn) = get_coords(segments[i]);
            if xn != xp && yn != yp {
                panic!("Unknown case");
            }
            if xn == xp {
                let (miny, maxy) = (min(yn, yp), max(yn, yp));
                for y in miny..maxy+1 {
                    global_max_y = max(y, global_max_y);
                    set.insert((xn, y));
                }
            }
            if yn == yp {
                global_max_y = max(yn, global_max_y);
                let (minx, maxx) = (min(xn, xp), max(xn, xp));
                for x in minx..maxx+1 {
                    set.insert((x, yn));
                }
            }
            xp = xn;
            yp = yn;
        }
    }
    (set, global_max_y)
}

enum State {
    Drop,
    Stop
}

struct Session {
    rocks: HashSet<Point>,
    sand: HashSet<Point>,
    max_y: i32,
    start: Point
}

impl Session {
    pub fn new(rocks: HashSet<Point>, max_y: i32, start_coord: Point) -> Self {
        Session {
            rocks: rocks,
            sand: HashSet::new(),
            max_y: max_y,
            start: start_coord
        }
    }

    fn get_next_pos(&self, sand_pos: &Point) -> Option<Point> {
        let (x, y) = *sand_pos;
        let possibles = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)];
        for pos in possibles {
            if !self.sand.contains(&pos) && !self.rocks.contains(&pos) {
                return Some(pos);
            }
        }
        None
    }

    fn simulate_drop(&mut self) -> State {
        let mut cur_pos = self.start;
        let mut i = 0;
        loop {
            i += 1;
            //println!("Iter {}, pos: {:?}", i, cur_pos);
            match self.get_next_pos(&cur_pos) {
                Some(v) => {
                    cur_pos = v;
                    if cur_pos.1 > self.max_y {
                        return State::Stop;
                    }
                },
                None => {
                    self.sand.insert(cur_pos);
                    return State::Drop;
                }
            }
        }
    }

    fn simulate_drop_2(&mut self) -> State {
        let mut cur_pos = self.start;
        let mut i = 0;
        loop {
            i += 1;
            //println!("Iter {}, pos: {:?}", i, cur_pos);
            match self.get_next_pos(&cur_pos) {
                Some(v) => {
                    if v.1 == self.max_y + 2 {
                        self.sand.insert(cur_pos);
                        //println!("Drop_here: {:?}", cur_pos);
                        return State::Drop;
                    }
                    cur_pos = v;
                },
                None => {
                    self.sand.insert(cur_pos);
                    if cur_pos == (500, 0) {
                        return State::Stop;
                    }
                    //println!("Drop now: {:?}", cur_pos);
                    return State::Drop;
                }
            }
        }
    }


    pub fn drop_sand_until_abyss(&mut self, part2: bool) -> usize {
        let mut iterations = 0;
        match part2 {
            true => {
                loop {
                    match self.simulate_drop_2() {
                        State::Drop => iterations += 1,
                        State::Stop => break,
                    }
                }
            },
            false => {
                loop {
                    match self.simulate_drop() {
                        State::Drop => iterations += 1,
                        State::Stop => break,
                    }
                } 
            }
        }
        iterations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (inp, max_y) = parse_input(read_input("in.test").unwrap());
        let mut session = Session::new(inp.clone(), max_y, (500, 0));
        let part1 = session.drop_sand_until_abyss(false);
        let mut session2 = Session::new(inp, max_y, (500, 0));
        let part2 = session2.drop_sand_until_abyss(true);
        println!("Test 1: {}", part1);
        println!("Test 2: {}", part2 + 1);
    }

    #[test]
    fn actual() {
        use std::time::Instant;
        let now = Instant::now();
        let (inp, max_y) = parse_input(read_input("in.1").unwrap());
        let mut session = Session::new(inp.clone(), max_y, (500, 0));
        let part1 = session.drop_sand_until_abyss(false);
        let mut session2 = Session::new(inp, max_y, (500, 0));
        let part2 = session2.drop_sand_until_abyss(true);
        let elapsed = now.elapsed();
        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2 + 1);
        println!("Elapsed: {:?}", elapsed);
    }
}
