use crate::monkey::{Monkey, Item};

use std::collections::VecDeque;

pub fn get_actual_monkeys() -> Vec<Monkey> {
    vec![
        Monkey::new(
            VecDeque::from([54, 89, 94]), 
            Box::new(|old: Item| -> Item { old * 7 }),
            Box::new(|val: Item| -> usize { 
                match val % 17 == 0 {
                    true => 5,
                    false => 3
                }
            })
        ),
        Monkey::new(
            VecDeque::from([66, 71]), 
            Box::new(|old: Item| -> Item { old + 4 }),
            Box::new(|val: Item| -> usize { 
                match val % 3 == 0 {
                    true => 0,
                    false => 3
                }
            })
        ),
        Monkey::new(
            VecDeque::from([76, 55, 80, 55, 55, 96, 78]), 
            Box::new(|old: Item| -> Item { old + 2 }),
            Box::new(|val: Item| -> usize { 
                match val % 5 == 0 {
                    true => 7,
                    false => 4
                }
            })
        ),
        Monkey::new(
            VecDeque::from([93, 69, 76, 66, 89, 54, 59, 94]), 
            Box::new(|old: Item| -> Item { old + 7 }),
            Box::new(|val: Item| -> usize { 
                match val % 7 == 0 {
                    true => 5,
                    false => 2
                }
            })
        ),
        Monkey::new(
            VecDeque::from([80, 54, 58, 75, 99]), 
            Box::new(|old: Item| -> Item { old * 17 }),
            Box::new(|val: Item| -> usize { 
                match val % 11 == 0 {
                    true => 1,
                    false => 6
                }
            })
        ),
        Monkey::new(
            VecDeque::from([69, 70, 85, 83]), 
            Box::new(|old: Item| -> Item { old + 8 }),
            Box::new(|val: Item| -> usize { 
                match val % 19 == 0 {
                    true => 2,
                    false => 7
                }
            })
        ),
        Monkey::new(
            VecDeque::from([89]), 
            Box::new(|old: Item| -> Item { old + 6 }),
            Box::new(|val: Item| -> usize { 
                match val % 2 == 0 {
                    true => 0,
                    false => 1
                }
            })
        ),
        Monkey::new(
            VecDeque::from([62, 80, 58, 57, 93, 56]), 
            Box::new(|old: Item| -> Item { old * old }),
            Box::new(|val: Item| -> usize { 
                match val % 13 == 0 {
                    true => 6,
                    false => 4
                }
            })
        ),
    ]
}