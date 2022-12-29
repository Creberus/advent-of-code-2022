use std::collections::HashSet;
use std::error::Error;
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

    Ok(())
}

#[derive(Debug, Hash, Eq)]
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
}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

fn parse_valve(s: String) -> Result<Valve, ()> {
    let line: Vec<&str> = s.split(';').collect();

    let valve_data: Vec<&str> = line[0].split(' ').collect();
    let (_, tunnels_data) = line[1].split_at(24);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valve() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";

        let valve = parse_valve(input.to_string()).unwrap();

        assert_eq!(*valve.label(), "AA".to_string());
        assert_eq!(valve.flow_rate(), 0);
    }
}
