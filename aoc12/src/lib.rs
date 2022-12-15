use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::VecDeque;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

type Pos = (usize, usize);

fn parse_input(inp: Vec<String>) -> (Pos, Pos, Vec<Vec<usize>>) {
    let mut res = vec![vec![0; inp[0].len()]; inp.len()];
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, line) in inp.into_iter().enumerate() {
        for (j, v) in line.chars().enumerate() {
            if v == 'S' {
                start = (i, j);
                continue;
            }
            if v == 'E' {
                end = (i, j);
                res[i][j] = 25;
                continue;
            }
            res[i][j] = v as usize - 97;
        }
    }
    (start, end, res)
}

struct Graph<'a> {
    start: &'a Pos,
    end: &'a Pos,
    topo: &'a Vec<Vec<usize>>
}

impl<'a> Graph<'a> {
    pub fn new(start: &'a Pos, end: &'a Pos, topo: &'a Vec<Vec<usize>>) -> Self {
        Graph {
            start: start,
            end: end,
            topo: topo
        }
    }

    pub fn get_shortest_path(&self) -> usize {
        let (m, n) = (self.topo.len(), self.topo[0].len());
        let mut shortest = vec![vec![usize::MAX; n]; m];
        let mut queue = VecDeque::from([(*self.start, 0)]);
        let get_next = |cur: Pos| -> Vec<Pos> {
            let mut res = Vec::new();
            if cur.0 > 0 {
                res.push((cur.0 - 1, cur.1));
            }
            if cur.0 < m - 1 {
                res.push((cur.0 + 1, cur.1));
            }
            if cur.1 > 0 {
                res.push((cur.0, cur.1 - 1));
            }
            if cur.1 < n - 1 {
                res.push((cur.0, cur.1 + 1));
            }
            res
        };
        while !queue.is_empty() {
            let ((cur_x, cur_y), cost) = queue.pop_front().unwrap();
            if cur_x < 0 || cur_x >= m || cur_y < 0 || cur_y >= n {
                continue;
            }
            let cur_height = self.topo[cur_x][cur_y];
            if cost >= shortest[cur_x][cur_y] {
                continue;
            }
            shortest[cur_x][cur_y] = cost;
            for v in get_next((cur_x, cur_y)) {
                let nex_height = self.topo[v.0][v.1];
                if cur_height + 1 >= nex_height {
                    queue.push_back((v, cost + 1));
                }
            }
        }
        shortest[self.end.0][self.end.1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (start, end, inp) = parse_input(read_input("in.test").unwrap());
        let graph = Graph::new(&start, &end, &inp);
        let part1 = graph.get_shortest_path();
        println!("Test 1: {}", part1);
    }

    #[test]
    fn actual() {
        let (start, end, inp) = parse_input(read_input("in.1").unwrap());
        let graph = Graph::new(&start, &end, &inp);
        let part1 = graph.get_shortest_path();
        println!("Part 1: {}", part1);
    }
}
