use std::io::BufRead;
use regex::Regex;

fn read_input() -> Vec<String> {
    std::io::stdin().lock().lines().collect::<std::io::Result<Vec<String>>>().unwrap()
}

fn parse_target_area(input: &str) -> ((i32, i32), (i32, i32)) {
    let re = Regex::new(r"=(-?\d+)\.\.(-?\d+).+=(-?\d+)\.\.(-?\d+)").unwrap();
    for cap in re.captures_iter(input) {
        let a = (&cap[1]).parse::<i32>().unwrap();
        let b = (&cap[2]).parse::<i32>().unwrap();
        let c = (&cap[3]).parse::<i32>().unwrap();
        let d = (&cap[4]).parse::<i32>().unwrap();
        return ((a, b), (c, d));
    }
    ((0,0), (0,0))
}

struct Probe {
    pos: (i32, i32),
    vel: (i32, i32),
    target: ((i32, i32), (i32, i32)),
}

impl Probe {
    fn new(vel: (i32, i32), target: ((i32, i32), (i32, i32))) -> Probe {
        Probe { pos: (0, 0), vel, target }
    }

    fn trace(&mut self) -> Vec<(i32, i32)> {
        let mut pts = Vec::new();
        loop {
            self.pos.0 += self.vel.0;
            self.pos.1 += self.vel.1;

            pts.push(self.pos);

            if self.pos.0 >= self.target.0.0 && self.pos.0 <= self.target.0.1 {
                if self.pos.1 >= self.target.1.0 && self.pos.1 <= self.target.1.1 {
                    // inside target
                    return pts;
                }
            }

            if self.pos.1 < self.target.1.0 {
                // below target, miss
                return Vec::new();
            }

            if self.vel.0 > 0 {
                self.vel.0 -= 1;
            } else if self.vel.0 < 0 {
                self.vel.0 += 1;
            }
            self.vel.1 -= 1;
        }
    }
} 

fn part_one(input: &Vec<String>) -> i32 {
    let target = parse_target_area(input.first().unwrap());
    let mut heights = Vec::new();
    for y in 0..150 {
        for x in 0..150 {
            let mut probe = Probe::new((x, y), target);
            let pts = probe.trace();
            if !pts.is_empty() {
                heights.push(pts.iter().map(|v| v.1).max().unwrap())
            }
        }
    }
    *heights.iter().max().unwrap()
}

fn part_two(input: &Vec<String>) -> usize {
    let target = parse_target_area(input.first().unwrap());
    let mut count = 0;
    for y in target.1.0..=150 {
        for x in 0..=target.0.1 {
            let mut probe = Probe::new((x, y), target);
            let pts = probe.trace();
            if !pts.is_empty() {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let input = read_input();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
