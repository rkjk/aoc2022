use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::cmp::min;

const TOT_DISK_SPACE: usize = 70000000;
const FREE_SPACE_NEEDED: usize = 30000000;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

#[derive(Debug)]
struct AFile {
    name: String,
    size: usize,
}

impl AFile {
    pub fn new(name: &str, size: usize) -> Self {
        AFile {
            name: name.to_string(),
            size: size
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
}

#[derive(Debug)]
struct Folder {
    name: String,
    files: Vec<AFile>,
    children: HashMap<String, Rc<RefCell<Folder>>>,
    parent: Weak<RefCell<Folder>>,
    size: usize
}

impl Folder {
    pub fn new(name: &str, parent: Weak<RefCell<Folder>>) -> Rc<RefCell<Self>> {
        Rc::new(
            RefCell::new(
                Folder {
                    name: name.to_string(),
                    files: vec![],
                    children: HashMap::new(),
                    parent: parent.clone(),
                    size: 0
                }
            )
        )
    }

    pub fn add_file(&mut self, name: &str, size: usize) {
        self.files.push(AFile::new(name, size));
    }

    pub fn add_directory(&mut self, name: &str, parent: Weak<RefCell<Folder>>) {
        if self.children.contains_key(name) {
            return;
        }
        self.children.insert(name.to_string(), Folder::new(name, parent));
    }

    pub fn get_parent(&self) -> Weak<RefCell<Folder>> {
        self.parent.clone()
    }

    pub fn get_child(&self, name: &str) -> Rc<RefCell<Folder>> {
        self.children.get(name).unwrap().clone()
    }

    pub fn update_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn get_size(&mut self) -> usize {
        let mut size = 0;
        for file in self.files.iter() {
            size += file.get_size();
        }
        for dir in self.children.values() {
            size += dir.borrow_mut().get_size();
        }
        //println!("{} {}", self.name, size);
        self.size = size;
        size
    }

    pub fn get_num_below_100000(&self) -> usize {
        let mut val = match self.size <= 100000 {
            true => self.size,
            false => 0
        };
        for dir in self.children.values() {
            val += dir.borrow().get_num_below_100000();
        }
        val
    }

    pub fn get_smallest_to_delete(&self, required_space: usize) -> usize {
        if self.size < required_space {
            return usize::MAX;
        }
        if self.size == required_space {
            return self.size;
        }
        let mut val = self.size;
        for dir in self.children.values() {
            val = min(val, dir.borrow().get_smallest_to_delete(required_space));
        }
        val
    }
}

struct Session {
    root_dir: Rc<RefCell<Folder>>
}

impl Session {
    pub fn new(root: Rc<RefCell<Folder>>) -> Self {
        Session {
            root_dir: root.clone()
        }
    }

    pub fn construct_graph(&self, input: Vec<String>) {
        let mut cur_dir = Rc::clone(&self.root_dir);
        for val in input.into_iter() {
            if !val.starts_with("$") {
                if !val.starts_with("dir") {
                    let line: Vec<&str> = val.split(" ").collect();
                    cur_dir.as_ref().borrow_mut().add_file(line[1], line[0].parse::<usize>().unwrap());
                } else {
                    let line: Vec<&str> = val.split(" ").collect();
                    cur_dir.as_ref().borrow_mut().add_directory(line[1], Rc::downgrade(&cur_dir));
                }
            } else {
                let command_line: Vec<&str> = val.split(" ").collect();
                if command_line[1] == "ls" {
                    continue;
                }
                let dir = command_line[2];
                if dir == ".." {
                    let next_dir = cur_dir.as_ref().borrow().get_parent().upgrade().unwrap();
                    cur_dir = next_dir;
                    continue;
                }
                cur_dir.as_ref().borrow_mut().add_directory(dir, Rc::downgrade(&cur_dir));
                let next_dir = cur_dir.as_ref().borrow().get_child(dir);
                cur_dir = next_dir;
            }
        }
    }

    pub fn get_size(&self) -> usize {
        self.root_dir.borrow_mut().get_size()
    }

    pub fn get_num_below_100000(&self) -> usize {
        self.root_dir.borrow().get_num_below_100000()
    }

    pub fn get_smallest_to_delete(&self, required_space: usize) -> usize {
        self.root_dir.borrow().get_smallest_to_delete(required_space)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = read_input("in.test").unwrap();
        let root = Folder::new("", Weak::new());
        let session = Session::new(root);
        session.construct_graph(input);
        let total_size = session.get_size();
        let unused_space = TOT_DISK_SPACE - total_size;
        let required_space = FREE_SPACE_NEEDED - unused_space;
        let part1 = session.get_num_below_100000();
        let part2 = session.get_smallest_to_delete(required_space);
        println!("Test 1: {}", part1);
        println!("Test 2: {}", part2);
    }

    #[test]
    fn actual() {
        use std::time::Instant;
        let now = Instant::now();
        let input = read_input("in.1").unwrap();
        let root = Folder::new("", Weak::new());
        let session = Session::new(root);
        session.construct_graph(input);
        let total_size = session.get_size();
        let unused_space = TOT_DISK_SPACE - total_size;
        let required_space = FREE_SPACE_NEEDED - unused_space;
        let part1 = session.get_num_below_100000();
        let part2 = session.get_smallest_to_delete(required_space);
        let part1 = session.get_num_below_100000();
        let part2 = session.get_smallest_to_delete(required_space);
        let elapsed = now.elapsed();
        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2);
        println!("Elapsed: {:.2?}", elapsed);
    }
}
