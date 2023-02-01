use std::error::Error;
use std::hash::Hash;
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

    let mut quality_levels = 0;

    for blueprint in &blueprints {
        println!("Blueprint {}/{}", blueprint.id, blueprints.len());

        let mut ctx = Context::new();

        let geode_collected = process(&mut ctx, blueprint);

        println!("\tGeodes: {}", geode_collected);

        let quality_level = blueprint.id * geode_collected;
        println!("\tQuality Level: {}", quality_level);

        quality_levels += quality_level;
    }

    println!("Final quality levels: {}", quality_levels);

    Ok(())
}

fn process(ctx: &mut Context, bp: &Blueprint) -> usize {
    if ctx.minute() == 24 {
        return ctx.geode();
    }

    let mut maximum = 0;

    if can_construct(&ctx, &bp.ore_robot) {
        let mut ore_ctx = ctx.clone();

        construct(&mut ore_ctx, &bp.ore_robot, RobotType::Ore);

        ore_ctx.collect();

        *ore_ctx.minute_mut() += 1;

        ore_ctx.construct();

        let ore_max = process(&mut ore_ctx, bp);
        maximum = maximum.max(ore_max);
    }

    if can_construct(&ctx, &bp.clay_robot) {
        let mut clay_ctx = ctx.clone();

        construct(&mut clay_ctx, &bp.clay_robot, RobotType::Clay);

        clay_ctx.collect();

        *clay_ctx.minute_mut() += 1;

        clay_ctx.construct();

        let clay_max = process(&mut clay_ctx, bp);
        maximum = maximum.max(clay_max);
    }

    if can_construct(&ctx, &bp.obsidian_robot) {
        let mut obsidian_ctx = ctx.clone();

        construct(&mut obsidian_ctx, &bp.obsidian_robot, RobotType::Obsidian);

        obsidian_ctx.collect();

        *obsidian_ctx.minute_mut() += 1;

        obsidian_ctx.construct();

        let obsidian_max = process(&mut obsidian_ctx, bp);
        maximum = maximum.max(obsidian_max);
    }

    if can_construct(&ctx, &bp.geode_robot) {
        let mut geode_ctx = ctx.clone();

        construct(&mut geode_ctx, &bp.geode_robot, RobotType::Geode);

        geode_ctx.collect();

        *geode_ctx.minute_mut() += 1;

        geode_ctx.construct();

        let geode_max = process(&mut geode_ctx, bp);
        maximum = maximum.max(geode_max);
    }

    ctx.collect();

    *ctx.minute_mut() += 1;

    let ctx_max = process(ctx, bp);
    maximum = maximum.max(ctx_max);

    maximum
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

fn can_construct(ctx: &Context, bp: &RobotBP) -> bool {
    ctx.ore() >= bp.ore() && ctx.clay() >= bp.clay() && ctx.obsidian() >= bp.obsidian()
}

fn construct(ctx: &mut Context, bp: &RobotBP, rt: RobotType) {
    ctx.consume_ore(bp.ore());
    ctx.consume_clay(bp.clay());
    ctx.consume_obsidian(bp.obsidian());

    *ctx.construct_robot_mut() = true;
    *ctx.construct_robot_type_mut() = rt;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy, Eq)]
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
            construct_robot: false,
            robot_type: RobotType::Ore,
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

    fn construct_robot_mut(&mut self) -> &mut bool {
        &mut self.construct_robot
    }

    fn construct_robot_type_mut(&mut self) -> &mut RobotType {
        &mut self.robot_type
    }

    fn construct(&mut self) {
        if self.construct_robot {
            self.construct_robot = false;
            match self.robot_type {
                RobotType::Ore => self.ore_robots += 1,
                RobotType::Clay => self.clay_robots += 1,
                RobotType::Obsidian => self.obsidian_robots += 1,
                RobotType::Geode => self.geode_robots += 1,
            }
        }
    }
}

impl PartialEq for Context {
    fn eq(&self, other: &Self) -> bool {
        self.minute == other.minute
            && self.ore == other.ore
            && self.clay == other.clay
            && self.obsidian == other.obsidian
            && self.geode == other.geode
            && self.ore_robots == other.ore_robots
            && self.clay_robots == other.clay_robots
            && self.obsidian_robots == other.obsidian_robots
            && self.geode_robots == other.geode_robots
    }
}

impl Hash for Context {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.minute.hash(state);
        self.ore.hash(state);
        self.clay.hash(state);
        self.obsidian.hash(state);
        self.geode.hash(state);
        self.ore_robots.hash(state);
        self.clay_robots.hash(state);
        self.obsidian_robots.hash(state);
        self.geode_robots.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {}
}
