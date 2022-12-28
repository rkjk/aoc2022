use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::{min, max};

type Cube = (i32, i32, i32);

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

fn parse_input(inp: Vec<String>) -> (HashSet<Cube>, i32, i32, i32, i32, i32, i32) {
    let (mut xmax, mut ymax, mut zmax) = (0, 0, 0);
    let (mut xmin, mut ymin, mut zmin) = (i32::MAX, i32::MAX, i32::MAX);
    let col = inp.into_iter().map(|l| {
        let tmp: Vec<i32> = l.trim().split(",").map(|num| num.parse::<i32>().unwrap()).collect();
        xmax = max(xmax, tmp[0]);
        ymax = max(ymax, tmp[1]);
        zmax = max(zmax, tmp[2]);
        xmin = min(xmin, tmp[0]);
        ymin = min(ymin, tmp[1]);
        zmin = min(zmin, tmp[2]);
        (tmp[0], tmp[1], tmp[2])
    }).collect();
    (col, xmax, ymax, zmax, xmin, ymin, zmin)
}

struct Session {
    cubes: HashSet<Cube>,
    starting_points: HashSet<Cube>,
    air_trapped: HashSet<Cube>,
    xmax: i32,
    ymax: i32,
    zmax: i32,
    xmin: i32,
    ymin: i32,
    zmin: i32
}

impl Session {
    pub fn new(inp: HashSet<Cube>, xmax: i32, ymax: i32, zmax: i32, xmin: i32, ymin: i32, zmin: i32) -> Self {
        Session {
            cubes: inp,
            starting_points: HashSet::new(),
            air_trapped: HashSet::new(),
            xmax: xmax,
            ymax: ymax,
            zmax: zmax,
            xmin: xmin,
            ymin: ymin,
            zmin: zmin
        }
    }

    fn get_neighbours(cube: &Cube) -> Vec<Cube> {
        let &(x, y, z) = cube;
        vec![
            (x + 1, y, z), (x - 1, y, z),
            (x, y + 1, z), (x, y - 1, z),
            (x, y, z + 1), (x, y, z - 1)
        ]
    }

    pub fn count_surfaces(&mut self) -> usize {
        let mut count = 0;
        for v in self.cubes.iter() {
            let mut init_count = 6;
            for n in Session::get_neighbours(v) {
                if self.cubes.contains(&n) {
                    init_count -= 1;
                }
            }
            count += init_count;
        }
        count
    }

    pub fn count_surfaces_2(&self) -> usize {
        let mut count = 0;
        for v in self.cubes.iter() {
            let mut init_count = 6;
            for n in Session::get_neighbours(v) {
                if self.cubes.contains(&n) || self.air_trapped.contains(&n) {
                    init_count -= 1;
                }
            }
            count += init_count;
        }
        count
    }

    fn check_boundary(&self, cube: &Cube) -> bool {
        cube.0 < self.xmin - 1 || cube.1 < self.ymin - 1|| cube.2 < self.zmin - 1
        || cube.0 > self.xmax + 1 || cube.1 > self.ymax + 1 || cube.2 > self.zmax + 1
    }

    fn dfs(&self, cur_air: Cube, visited: &mut HashSet<Cube>, result: &mut HashMap<Cube, bool>, depth: usize) -> bool {
        //println!("cur_air: {:?}, depth: {}", cur_air, depth);
        if result.contains_key(&cur_air) {
            return *result.get(&cur_air).unwrap();
        }
        if visited.contains(&cur_air) {
            return true;
        }
        visited.insert(cur_air);
        let nns = Session::get_neighbours(&cur_air);
        let tmp: Vec<bool> = nns.into_iter().map(|potential| {
            if self.cubes.contains(&potential) {
                return true;
            }
            if self.check_boundary(&potential) {
                return false;
            }
            if visited.contains(&potential) {
                return true;
            }
            return self.dfs(potential, visited, result, depth + 1);
        }).collect();
        let res = tmp.into_iter().all(|v| v);
        result.insert(cur_air, res);
        visited.remove(&cur_air);
        res
    }

    fn bfs(&mut self) {
        let mut map: HashSet<Cube> = HashSet::new();
        map.insert((self.xmin - 1, self.ymin - 1, self.zmin - 1));
        let mut q = VecDeque::from(vec![(self.xmin - 1, self.ymin - 1, self.zmin - 1)]);
        while !q.is_empty() {
            let cube = q.pop_front().unwrap();
            for n in Session::get_neighbours(&cube) {
                if self.cubes.contains(&n) || map.contains(&n) || self.check_boundary(&n) {
                    continue;
                }
                map.insert(n);
                q.push_back(n);
            }
        }
        let mut set = HashSet::new();
        let (xmin, ymin, zmin) = (self.xmin, self.ymin, self.zmin);
        let (xmax, ymax, zmax) = (self.xmax, self.ymax, self.zmax);
        for x in xmin - 1..xmax + 1 {
            for y in ymin - 1..ymax + 1 {
                for z in zmin - 1..zmax + 1 {
                    let cube = (x, y, z);
                    if !map.contains(&cube) && !self.cubes.contains(&cube) {
                        set.insert(cube);
                    }
                }
            }
        }
        //println!("{:?}", set);
        self.air_trapped = set;
    }

    /// DFS exceeds recursion limit
    pub fn get_air_trapped(&mut self) {
        self.bfs();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (inp, xmax, ymax, zmax, xmin, ymin, zmin) = parse_input(read_input("in.test").unwrap());
        let mut session = Session::new(inp, xmax, ymax, zmax, xmin, ymin, zmin);
        let part1 = session.count_surfaces();
        session.get_air_trapped();
        let part2 = session.count_surfaces_2();
        println!("Test 1: {}", part1);
        println!("Test 2: {}", part2);
    }

    #[test]
    fn actual() {
        let (inp, xmax, ymax, zmax, xmin, ymin, zmin) = parse_input(read_input("in.1").unwrap());
        let mut session = Session::new(inp, xmax, ymax, zmax, xmin, ymin, zmin);
        let part1 = session.count_surfaces();
        session.get_air_trapped();
        let part2 = session.count_surfaces_2();
        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2);
    }
}
