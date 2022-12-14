pub mod example;
pub mod monkey;
pub mod actual;

use monkey::Monkey;

struct Session {
    monkeys: Vec<Monkey>,
    counts: Vec<usize>
}

impl Session {
    pub fn new(monkeys: Vec<Monkey>) -> Self {
        let size = monkeys.len();
        Session {
            monkeys: monkeys,
            counts: vec![0; size]
        }
    }

    fn run_turn(&mut self, index: usize, should_worry: bool) {
        let size = self.monkeys.len();
        let monkey_ptr: *mut Monkey = &mut self.monkeys[index];
        unsafe {
            let monkey_ref = monkey_ptr.as_mut().unwrap();
            match monkey_ref.is_empty() {
                true => return,
                false => {
                    self.counts[index] += monkey_ref.items.len();
                    while !monkey_ref.items.is_empty() {
                        let v = monkey_ref.items.pop_front().unwrap();
                        let worry = (monkey_ref.worry_fn)(v);
                        let worry = match should_worry {
                            true => worry % 96577,
                            false => worry / 3
                        };
                        let destination = (monkey_ref.test_fn)(worry);
                        if destination < 0 || destination > size - 1 {
                            panic!("destination exceeds size: {}", destination);
                        }
                        let dest_ptr: *mut Monkey = &mut self.monkeys[destination];
                        dest_ptr.as_mut().unwrap().items.push_back(worry);
                    }
                }
            }
        }
    }

    pub fn run_round(&mut self, should_worry: bool) {
        let size = self.monkeys.len();
        for i in 0..size {
            self.run_turn(i, should_worry);
        }
    }

    pub fn test(&mut self) {
        self.run_round(false);
        for i in 0..self.monkeys.len() {
            self.monkeys[i].print();
        }
        println!("Counts: {:?}", self.counts);
    }

    pub fn run_rounds(&mut self, num_rounds: usize, should_worry: bool) -> usize {
        for i in 0..num_rounds {
            self.run_round(should_worry);
        }
        let mut counts = self.counts.clone();
        counts.sort();
        counts[counts.len() -1] * counts[counts.len() - 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use example::*;
        let inp = get_example_monkeys();
        let mut session = Session::new(inp);
        let part1 = session.run_rounds(20, false);
        let part2 = session.run_rounds(10000, true);
        println!("Test 1: {}", part1);
        println!("Test 2: {}", part2);
    }

    //#[test]
    fn actual() {
        use actual::*;
        let inp = get_actual_monkeys();
        let mut session = Session::new(inp);
        let part1 = session.run_rounds(20, false);
        let part2 = session.run_rounds(10000, false);
        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2);
    }
}
