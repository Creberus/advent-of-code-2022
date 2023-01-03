use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::io::{self, Write};

pub fn main_p2() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut nodes = HashSet::new();
    let mut edges = HashSet::new();

    for line in lines {
        let line = line.unwrap();

        let (node, node_edges) = parse_valve(line).unwrap();

        nodes.insert(node);

        for node_edge in node_edges {
            edges.insert(node_edge);
        }
    }

    write_graph("graph_1_parsed.dot", &edges)?;

    let nodes_to_remove = simplify_graph(&nodes, &mut edges);

    // Remove useless nodes with flow_rate=0
    for node in nodes_to_remove {
        nodes.remove(&node);
    }

    write_graph("graph_2_simplified.dot", &edges)?;

    // Modify the Graph to make all nodes linked to each others.
    // This way we can do a simple BFS to find the best path.
    //
    edges = link_all(&nodes, &mut edges);

    write_graph("graph_3_all_linked.dot", &edges)?;

    // Now perform a BFS to find the best path.
    // We need to brute force and test every possibilities but this shouldn't take too long thanks
    // to previus steps.

    let mut possible_paths = find_most_pressure(&nodes, &edges);

    println!("Number of paths: {}", possible_paths.len());

    possible_paths.sort_by(|a, b| compute_pression(b, 26).cmp(&compute_pression(a, 26)));

    println!("Possible paths has been sorted");

    let total_paths = possible_paths.len();
    let mut current_path = 0;

    let mut max_pression = 0;

    let mut paths_it = possible_paths.iter();
    loop {
        if let Some(path) = paths_it.next() {
            let mut next_path = paths_it.clone();

            print!(
                "Max pression: {} | Remaining: {}/{}\r",
                max_pression, current_path, total_paths
            );
            std::io::stdout().flush().unwrap();

            loop {
                if let Some(next_path) = next_path.next() {
                    match combine_paths(&path, &next_path, nodes.len() - 1) {
                        Some(pression) => max_pression = max_pression.max(pression),
                        None => (),
                    }
                } else {
                    break;
                }
            }

            current_path += 1;
        } else {
            break;
        }
    }

    println!("Max pression: {}", max_pression);

    Ok(())
}

fn compute_pression(path: &Vec<(String, usize, usize)>, minutes: usize) -> usize {
    let mut pression = 0;
    for (_, min, p) in path {
        pression += p * (minutes - *min as usize);
    }
    pression
}

fn combine_paths(
    a: &Vec<(String, usize, usize)>,
    b: &Vec<(String, usize, usize)>,
    number_of_valves: usize,
) -> Option<usize> {
    let mut minutes = 0;

    let mut error = false;
    let mut valve_opened = Vec::<String>::new();
    let mut new_path = Vec::<(String, usize, usize)>::new();

    while minutes < 26 && valve_opened.len() != number_of_valves {
        let valve_a = a.iter().find(|p| p.1 == minutes);
        let valve_b = b.iter().find(|p| p.1 == minutes);

        match (valve_a, valve_b) {
            (Some(a), Some(b)) => {
                if !valve_opened.contains(&a.0) {
                    valve_opened.push(a.0.clone());
                    new_path.push(a.clone());
                } else {
                    error = true;
                    break;
                }
                if !valve_opened.contains(&b.0) {
                    valve_opened.push(b.0.clone());
                    new_path.push(b.clone());
                } else {
                    error = true;
                    break;
                }
            }
            (Some(a), None) | (None, Some(a)) => {
                if !valve_opened.contains(&a.0) {
                    valve_opened.push(a.0.clone());
                    new_path.push(a.clone());
                } else {
                    error = true;
                    break;
                }
            }
            _ => (),
        }

        minutes += 1;
    }

    if error {
        None
    } else {
        Some(compute_pression(&new_path, 26))
    }
}

fn simplify_graph(nodes: &HashSet<Node>, edges: &mut HashSet<Edge>) -> Vec<Node> {
    let mut nodes_to_remove = Vec::<Node>::new();

    // Reduce the graph by removing the node with flow_rate of 0.
    for node in nodes {
        if node.flow_rate() != 0 || *node.label() == String::from("AA") {
            continue;
        }

        nodes_to_remove.push(node.clone());

        let node_edges: Vec<Edge> = edges
            .iter()
            .filter_map(|e| {
                if e.a() == node.label() || e.b() == node.label() {
                    Some(e.clone())
                } else {
                    None
                }
            })
            .collect();

        // Remove the edges from the Edgelist
        for node_edge in &node_edges {
            edges.remove(node_edge);
        }

        // Now create new edges between elements that were connected
        if edges.len() <= 1 {
            continue; // Do nothing, the node will be suppressed from the graph.
        } else {
            for (index, node_edge) in node_edges.iter().enumerate() {
                let a = if node_edge.a() == node.label() {
                    node_edge.b()
                } else {
                    node_edge.a()
                };

                for next_nodes in index + 1..node_edges.len() {
                    let b_node = &node_edges[next_nodes];
                    let b = if b_node.a() == node.label() {
                        b_node.b()
                    } else {
                        b_node.a()
                    };

                    edges.insert(Edge::new(
                        a.clone(),
                        b.clone(),
                        node_edge.weight() + b_node.weight(),
                    ));
                }
            }
        }
    }

    nodes_to_remove
}

fn link_all(nodes: &HashSet<Node>, edges: &mut HashSet<Edge>) -> HashSet<Edge> {
    let mut linked_edges = HashSet::new();

    for node in nodes {
        for other in nodes {
            if node == other {
                continue;
            }

            let time_cost = find_shortest_time(&node, &other, edges);

            if let Ok(time_cost) = time_cost {
                linked_edges.insert(Edge::new(
                    node.label().clone(),
                    other.label().clone(),
                    time_cost,
                ));
            } else {
                panic!("Path not found between {:?} and {:?}", node, other);
            }
        }
    }

    linked_edges
}

fn find_shortest_time(a: &Node, b: &Node, edges: &HashSet<Edge>) -> Result<usize, ()> {
    let mut queue = VecDeque::<STNode>::new();

    queue.push_back(STNode::from(a));

    while !queue.is_empty() {
        let mut node = queue.pop_front().unwrap();

        node.visit();

        if node.label() == b.label() {
            return Ok(node.time_cost());
        }

        let neighbors: Vec<Edge> = edges
            .iter()
            .filter_map(|e| {
                if e.a() == node.label() || e.b() == node.label() {
                    Some(e.clone())
                } else {
                    None
                }
            })
            .collect();

        for neighbor in neighbors {
            let neighbor_label = if neighbor.a() == node.label() {
                neighbor.b()
            } else {
                neighbor.a()
            };

            let mut n = STNode::new(neighbor_label.clone(), Some(&node));

            n.add_time(neighbor.weight());

            queue.push_back(n);
        }
    }

    // Not found
    Err(())
}

#[derive(Debug)]
struct STNode {
    label: String,
    time_cost: usize,
    visited: Vec<String>,
}

impl STNode {
    fn new(label: String, other: Option<&STNode>) -> Self {
        match other {
            Some(other) => Self {
                label,
                time_cost: other.time_cost,
                visited: other.visited.clone(),
            },
            None => Self {
                label,
                time_cost: 0,
                visited: Vec::new(),
            },
        }
    }

    fn label(&self) -> &String {
        &self.label
    }

    fn time_cost(&self) -> usize {
        self.time_cost
    }

    fn visited(&self) -> &Vec<String> {
        &self.visited
    }

    fn add_time(&mut self, time: usize) {
        self.time_cost += time
    }

    fn visit(&mut self) {
        self.visited.push(self.label.clone())
    }
}

impl From<&Node> for STNode {
    fn from(node: &Node) -> Self {
        Self {
            label: node.label().clone(),
            time_cost: 0,
            visited: Vec::new(),
        }
    }
}

fn find_most_pressure(
    nodes: &HashSet<Node>,
    edges: &HashSet<Edge>,
) -> Vec<Vec<(String, usize, usize)>> {
    let mut queue = VecDeque::<MPNode>::new();

    let mut paths = Vec::new();

    queue.push_back(MPNode::new(String::from("AA"), None));

    while !queue.is_empty() {
        let mut node = queue.pop_front().unwrap();
        let graph_node = nodes.get(&Node::new(node.label().clone(), 0)).unwrap();

        node.visit();

        // Check if we run out of time
        if node.minutes() >= 26 {
            paths.push(node.valve_opened().clone());
            continue;
        }

        // Open the valve if we are not on starting node
        if node.label() != &String::from("AA") {
            // Always open valve for the node we are in
            node.increase_min(1);
            node.open_valve(graph_node.flow_rate());
        }

        // Check if we visited all nodes or we run out of time
        if node.visited().len() == nodes.len() || node.minutes() >= 26 {
            paths.push(node.valve_opened().clone());
            continue;
        }

        for neighbor in nodes {
            if neighbor.label() == node.label() || node.visited().contains(neighbor.label()) {
                continue;
            }

            let edge = edges
                .get(&Edge::new(
                    node.label().clone(),
                    neighbor.label().clone(),
                    0,
                ))
                .unwrap();

            let mut n = MPNode::new(neighbor.label().clone(), Some(&node));

            n.increase_min(edge.weight());

            queue.push_back(n);
        }
    }

    paths
}

#[derive(Debug, Eq, Ord)]
struct MPNode {
    label: String,
    minutes: usize,
    visited: Vec<String>,
    valve_opened: Vec<(String, usize, usize)>,
}

impl MPNode {
    fn new(label: String, other: Option<&MPNode>) -> Self {
        match other {
            Some(other) => Self {
                label,
                minutes: other.minutes,
                visited: other.visited.clone(),
                valve_opened: other.valve_opened.clone(),
            },
            None => Self {
                label,
                minutes: 0,
                visited: Vec::new(),
                valve_opened: Vec::new(),
            },
        }
    }

    fn label(&self) -> &String {
        &self.label
    }

    fn minutes(&self) -> usize {
        self.minutes
    }

    fn visited(&self) -> &Vec<String> {
        &self.visited
    }

    fn valve_opened(&self) -> &Vec<(String, usize, usize)> {
        &self.valve_opened
    }

    fn visit(&mut self) {
        self.visited.push(self.label.clone())
    }

    fn open_valve(&mut self, pression: usize) {
        self.valve_opened
            .push((self.label.clone(), self.minutes, pression))
    }

    fn increase_min(&mut self, min: usize) {
        self.minutes += min
    }

    fn pression(&self) -> usize {
        self.current_pression(26)
    }

    fn current_pression(&self, minutes: usize) -> usize {
        let mut pression = 0;
        for (_, min, p) in &self.valve_opened {
            pression += p * (minutes - *min as usize);
        }
        pression
    }
}

impl PartialEq for MPNode {
    fn eq(&self, other: &Self) -> bool {
        self.pression() == other.pression()
    }
}

impl PartialOrd for MPNode {
    fn lt(&self, other: &Self) -> bool {
        self.pression() < other.pression()
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn write_graph(path: &str, edges: &HashSet<Edge>) -> Result<(), Box<dyn Error>> {
    let mut dot_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    dot_file.write("graph Day16 {\n".as_bytes())?;

    for edge in edges {
        dot_file.write(
            format!(
                "\t{} -- {} [label={}];\n",
                edge.a(),
                edge.b(),
                edge.weight()
            )
            .as_bytes(),
        )?;
    }

    dot_file.write("}".as_bytes())?;

    Ok(())
}

fn parse_valve(s: String) -> Result<(Node, Vec<Edge>), ()> {
    let line: Vec<&str> = s.split(';').collect();

    let valve_data: Vec<&str> = line[0].split(' ').collect();
    let (_, mut tunnels_data) = line[1].split_at(23);
    if tunnels_data.chars().nth(0).unwrap() == ' ' {
        (_, tunnels_data) = tunnels_data.split_at(1);
    }

    let name = valve_data[1];
    let flow_rate = valve_data[4];
    let (_, flow_rate) = flow_rate.split_at(5);
    let flow_rate = flow_rate.parse().unwrap();

    let node = Node::new(String::from(name), flow_rate);

    let tunnels: Vec<&str> = tunnels_data.split(", ").collect();

    let mut edges = Vec::new();

    for tunnel in tunnels {
        let edge = Edge::new(String::from(name), String::from(tunnel), 1);
        edges.push(edge);
    }

    Ok((node, edges))
}

// Node
#[derive(Debug, Eq, Clone)]
struct Node {
    label: String,
    flow_rate: usize,
}

impl Node {
    fn new(label: String, flow_rate: usize) -> Self {
        Node { label, flow_rate }
    }

    fn label(&self) -> &String {
        &self.label
    }

    fn flow_rate(&self) -> usize {
        self.flow_rate
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.label.hash(state)
    }
}

// Edge
#[derive(Debug, Eq, Clone)]
struct Edge {
    a: String,
    b: String,
    weight: usize,
}

impl Edge {
    fn new(a: String, b: String, weight: usize) -> Self {
        Edge { a, b, weight }
    }

    fn a(&self) -> &String {
        &self.a
    }

    fn b(&self) -> &String {
        &self.b
    }

    fn weight(&self) -> usize {
        self.weight
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a)
    }
}

impl Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.a <= self.b {
            self.a.hash(state);
            self.b.hash(state);
        } else {
            self.b.hash(state);
            self.a.hash(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valve() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";

        let (node, edges) = parse_valve(input.to_string()).unwrap();

        let tunnels = Vec::from([
            Edge::new(String::from("AA"), String::from("DD"), 1),
            Edge::new(String::from("AA"), String::from("II"), 1),
            Edge::new(String::from("AA"), String::from("BB"), 1),
        ]);

        assert_eq!(*node.label(), "AA".to_string());
        assert_eq!(node.flow_rate(), 0);

        assert!(edges.iter().eq(&tunnels));
    }
}
