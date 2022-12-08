use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::HashMap;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

fn parse_input(inp: Vec<String>) -> Vec<Vec<Tree>> {
    inp.into_iter().map(|line| line.bytes().map(|b| Tree::new(b - b'0')).collect()).collect()
}

#[derive(Debug, Clone)]
struct Tree {
    pub height: u8,
    pub visible: HashMap<Direction, bool>,
    pub scenic_distances: Vec<usize>,
}

impl Tree {
    pub fn new(height: u8) -> Self {
        Tree {
            height: height,
            visible: HashMap::new(),
            scenic_distances: Vec::new(),
        }
    }

    pub fn mark_visibility(&mut self, direction: Direction, visibility: bool) {
        self.visible.insert(direction, visibility);
    }

    pub fn is_visible(&self) -> bool {
        self.visible.values().any(|&x| x)
    }

    pub fn get_distance(&self) -> usize {
        if self.scenic_distances.len() < 4 {
            return 0;
        }
        if self.scenic_distances.len() > 4 {
            panic!("Should not be greater than 4");
        }
        let mut val = 1;
        for v in self.scenic_distances.iter() {
            val *= v;
        }
        val
    }

    pub fn add_distance(&mut self, dist: usize) {
        self.scenic_distances.push(dist);
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom
}

fn process_direction(forest: &mut Vec<Vec<Tree>>, direction: Direction) {
    let end_idx = match direction {
        Direction::Left | Direction::Right => forest[0].len(),
        Direction::Bottom | Direction::Top => forest.len()
    };
    let start_idx = match direction {
        Direction::Left | Direction::Top => 0,
        Direction::Bottom => forest.len(),
        Direction::Right => forest[0].len()
    };
    let range = match direction {
        Direction::Left | Direction::Top => (0..end_idx).collect::<Vec<usize>>(),
        Direction::Right | Direction::Bottom => (0..end_idx).rev().collect::<Vec<usize>>()
    };
    let iterations = match direction {
        Direction::Left | Direction::Top => (0..forest.len()).collect::<Vec<usize>>(),
        Direction::Right | Direction::Bottom => (0..forest[0].len()).collect::<Vec<usize>>()
    };
    //println!("Direction: {:?}, iteration: {:?}, range: {:?}", direction, iterations, range);
    for it in iterations.iter() {
        let mut cur_max: Option<u8> = None;
        for ind in range.iter() {
            let (left_ind, right_ind) = match direction {
                Direction::Left | Direction::Right => (*it, *ind),
                Direction::Top | Direction::Bottom => (*ind, *it)
            };
            let height = forest[left_ind][right_ind].height;
            if cur_max.is_none() {
                cur_max = Some(height);
                forest[left_ind][right_ind].mark_visibility(direction, true);
                //println!("Left Ind: {}, Right Ind: {}, cur_max: {:?}, tree: {:?}", left_ind, right_ind, cur_max, forest[left_ind][right_ind]);
                continue;
            }
            if cur_max.unwrap() < height {
                cur_max = Some(height);
                forest[left_ind][right_ind].mark_visibility(direction, true);
            }
            //println!("Left Ind: {}, Right Ind: {}, cur_max: {:?}, tree: {:?}", left_ind, right_ind, cur_max, forest[left_ind][right_ind]);
            let mut val = 0;
            match direction {
                Direction::Left => {
                    for i in (0..*ind).rev() {
                        val += 1;
                        if forest[*it][i].height >= height {
                            break;
                        }
                    }
                },
                Direction::Right => {
                    for i in *ind+1..forest[0].len() {
                        val += 1;
                        if forest[*it][i].height >= height {
                            break;
                        }
                    }
                },
                Direction::Top => {
                    for j in (0..*ind).rev() {
                        val += 1;
                        if forest[j][*it].height >= height {
                            break;
                        }
                    }
                },
                Direction::Bottom => {
                    for j in *ind+1..forest.len() {
                        val += 1;
                        if forest[j][*it].height >= height {
                            break;
                        }
                    }
                }
            };
            forest[left_ind][right_ind].add_distance(val);
        }
    }
}

fn generate_visibility(forest: &mut Vec<Vec<Tree>>) {
    for direction in [Direction::Left, Direction::Right, Direction::Top, Direction::Bottom].into_iter() {
        process_direction(forest, direction);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut input = parse_input(read_input("in.test").unwrap());
        generate_visibility(&mut input);
        let part1 = input.iter().cloned().into_iter().flatten().filter(|tree| tree.is_visible()).count();
        let part2 = input.into_iter().flatten().map(|tree| tree.get_distance()).max().unwrap();
        println!("Test 1: {}", part1);
        println!("Test 2: {}", part2);
    }

    #[test]
    fn actuak() {
        use std::time::Instant;
        let now = Instant::now();
        let mut input = parse_input(read_input("in.1").unwrap());
        generate_visibility(&mut input);
        let part1 = input.iter().cloned().into_iter().flatten().filter(|tree| tree.is_visible()).count();
        let part2 = input.into_iter().flatten().map(|tree| tree.get_distance()).max().unwrap();
        let elapsed = now.elapsed();
        println!("Part 1: {:?}", part1);
        println!("Part 2: {}", part2);
        println!("Elapsed: {:.2?}", elapsed);
    }
}
