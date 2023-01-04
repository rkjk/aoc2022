use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::{HashMap, VecDeque};

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

type ValueType = i64;

#[derive(Debug, Clone)]
enum Op {
    Num(ValueType),
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

#[derive(Debug, Clone)]
struct WrapperJob {
    job: Job,
    value: Option<ValueType>
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

    pub fn get_value(&self) -> Option<ValueType> {
        self.value
    }

    pub fn get_job(&self) -> Job {
        self.job.clone()
    }

    pub fn evaluate(&mut self) {
        if self.get_value().is_some() {
            return;
        }
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

    fn get_position(&self, nn: &str) -> usize {
        match self.get_job() {
            Job::Add((o1, o2)) |
            Job::Sub((o1, o2)) |
            Job::Mul((o1, o2)) |
            Job::Div((o1, o2)) => {
                let s1 = match o1 {
                    Op::Node(ref v1) => Some(v1.clone()),
                    _ => None
                };
                let s2 = match o2 {
                    Op::Node(ref v2) => Some(v2.clone()),
                    _ => None,
                };
                if let Some(s) = s1 {
                    if s == nn {
                        return 0;
                    }
                }
                if let Some(s) = s2 {
                    if s == nn {
                        return 1;
                    }
                }
                panic!("Cannot replace with node: {}, o1: {:?}, o2: {:?}", nn, o1, o2);
            },
            _ => panic!("Cannot replace with node: {}, job: {:?}", nn, self.job),
        }
    }

    pub fn replace(&mut self, node: &str, val: ValueType) {
        let num = Op::Num(val);
        let pos = self.get_position(node);
        //println!("position: {}", pos);
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
        if let Ok(v) = r.parse::<ValueType>() {
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

    pub fn new_humn(connections: Connections, mut graph: Graph, v: ValueType) -> Self {
        graph.insert("humn".to_owned(), WrapperJob::new(Job::Val(Op::Num(v))));
        Session {
            connections: connections,
            graph: graph
        }
    }

    fn get_starting_points(&self) -> VecDeque<String> {
        self.graph.iter().filter(|(k, v)| v.get_value().is_some()).map(|(k, v)| k.to_owned()).collect()
    }

    /// If a node has an evaluated expression, then we can consider that visited
    /// In first iteration, only Val's are visited. Using these, we visit connected nodes and replace Nodes with Nums.
    /// When doing the replacement, if we detect both to be Nums, we can add to visited and use these parents
    /// as our next starting points. Continue iterations until we exhaust everything.
    /// Finally, we can return the value of root alone.
    fn fill_graph(&mut self) {
        let mut q = self.get_starting_points();
        //println!("Starting with: {:?}", q);
        while !q.is_empty() {
            let cur_node = q.pop_front().unwrap();
            let v = self.graph.get(&cur_node).unwrap().get_value().unwrap();
            //println!("Current node: {}, val: {}", cur_node, v);
            for con in self.connections.get(&cur_node) {
                for c in con {
                    let p_wrapperjob = self.graph.get_mut(c).unwrap();
                    //println!("Current job: {:?}", p_wrapperjob);
                    if p_wrapperjob.get_value().is_some() {
                        continue;
                    }
                    p_wrapperjob.replace(&cur_node, v);
                    //println!("Replaced job: {:?}", p_wrapperjob);
                    p_wrapperjob.evaluate();
                    if p_wrapperjob.get_value().is_some() {
                        //println!("Evaluated node {}, value: {}", c, p_wrapperjob.get_value().unwrap());
                        q.push_back(c.to_owned());
                    }
                }
            }
        }
    }

    pub fn get_root(&mut self) -> ValueType {
        self.fill_graph();
        self.graph.get("root").unwrap().get_value().unwrap()
    }

    pub fn get_root_equal(&mut self) -> DoubleArg {
        self.fill_graph();
        match self.graph.get("root").unwrap().get_job() {
            Job::Add(v) | Job::Sub(v) |
            Job::Mul(v) | Job::Div(v) => return v,
            _ => panic!("Should be a doublearg"),
        };
    }

    pub fn print_all(&self) {
        for (k, v) in self.graph.iter() {
            println!("Key: {}, Val: {}", k, v.get_value().unwrap());
        }
    }
}

struct SessionBinSearch {
    graph: Graph,
    connections: Connections
}

impl SessionBinSearch {
    pub fn new(connections: Connections, graph: Graph) -> Self {
        SessionBinSearch {
            connections: connections,
            graph: graph
        }
    }

    /// Example and actual differ in direction i.e if o1 > o2, for example, h = m -1 
    /// but for actual, l = m + 1
    pub fn run_binsearch(&self) -> ValueType {
        let mut l = 0;
        let mut h = 388222446619011;
        while l <= h {
            let m = l + (h  - l) / 2;
            let mut cur_session = Session::new_humn( self.connections.clone(), self.graph.clone(), m);
            let job = cur_session.get_root_equal();
            if let (Op::Num(o1), Op::Num(o2)) = job {
                if o1 > o2 {
                    l = m + 1;
                } else if o1 < o2 {
                    h  = m - 1;
                } else {
                    return m - 1;
                }
            }
            
        }
        panic!("No answer found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn it_works() {
        let (connections, graph) = parse_input(read_input("in.test").unwrap());
        let mut session_bin_search = SessionBinSearch::new(connections.clone(), graph.clone());
        let mut session = Session::new(connections, graph);
        let part1 = session.get_root();
        let part2 = session_bin_search.run_binsearch();
        //session.print_all();
        println!("Test 1: {}", part1);
        println!("Test 2: {}", part2);
    }

    #[test]
    fn actual() {
        use std::time::Instant;
        let now = Instant::now();
        let (connections, graph) = parse_input(read_input("in.1").unwrap());
        let mut session_bin_search = SessionBinSearch::new(connections.clone(), graph.clone());
        let mut session = Session::new(connections, graph);
        let part1 = session.get_root();
        let part2 = session_bin_search.run_binsearch();
        let elapsed = now.elapsed();
        //session.print_all();
        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2);
        println!("Elapsed: {:?}", elapsed);
    }
}
