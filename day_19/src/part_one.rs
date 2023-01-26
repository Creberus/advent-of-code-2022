use std::collections::VecDeque;
use std::error::Error;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut blueprints = Vec::new();

    for line in lines {
        let line = line.unwrap();

        let (blueprint_id, robots) = line.split_once(": ").unwrap();

        let blueprint_id: Vec<&str> = blueprint_id.split(' ').collect();

        let mut bp = Blueprint::new(blueprint_id[1].parse().unwrap());

        let robots: Vec<&str> = robots.split(". ").collect();

        let ore_robot_str: Vec<&str> = robots[0].split(' ').collect();
        let ore_robot_cost = ore_robot_str[4].parse().unwrap();
        let ore_robot = RobotBP::new(ore_robot_cost, 0, 0);

        let clay_robot_str: Vec<&str> = robots[1].split(' ').collect();
        let clay_robot_cost = clay_robot_str[4].parse().unwrap();
        let clay_robot = RobotBP::new(clay_robot_cost, 0, 0);

        let obsidian_robot_str: Vec<&str> = robots[2].split(' ').collect();
        let obsidian_robot_ore_cost = obsidian_robot_str[4].parse().unwrap();
        let obsidian_robot_clay_cost = obsidian_robot_str[7].parse().unwrap();
        let obsidian_robot = RobotBP::new(obsidian_robot_ore_cost, obsidian_robot_clay_cost, 0);

        let geode_robot_str: Vec<&str> = robots[3].split(' ').collect();
        let geode_robot_ore_cost = geode_robot_str[4].parse().unwrap();
        let geode_robot_obsidian_cost = geode_robot_str[7].parse().unwrap();
        let geode_robot = RobotBP::new(geode_robot_ore_cost, 0, geode_robot_obsidian_cost);

        bp.add_ore_robot_bp(ore_robot);
        bp.add_clay_robot_bp(clay_robot);
        bp.add_obsidian_robot_bp(obsidian_robot);
        bp.add_geode_robot_bp(geode_robot);

        blueprints.push(bp);
    }

    let mut bp_contexts = Vec::<Context>::new();

    for blueprint in blueprints {
        let mut contexts = VecDeque::<Context>::new();
        let mut best_context = Context::new();
        contexts.push_back(Context::new());

        while !contexts.is_empty() {
            let mut ctx = contexts.pop_front().unwrap();

            // 0. Check if the minutes is over 24
            if ctx.minute() == 24 {
                best_context = if ctx.geode() > best_context.geode() {
                    ctx
                } else {
                    best_context
                };

                continue;
            }

            // 1. Start of turn
            // You can choose to construct a robot

            // 2. Collect phase
            // Each robot collects its mineral
            ctx.collect();

            // 3. Robot have been constructed
            // The robot you constructed at the start of the phase is finished
        }

        bp_contexts.push(best_context);
    }

    Ok(())
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_robot: RobotBP,
    clay_robot: RobotBP,
    obsidian_robot: RobotBP,
    geode_robot: RobotBP,
}

impl Blueprint {
    fn new(id: usize) -> Self {
        Self {
            id,
            ore_robot: RobotBP::default(),
            clay_robot: RobotBP::default(),
            obsidian_robot: RobotBP::default(),
            geode_robot: RobotBP::default(),
        }
    }

    fn add_ore_robot_bp(&mut self, ore_robot: RobotBP) {
        self.ore_robot = ore_robot;
    }

    fn add_clay_robot_bp(&mut self, clay_robot: RobotBP) {
        self.clay_robot = clay_robot;
    }

    fn add_obsidian_robot_bp(&mut self, obsidian_robot: RobotBP) {
        self.obsidian_robot = obsidian_robot;
    }

    fn add_geode_robot_bp(&mut self, geode_robot: RobotBP) {
        self.geode_robot = geode_robot;
    }
}

#[derive(Debug)]
struct RobotBP {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

impl RobotBP {
    fn new(ore: usize, clay: usize, obsidian: usize) -> Self {
        Self {
            ore,
            clay,
            obsidian,
        }
    }

    fn ore(&self) -> usize {
        self.ore
    }

    fn clay(&self) -> usize {
        self.clay
    }

    fn obsidian(&self) -> usize {
        self.obsidian
    }
}

impl Default for RobotBP {
    fn default() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy)]
struct Context {
    minute: u8,
    // Minerals
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    // Robots
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
    // Construct
    construct_robot: bool,
    robot_type: RobotType,
}

impl Context {
    fn new() -> Self {
        Self {
            minute: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }

    fn minute(&self) -> u8 {
        self.minute
    }

    fn minute_mut(&mut self) -> &mut u8 {
        &mut self.minute
    }

    fn ore(&self) -> usize {
        self.ore
    }

    fn clay(&self) -> usize {
        self.clay
    }

    fn obsidian(&self) -> usize {
        self.obsidian
    }

    fn geode(&self) -> usize {
        self.geode
    }

    fn collect(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
    }

    fn consume_ore(&mut self, consume: usize) {
        self.ore -= consume;
    }

    fn consume_clay(&mut self, consume: usize) {
        self.clay -= consume;
    }

    fn consume_obsidian(&mut self, consume: usize) {
        self.obsidian -= consume;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {}
}
