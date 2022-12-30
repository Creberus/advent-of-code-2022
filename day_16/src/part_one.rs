use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::hash::Hash;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut valves = HashSet::new();

    for line in lines {
        let line = line.unwrap();

        let valve = parse_valve(line).unwrap();

        valves.insert(valve);
    }

    println!("{:?}", valves);

    // Find path with highest flow rate
    let mut minutes = 0;
    let mut nodes_left = 1;
    let mut queue = VecDeque::<BFSNode>::new();
    queue.push_back(BFSNode::new(String::from("AA"), None));

    println!("Minute: {}", minutes);

    while minutes != 30 && !queue.is_empty() {
        let bfs_node = queue.pop_front().unwrap();
        nodes_left -= 1;

        let valve = bfs_node.clone().into();
        let node = valves.get(&valve).unwrap();

        // Check if the flow rate of the valve is higher than 0.
        if node.flow_rate() > 0 && !bfs_node.is_valve_opened(node.label()) {
            let mut node_open = BFSNode::new(bfs_node.label().clone(), Some(&bfs_node));
            node_open.increase_min();
            node_open.open_valve();

            queue.push_back(node_open);
        }

        // Add neighbors to queue
        for tunnel in node {
            let mut t = BFSNode::new(tunnel.clone(), Some(&bfs_node));
            t.increase_min();
            queue.push_back(t);
        }

        if nodes_left == 0 {
            nodes_left = queue.len();
            minutes += 1;
            println!("Minute: {}", minutes);
        }
    }

    Ok(())
}

#[derive(Debug, Eq)]
struct Valve {
    label: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

impl Valve {
    fn new(label: String, flow_rate: u32) -> Self {
        Valve {
            label,
            flow_rate,
            tunnels: Vec::new(),
        }
    }

    fn add_tunnel(&mut self, tunnel: String) {
        self.tunnels.push(tunnel);
    }

    fn label(&self) -> &String {
        &self.label
    }

    fn flow_rate(&self) -> u32 {
        self.flow_rate
    }

    fn iter(&self) -> TunnelIter {
        TunnelIter::new(&self.tunnels)
    }
}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Hash for Valve {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.label.hash(state)
    }
}

impl<'a> IntoIterator for &'a Valve {
    type Item = &'a String;
    type IntoIter = TunnelIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

struct TunnelIter<'a> {
    index: usize,
    tunnels: &'a Vec<String>,
}

impl<'a> TunnelIter<'a> {
    fn new(tunnels: &'a Vec<String>) -> Self {
        TunnelIter { index: 0, tunnels }
    }
}

impl<'a> Iterator for TunnelIter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tunnels.len() {
            let tunnel = Some(&self.tunnels[self.index]);
            self.index += 1;
            tunnel
        } else {
            None
        }
    }
}

fn parse_valve(s: String) -> Result<Valve, ()> {
    let line: Vec<&str> = s.split(';').collect();

    let valve_data: Vec<&str> = line[0].split(' ').collect();
    let (_, mut tunnels_data) = line[1].split_at(23);
    if tunnels_data.chars().nth(0).unwrap() == ' ' {
        (_, tunnels_data) = tunnels_data.split_at(1);
    }

    let name = valve_data[1];
    let flow_rate = valve_data[4];
    let (_, flow_rate) = flow_rate.split_at(5);
    let flow_rate: u32 = flow_rate.parse().unwrap();

    let mut valve = Valve::new(String::from(name), flow_rate);

    let tunnels: Vec<&str> = tunnels_data.split(", ").collect();

    for tunnel in tunnels {
        valve.add_tunnel(String::from(tunnel));
    }

    Ok(valve)
}

#[derive(Debug, Clone)]
struct BFSNode {
    label: String,
    valves_open: Vec<(String, u32)>,
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

    fn open_valve(&mut self) -> &mut Self {
        self.valves_open.push((self.label.clone(), self.minutes));
        self
    }

    fn label(&self) -> &String {
        &self.label
    }

    fn increase_min(&mut self) -> &mut Self {
        self.minutes += 1;
        self
    }

    fn is_valve_opened(&self, valve: &String) -> bool {
        self.valves_open.iter().any(|v| v.0 == *valve)
    }
}

impl Into<Valve> for BFSNode {
    fn into(self) -> Valve {
        Valve::new(self.label.clone(), 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valve() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";

        let valve = parse_valve(input.to_string()).unwrap();
        let tunnels = Vec::from([String::from("DD"), String::from("II"), String::from("BB")]);

        assert_eq!(*valve.label(), "AA".to_string());
        assert_eq!(valve.flow_rate(), 0);

        assert!(valve.iter().map(|s| s.clone()).eq(tunnels));
    }
}
