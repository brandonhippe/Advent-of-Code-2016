use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use regex::Regex;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Node {
    x: i64,
    y: i64,
    size: i64,
    used: i64,
    avail: i64,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    fn new(node_str: &str) -> Node {
        let caps = Regex::new(r"node-x(?P<x>\d+)-y(?P<y>\d+)\s*(?P<size>\d+)T\s+(?P<used>\d+)T\s+(?P<avail>\d+)T\s+")
            .unwrap().captures(node_str).unwrap();
        Node {
            x: caps["x"].parse::<i64>().unwrap(),
            y: caps["y"].parse::<i64>().unwrap(),
            size: caps["size"].parse::<i64>().unwrap(),
            used: caps["used"].parse::<i64>().unwrap(),
            avail: caps["avail"].parse::<i64>().unwrap(),
        }
    }
}

fn manhat_dist(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

#[derive(Clone, Debug, Hash, Eq)]
struct GraphNode {
    pos: (i64, i64),
    end: (i64, i64),
    steps: i64,
}

impl Ord for GraphNode {
    fn cmp(&self, other: &Self) -> Ordering {
        (manhat_dist(other.pos, other.end) + other.steps).cmp(&(manhat_dist(self.pos, self.end) + self.steps))
    }
}

impl PartialOrd for GraphNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for GraphNode {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

fn valid_moves(curr_node: &GraphNode, valid_positions: &HashSet<(i64, i64)>) -> Vec<GraphNode> {
    let mut next_nodes: Vec<GraphNode> = Vec::new();
    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let x = curr_node.pos.0 + (dx as i64);
        let y = curr_node.pos.1 + (dy as i64);

        if valid_positions.contains(&(x, y)) {
            next_nodes.push(GraphNode {
                pos: (x, y),
                end: curr_node.end,
                steps: curr_node.steps + 1,
            });
        }
    }

    return next_nodes;
}

fn get_steps(nodes: Vec<Node>, data_start: (i64, i64), data_end: (i64, i64)) -> Option<i64> {
    let avg_log10 = nodes.iter().map(|n| (n.used as f64 + 1.0).log10().ceil() as i64).sum::<i64>() / (nodes.len() as i64);
    let valid_positions: HashSet<(i64, i64)> = HashSet::from_iter(nodes.iter().filter_map(|n| {
        if ((n.used as f64 + 1.0).log10().ceil() as i64) <= avg_log10 {
            Some((n.x, n.y))
        } else {
            None
        }
    }));

    let mut open_list: BinaryHeap<GraphNode> = BinaryHeap::from([GraphNode {
        pos: data_start.clone(),
        end: data_end.clone(),
        steps: 0,
    }]);
    let mut visited: HashSet<GraphNode> = HashSet::new();
    
    while let Some(node) = open_list.pop() {
        if node.pos == node.end {
            return Some(node.steps);
        }

        if visited.contains(&node) && visited.get(&node).unwrap().steps <= node.steps {
            continue;
        }
        visited.insert(node.clone());

        for n in valid_moves(&node, &valid_positions).into_iter().filter(|n| {
            !visited.contains(n) || n.steps < visited.get(n).unwrap().steps
        }) {
            open_list.push(n);
        }
    }
    
    return None;
}

fn part1(contents: String) -> i64 {
    let nodes: Vec<Node> = Vec::from_iter(contents.lines().skip(2).filter_map(|l| {
        let line = l.trim();
        if line.len() > 0 {
            Some(Node::new(l))
        } else {
            None
        }
    }));
    let avg_log10 = nodes.iter().map(|n| (n.used as f64 + 1.0).log10().ceil() as i64).sum::<i64>() / (nodes.len() as i64);
    return nodes.iter().filter(|n| {
        ((n.used as f64 + 1.0).log10().ceil() as i64) <= avg_log10
    }).count() as i64 - 1;
}

fn part2(contents: String) -> i64 {
    let mut start_nodes: Vec<Node> = Vec::from_iter(contents.lines().skip(2).filter_map(|l| {
        let line = l.trim();
        if line.len() > 0 {
            Some(Node::new(l))
        } else {
            None
        }
    }));
    start_nodes.sort();

    let min_node = start_nodes.iter().min_by_key(|n| n.used).unwrap();    
    let mut goal_pos: (i64, i64) = start_nodes.iter().filter_map(|n| {if n.y == 0 {Some((n.x, n.y))} else {None}}).max_by_key(|n| n.0).unwrap();
    let mut steps: i64 = get_steps(start_nodes.clone(), (min_node.x, min_node.y), goal_pos).unwrap();

    while goal_pos != (1, 0) {
        steps += 5;
        goal_pos = (goal_pos.0 - 1, goal_pos.1);
    }

    return steps;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 7);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2016".to_string();
    let day = "22".to_string();

    let root = env::current_dir().unwrap();
    let path_str = if args.len() > 1 {
        args[1].clone()
    } else if root.ends_with(format!("{}", day)) {
        format!("../../../Inputs/{}_{}.txt", year, day)
    } else {
        format!("/Inputs/{}_{}.txt", year, day)
    };

    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nViable pairs of nodes: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSteps to access data: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}