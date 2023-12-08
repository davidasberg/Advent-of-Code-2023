use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fmt::{Display, Formatter},
    fs,
    rc::Rc,
};

fn main() {
    let input = fs::read_to_string("input/day08/day08.txt").expect("File not found!");
    let (dirs, roots) = read_input(&input);
    println!("dirs: {:?}", dirs);

    for root in roots.iter() {
        println!("root: {}", root.borrow());
    }

    // only needs first root
    let root = roots
        .iter()
        .filter(|node| {
            let node = node.borrow();
            node.label == "AAA"
        })
        .next()
        .unwrap();

    let part1 = part1(&dirs, &root, "ZZZ");
    println!("part1: {}", part1);

    let part2 = part2(&dirs, &roots, "Z");
    println!("part2: {}", part2);
}

fn part1(dirs: &Vec<Dir>, root: &Rc<RefCell<Node>>, dest: &str) -> u64 {
    let mut node = root.clone();
    let mut path_len = 0;

    for dir in dirs.iter().cycle() {
        match dir {
            Dir::Left => {
                let left = node.borrow().left.clone().unwrap();
                node = left;
            }
            Dir::Right => {
                let right = node.borrow().right.clone().unwrap();
                node = right;
            }
        }
        path_len += 1;

        if node.borrow().label.ends_with(dest) {
            break;
        }
    }

    path_len
}

fn part2(dirs: &Vec<Dir>, roots: &Vec<Rc<RefCell<Node>>>, dest: &str) -> u64 {
    let mut counts = Vec::new();
    for root in roots.iter() {
        let part1 = part1(dirs, root, dest);
        counts.push(part1);
    }

    // find lcm of all counts
    let lcm = counts.iter().fold(counts[0], |acc, &x| lcm(acc, x));
    return lcm;
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn read_input(input: &str) -> (Vec<Dir>, Vec<Rc<RefCell<Node>>>) {
    let dirs = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!("Invalid input!"),
        })
        .collect();

    let nodes: HashMap<String, (String, String)> = input
        .lines()
        .skip(2)
        .map(|line| {
            let mut parts = line.split(" = ");
            let label = parts.next().unwrap().to_string();
            let mut node = parts
                .next()
                .unwrap()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split(", ");

            let left = node.next().unwrap().to_string();
            let right = node.next().unwrap().to_string();

            (label, (left, right))
        })
        .collect();

    let roots = build_graph(nodes);

    (dirs, roots)
}

fn build_graph(nodes: HashMap<String, (String, String)>) -> Vec<Rc<RefCell<Node>>> {
    let mut queue: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
    let mut node_map: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();
    for (label, _) in nodes.iter() {
        let node = Rc::new(RefCell::new(Node {
            label: label.to_string(),
            left: None,
            right: None,
        }));

        queue.push_back(node.clone());
        node_map.insert(label.to_string(), node.clone());
    }

    while let Some(node) = queue.pop_front() {
        let mut node = node.borrow_mut();

        if let (Some(_), Some(_)) = (&node.left, &node.right) {
            continue;
        }

        let (l, r) = nodes.get(&node.label as &str).unwrap();

        if let Some(left) = node_map.get(l) {
            node.left = Some(left.clone());
        } else {
            let left = Rc::new(RefCell::new(Node {
                label: l.to_string(),
                left: None,
                right: None,
            }));

            node.left = Some(left.clone());
            node_map.insert(l.to_string(), left.clone());
            queue.push_back(left);
        }

        if let Some(right) = node_map.get(r) {
            node.right = Some(right.clone());
        } else {
            let right = Rc::new(RefCell::new(Node {
                label: r.to_string(),
                left: None,
                right: None,
            }));

            node.right = Some(right.clone());
            node_map.insert(r.to_string(), right.clone());
            queue.push_back(right);
        }
    }

    for (label, node) in node_map.iter() {
        println!("{}: {}", label, node.borrow());
    }

    // return all nodes that end in A, excluding the root node
    node_map
        .iter()
        .filter(|(_, node)| {
            let node = node.borrow();
            node.label.ends_with("A")
        })
        .map(|(_, node)| node.clone())
        .collect()
}

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Node {
    label: String,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)?;
        write!(f, "({})", self.left.as_ref().unwrap().borrow().label)?;
        write!(f, "({})", self.right.as_ref().unwrap().borrow().label)?;

        Ok(())
    }
}
