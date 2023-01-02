use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::io::{self, Write};

pub fn main_p1() -> Result<(), Box<dyn Error>> {
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

    let mut nodes_to_remove = Vec::<Node>::new();

    // Reduce the graph by removing the node with flow_rate of 0.
    for node in &nodes {
        if node.flow_rate() != 0 || *node.label() == String::from("AA") {
            continue;
        }

        println!("Removing node: {:?}", node);
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

    // Remove useless nodes with valve broken
    for node in nodes_to_remove {
        nodes.remove(&node);
    }

    write_graph("graph_2_simplified.dot", &edges)?;

    // Modify the Graph to make all nodes linked to each others.
    // This way we can do a simple BFS to find the best path.

    write_graph("graph_3_all_linked.dot", &edges)?;

    // Find path with highest flow rate
    let mut max_pression_per_minute = Vec::<usize>::new();
    for _ in 0..31 {
        max_pression_per_minute.push(0);
    }

    let mut node_per_min: Vec<Vec<BFSNode>> = Vec::new();
    for _ in 0..31 {
        node_per_min.push(Vec::new());
    }
    node_per_min[0].push(BFSNode::new(String::from("AA"), None));

    let mut minutes = 0;
    println!("Minute: {}", minutes);

    let mut paths = Vec::new();

    while minutes <= 30 {
        let bfs_node = node_per_min[minutes].pop().unwrap();

        if bfs_node.minutes() >= 30 {
            paths.push(bfs_node);
        } else if bfs_node.minutes() > 5
            && bfs_node.current_pression(minutes) < max_pression_per_minute[minutes - 5]
        {
            ();
        } else {
            max_pression_per_minute[minutes] =
                max_pression_per_minute[minutes].max(bfs_node.current_pression(minutes));

            let valve = Node::new(bfs_node.label().clone(), 0);
            let node = nodes.get(&valve).unwrap();

            // If all the valves are open, the node finished her journey
            if bfs_node.valves_open() == nodes.len() - 1 {
                paths.push(bfs_node);
            } else {
                // Check if the flow rate of the valve is higher than 0.
                if node.flow_rate() > 0 && !bfs_node.is_valve_opened(node.label()) {
                    let mut opened_node = BFSNode::new(node.label().clone(), Some(&bfs_node));

                    opened_node.increase_min(1);
                    opened_node.open_valve(node.flow_rate());

                    node_per_min[opened_node.minutes() as usize].push(opened_node);
                }

                // Add neighbors to queue
                for edge in &edges {
                    let neighbor = if edge.a() == node.label() {
                        Some(edge.b())
                    } else if edge.b() == node.label() {
                        Some(edge.a())
                    } else {
                        None
                    };

                    match neighbor {
                        Some(n) => {
                            let mut new_bfs_node = BFSNode::new(n.clone(), Some(&bfs_node));
                            new_bfs_node.increase_min(edge.weight() as u32);

                            node_per_min[new_bfs_node.minutes() as usize].push(new_bfs_node);
                        }
                        None => (),
                    }
                }
            }
        }

        if node_per_min[minutes].len() == 0 {
            let max = paths.iter().reduce(|acc, item| {
                if item.pression() > acc.pression() {
                    item
                } else {
                    acc
                }
            });
            match max {
                Some(node) => println!("Node {:?} with pression {}", node, node.pression()),
                None => (),
            };
            minutes += 1;
            println!("Minute: {}", minutes);
            println!(
                "Number of items to analyse: {}",
                node_per_min[minutes].len()
            );
            println!("Max pressions:\n{:?}", max_pression_per_minute);
        }
    }

    println!("Finished !");

    let max = paths.iter().reduce(|acc, item| {
        if item.pression() > acc.pression() {
            item
        } else {
            acc
        }
    });
    match max {
        Some(node) => println!("Node {:?} with pression {}", node, node.pression()),
        None => (),
    };

    Ok(())
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

// BFSNode

#[derive(Debug, Clone)]
struct BFSNode {
    label: String,
    valves_open: Vec<(String, usize, u32)>,
    minutes: u32,
}

impl BFSNode {
    fn new(label: String, node: Option<&BFSNode>) -> Self {
        match node {
            None => BFSNode {
                label,
                valves_open: Vec::new(),
                minutes: 0,
            },
            Some(node) => BFSNode {
                label,
                valves_open: node.valves_open.clone(),
                minutes: node.minutes,
            },
        }
    }

    fn open_valve(&mut self, pression: usize) -> &mut Self {
        self.valves_open
            .push((self.label.clone(), pression, self.minutes));
        self
    }

    fn valves_open(&self) -> usize {
        self.valves_open.len()
    }

    fn label(&self) -> &String {
        &self.label
    }

    fn minutes(&self) -> u32 {
        self.minutes
    }

    fn increase_min(&mut self, number: u32) -> &mut Self {
        self.minutes += number;
        self
    }

    fn is_valve_opened(&self, valve: &String) -> bool {
        self.valves_open.iter().any(|v| v.0 == *valve)
    }

    fn pression(&self) -> usize {
        self.current_pression(30)
    }

    fn current_pression(&self, minutes: usize) -> usize {
        let mut pression = 0;
        for (_, p, min) in &self.valves_open {
            pression += p * (minutes - *min as usize);
        }
        pression
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
