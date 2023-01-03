use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::{HashMap, VecDeque};
use std::cmp::{min, max};

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

#[derive(Debug, Clone)]
enum Op {
    Num(i32),
    Node(String)
}

type DoubleArg = (Op, Op);

#[derive(Debug, Clone)]
enum Job {
    Val(Op),
    Add(DoubleArg),
    Sub(DoubleArg),
    Mul(DoubleArg),
    Div(DoubleArg)
}

#[derive(Debug)]
struct WrapperJob {
    job: Job,
    value: Option<i32>
}

impl WrapperJob {
    pub fn new(job: Job) -> Self {
        let mut wj = WrapperJob {
            job: job,
            value: None
        };
        wj.evaluate();
        wj
    }

    pub fn get_value(&self) -> Option<i32> {
        self.value
    }

    pub fn get_job(&self) -> Job {
        self.job.clone()
    }

    pub fn evaluate(&mut self) {
        match self.get_job() {
            Job::Val(op) => {
                if let Op::Num(v) = op {
                    self.value = Some(v);
                }
            },
            Job::Add((o1, o2)) => {
                if let (Op::Num(v1), Op::Num(v2)) = (o1, o2) {
                    self.value = Some(v1 + v2);
                }
            },
            Job::Sub((o1, o2)) => {
                if let (Op::Num(v1), Op::Num(v2)) = (o1, o2) {
                    self.value = Some(v1 - v2);
                }
            },
            Job::Mul((o1, o2)) => {
                if let (Op::Num(v1), Op::Num(v2)) = (o1, o2) {
                    self.value = Some(v1 * v2);
                }
            },
            Job::Div((o1, o2)) => {
                if let (Op::Num(v1), Op::Num(v2)) = (o1, o2) {
                    self.value = Some(v1 / v2);
                }
            }
        };
    }

    pub fn replace(&mut self, val: i32, pos: usize) {
        let num = Op::Num(val);
        match self.get_job() {
            Job::Add((o1, o2)) => {
                match pos == 0 {
                    true => self.job = Job::Add((num, o2)),
                    false => self.job = Job::Add((o1, num)),
                };
            },
            Job::Sub((o1, o2)) => {
                match pos == 0 {
                    true => self.job = Job::Sub((num, o2)),
                    false => self.job = Job::Sub((o1, num)),
                };
            },
            Job::Mul((o1, o2)) => {
                match pos == 0 {
                    true => self.job = Job::Mul((num, o2)),
                    false => self.job = Job::Mul((o1, num)),
                };
            },
            Job::Div((o1, o2)) => {
                match pos == 0 {
                    true => self.job = Job::Div((num, o2)),
                    false => self.job = Job::Div((o1, num)),
                };
            },
            _ => panic!("cannot replace value"),
        };
    }
}

type Connections = HashMap<String, Vec<String>>;

type Graph = HashMap<String, WrapperJob>;

fn parse_input(inp: Vec<String>) -> (Connections, Graph) {
    let mut connections = HashMap::new();
    let mut graph = HashMap::new();
    for line in inp.into_iter() {
        let l_r: Vec<&str> = line.split(":").collect();
        let parent = l_r[0].to_string();
        let r = l_r[1].trim();
        if let Ok(v) = r.parse::<i32>() {
            graph.insert(parent, WrapperJob::new(Job::Val(Op::Num(v))));
            continue;
        }
        let rhs: Vec<&str> = r.split_whitespace().collect();
        let (lop, op, rop) = (rhs[0].to_string(), rhs[1], rhs[2].to_string());
        connections.entry(lop.to_owned()).and_modify(|v: &mut Vec<String>| v.push(parent.to_owned())).or_insert(vec![parent.to_owned()]);
        connections.entry(rop.to_owned()).and_modify(|v: &mut Vec<String>| v.push(parent.to_owned())).or_insert(vec![parent.to_owned()]);
        match op {
            "+" => graph.insert(parent, WrapperJob::new(Job::Add((Op::Node(lop), Op::Node(rop))))),
            "-" => graph.insert(parent, WrapperJob::new(Job::Sub((Op::Node(lop), Op::Node(rop))))),
            "*" => graph.insert(parent, WrapperJob::new(Job::Mul((Op::Node(lop), Op::Node(rop))))),
            "/" => graph.insert(parent, WrapperJob::new(Job::Div((Op::Node(lop), Op::Node(rop))))),
            _ => panic!("Unknown op"),
        };
    }
    (connections, graph)
}

struct Session {
    connections: Connections,
    graph: Graph
}

impl Session {
    pub fn new(connections: Connections, graph: Graph) -> Self {
        Session {
            connections: connections,
            graph: graph
        }
    }

    fn get_starting_points(&self) -> VecDeque<String> {
        self.graph.iter().filter(|(k, v)| match v.get_job() {
            Job::Val(v) => true,
            _ => false
        }).map(|(k, v)| k.to_owned()).collect()
    }

    /// If a node has Job::Val or any other job with both Num, then we can consider that visited
    /// In first iteration, only Val's are visited. Using these, we visit connected nodes and replace Nodes with Nums.
    /// When doing the replacement, if we detect both to be Nums, we can add to visited and use these parents
    /// as our next starting points. Continue iterations until we exhaust everything.
    /// Finally, we can return the value of root alone.
    fn fill_graph(&mut self) {
        let mut q = self.get_starting_points();
        while !q.is_empty() {
            let cur_node = q.pop_front().unwrap();
            let v = self.graph.get(&cur_node);
            //match v {
            //    &Job::Val(v) => 
            //}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (connections, graph) = parse_input(read_input("in.test").unwrap());
        println!("connections: {:?}", connections);
        println!("graph: {:?}", graph);
    }
}
