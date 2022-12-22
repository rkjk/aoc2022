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

/// (current_tunnel, current_elephant_tunnel, cur_time, flow_rate achieved so far)
type DoubleState = (usize, usize, usize, usize);

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
    time: usize,
    visited_nodes: HashSet<usize>,
    cur_max: usize,
}

impl Session {
    pub fn new(g: Graph, m: Vec<Mapping>, n: usize) -> Self {
        Session {
            graph: g,
            mappings: m,
            time: n,
            visited_nodes: HashSet::new(),
            cur_max: 0
        }
    }

    fn backtrack(&mut self, state: State, visited: &mut HashSet<usize>, memo: &mut HashMap<State, usize>) -> usize {
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
                if cur_flow > self.cur_max {
                    self.cur_max = cur_flow;
                    self.visited_nodes = visited.clone();
                }
                return cur_flow;
            }
            return 0;
        }
        let mut max_val = cur_flow;
        let next_nodes = self.graph.get(&cur_node).unwrap().clone();
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
            max_val = max(max_val, self.backtrack((node, cur_time + 1, cur_flow), visited, memo));
        }
        //visited.remove(&cur_node);
        if max_val > self.cur_max {
            self.cur_max = cur_flow;
            self.visited_nodes = visited.clone();
        }
        memo.insert(state, max_val);
        max_val
    }

    // Works but is too slow for part 2. This will exhaustively check all combinations
    fn backtrack_doublestate(&self, state: DoubleState, visited: &mut HashSet<usize>, memo: &mut HashMap<DoubleState, usize>) -> usize {
        //println!("State: {:?}, visited: {:?}", state, visited);
        if visited.len() == self.mappings.len() {
            return 0;
        }
        if memo.contains_key(&state) {
            return *memo.get(&state).unwrap();
        }
        let (cn1, cn2, cur_time, cur_flow) = state;
        if cur_time >= self.time {
            if cur_time == self.time {
                memo.insert(state, cur_flow);
                return cur_flow;
            }
            memo.insert(state, 0);
            return 0;
        }
        let mut max_val = cur_flow;
        let (nn1_nodes, nn2_nodes) = (self.graph.get(&cn1).unwrap(), self.graph.get(&cn2).unwrap());
        let nf1 = (self.time - (cur_time + 1)) * self.mappings[cn1].flow_rate;
        let nf2 = (self.time - (cur_time + 1)) * self.mappings[cn2].flow_rate;
        // Option-0: If both elephant and man in same room,
        // room is visited and only one tunnel from here, then illegal state
        if cn1 == cn2 && visited.contains(&cn1) && nn1_nodes.len() == 1 {
            memo.insert(state, 0);
            return 0;
        }
        // Option-1: elephant and man in different rooms, and both rooms unvisited
        // So, both open
        if !visited.contains(&cn1) && !visited.contains(&cn2) && cn1 != cn2 {
            visited.insert(cn1);
            visited.insert(cn2);
            let new_flow_rate = cur_flow + nf1 + nf2;
            max_val = max(max_val, self.backtrack_doublestate((cn1, cn2, cur_time + 1, new_flow_rate), visited, memo));
            visited.remove(&cn1);
            visited.remove(&cn2);
        }
        // Option-2 one of elephant and man open, and the other moves on
        // This can happen if
        //  1. Both in different rooms, and at least one of them is unvisited
        //  2. Both in same room, room is unvisited and Room is not terminal -> take turns to visit
        // Both cases should be covered by the following two if blocks.
        if !visited.contains(&cn1) {
            visited.insert(cn1);
            for nn2 in nn2_nodes {
                max_val = max(max_val, self.backtrack_doublestate((cn1, *nn2, cur_time + 1, cur_flow + nf1), visited, memo));
            }
            visited.remove(&cn1);
        }
        if !visited.contains(&cn2) {
            visited.insert(cn2);
            for nn1 in nn1_nodes {
                max_val = max(max_val, self.backtrack_doublestate((*nn1, cn2, cur_time + 1, cur_flow + nf2), visited, memo));
            }
            visited.remove(&cn2);
        }
        // Option-3: No one opens
        for nn1 in nn1_nodes.into_iter() {
            for nn2 in nn2_nodes.into_iter() {
                if cn1 == cn2 && nn1 == nn2 {
                    continue;
                }
                max_val = max(max_val, self.backtrack_doublestate((*nn1, *nn2, cur_time + 1, cur_flow), visited, memo));
            }
        }
        memo.insert(state, max_val);
        max_val
    }

    pub fn get_max_pressure(&mut self) -> usize {
        /// Map of (tun_id, cur_time, cur_flow) -> flow_rate
        let mut memo: HashMap<(usize, usize, usize), usize> = HashMap::new();
        let mut visited: HashSet<usize> = HashSet::new();
        let mut aa_node = 0;
        for (i, m) in self.mappings.iter().enumerate() {
            if &m.tunnel == "AA" {
                aa_node = i;
                break;
            }
        }
        self.backtrack((aa_node, 0, 0), &mut visited, &mut memo)
    }

    /// Maha-Bongu
    /// So, this works only for the actual input and not the example
    /// Run the backtracking once with N=26 (set cur_time = 4).
    /// Then, for al the visited valves, set flow_rate to 0, then rn backtracking again
    /// Assumes the two processes are independent/linear. 
    /// backtrack_doublestate is correct and will work, but will take hours
    /// not to mention I don't think my machine is powerful enough or enough RAM.
    /// Right way -> do a DFS for man -> compute max_val and visited. 
    /// Followed immmeditately by DFS for elephant with a complementary set of nodes
    /// max of sum of these two is your memo table storage.
    pub fn get_max_pressure_with_elephant(&mut self) -> usize {
        let mut memo: HashMap<State, usize> = HashMap::new();
        let mut visited: HashSet<usize> = HashSet::new();
        let mut aa_node = 0;
        for (i, m) in self.mappings.iter().enumerate() {
            if &m.tunnel == "AA" {
                aa_node = i;
                break;
            }
        }
        let val1 = self.backtrack((aa_node, 4, 0), &mut visited, &mut memo);
        println!("Val1: {}, visited: {:?}", val1, self.visited_nodes);
        for i in self.visited_nodes.iter() {
            self.mappings[*i].flow_rate = 0;
        }
        //visited = self.visited_nodes.clone();
        visited = HashSet::new();
        self.visited_nodes = HashSet::new();
        self.cur_max = 0;
        memo = HashMap::new();
        let val2 = self.backtrack((aa_node, 4, 0), &mut visited, &mut memo);
        //println!("Mappings: {:?}", self.mappings);
        println!("Val2: {}, visited: {:?}", val2, self.visited_nodes);
        val1 + val2
    }

    pub fn get_visited(&self) -> HashSet<usize> {
        self.visited_nodes.clone()
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
        let mut session = Session::new(graph, mappings, 30);
        let part1 = session.get_max_pressure();
        // Doesn't work, use the backtrack_doublestate here
        let part2 = session.get_max_pressure_with_elephant();
        //session.print_mappings();
        println!("Test 1: {}", part1);
        println!("Test 2: {}", part2);
    }

    #[test]
    fn actual() {
        let (graph, mappings) = parse_input(read_input("in.1").unwrap());
        let mut session = Session::new(graph, mappings, 30);
        let part1 = session.get_max_pressure();
        let part2 = session.get_max_pressure_with_elephant();
        //session.print_mappings();
        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2);
    }
}
