use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::cmp::{min, max};

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

type Graph = HashMap<usize, Vec<usize>>;

/// (current_tunnel, cur_time, flow_rate achieved so far)
type State = (usize, usize, usize);

#[derive(Debug, Clone)]
struct Mapping {
    pub tunnel: String,
    pub flow_rate: usize
}

impl Mapping {
    pub fn new(tun: String, rate: usize) -> Self {
        Mapping {
            tunnel: tun,
            flow_rate: rate
        }
    }
}

/// Graphs and map of tunnel to integer id
fn parse_input(inp: Vec<String>) -> (Graph, Vec<Mapping>) {
    let mut i: usize = 0;
    let mut mappings = HashMap::new();
    let mut graph = HashMap::new();
    let mut flow_rates_map = HashMap::new();
    let mut add_mapping = |val: String| -> usize {
        *mappings.entry(val).or_insert_with(|| {
            let tmp = i;
            i += 1;
            tmp
        })
    };
    for line in inp.into_iter() {
        let space_sep: Vec<&str> = line.split_whitespace().collect();
        let cur = space_sep[1];
        let cur_id = add_mapping(cur.to_string());
        let flrt_str: Vec<&str> = space_sep[4].split(";").next().unwrap().split("=").collect();
        let flrt = flrt_str[1].parse::<usize>().unwrap();
        flow_rates_map.insert(cur_id, flrt);
        let cons = space_sep[9..].to_vec();
        let mut tunnels = vec![];
        for val in cons {
            let val_new = match val.len() == 3 {
                true => val.split(",").next().unwrap(),
                false => val
            };
            let ind = add_mapping(val_new.to_string());
            tunnels.push(ind);
        }
        graph.insert(cur_id, tunnels);
    }
    let mut reverse_mappings = vec![Mapping::new("".to_string(), 0); mappings.len()];
    for (k, v) in mappings.into_iter() {
        let mapping = Mapping::new(k.to_string(), *flow_rates_map.get(&v).unwrap());
        reverse_mappings[v] = mapping;
    }
    (graph, reverse_mappings)
}

struct Session {
    graph: Graph,
    mappings: Vec<Mapping>,
    time: usize
}

impl Session {
    pub fn new(g: Graph, m: Vec<Mapping>, n: usize) -> Self {
        Session {
            graph: g,
            mappings: m,
            time: n
        }
    }

    fn backtrack(&self, state: State, visited: &mut HashSet<usize>, memo: &mut HashMap<State, usize>) -> usize {
        if visited.len() == self.mappings.len() {
            return 0;
        }
        if memo.contains_key(&state) {
            return *memo.get(&state).unwrap();
        }
        let (cur_node, cur_time, cur_flow) = state;
        if cur_time >= self.time {
            if cur_time == self.time {
                memo.insert(state, cur_flow);
                return cur_flow;
            }
            return 0;
        }
        let mut max_val = cur_flow;
        let next_nodes = self.graph.get(&cur_node).unwrap();
        // Option-1 -> if cur_node is not visited, you can spend one sec and open it
        if !visited.contains(&cur_node) {
            visited.insert(cur_node);
            let new_flow_rate = cur_flow + (self.time - (cur_time + 1)) * self.mappings[cur_node].flow_rate;
            //println!("state: {:?}, flow_rate {}, Added flow rate: {}", state, self.mappings[cur_node].flow_rate, new_flow_rate);
            max_val = max(max_val, self.backtrack((cur_node, cur_time + 1, new_flow_rate), visited, memo));
            visited.remove(&cur_node);
        }
        // Option-2 -> use the second to go to the neighboring nodes
        for node in next_nodes {
            max_val = max(max_val, self.backtrack((*node, cur_time + 1, cur_flow), visited, memo));
        }
        //visited.remove(&cur_node);
        memo.insert(state, max_val);
        max_val
    }

    pub fn get_max_pressure(&self) -> usize {
        /// Map of (tun_id, cur_time, cur_flow) -> flow_rate
        let mut memo: HashMap<(usize, usize, usize), usize> = HashMap::new();
        let mut visited: HashSet<usize> = HashSet::new();
        let mut max_val = 0;
        let mut aa_node = 0;
        for (i, m) in self.mappings.iter().enumerate() {
            if &m.tunnel == "AA" {
                aa_node = i;
                break;
            }
        }
        self.backtrack((aa_node, 0, 0), &mut visited, &mut memo)
    }

    pub fn print_mappings(&self) {
        for (i, val) in self.mappings.iter().enumerate() {
            let nodes = self.graph.get(&i).unwrap();
            let actuals: Vec<String> = nodes.into_iter().map(|v| self.mappings[*v].tunnel.to_owned()).collect();
            println!("Node id: {}, tunnel: {:?}, leads_to: {:?}", i, val.tunnel, actuals);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (graph, mappings) = parse_input(read_input("in.test").unwrap());
        let session = Session::new(graph, mappings, 30);
        let part1 = session.get_max_pressure();
        //session.print_mappings();
        println!("Test 1: {}", part1);
    }

    #[test]
    fn actual() {
        let (graph, mappings) = parse_input(read_input("in.1").unwrap());
        let session = Session::new(graph, mappings, 30);
        let part1 = session.get_max_pressure();
        //session.print_mappings();
        println!("Part 1: {}", part1);
    }
}
