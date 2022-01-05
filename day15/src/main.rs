use std::io::BufRead;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

fn read_input() -> Vec<String> {
    std::io::stdin().lock().lines().collect::<std::io::Result<Vec<String>>>().unwrap()
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    pos: (i32, i32),
    cost: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Cavern = HashMap<(i32, i32), i32>;

fn parse_cavern(input: &Vec<String>) -> Cavern {
    input.iter().enumerate().map(|f| { 
        let y = f.0;
        f.1.bytes().enumerate().map(|f| {
            ((f.0 as i32, y as i32), (f.1 - b'0') as i32)
        }).collect::<Vec<((_, _), _)>>()
    }).flatten().collect::<HashMap<(_, _), _>>()
}

fn cavern_dimensions(cavern: &Cavern) -> (i32, i32) {
    (cavern.keys().map(|k| k.0).max().unwrap(),
    cavern.keys().map(|k| k.1).max().unwrap())
}

fn shortest_distance(cavern: &Cavern) -> i32 {
    let mut min_distances: HashMap<(i32, i32), i32> = cavern.iter().map(|v| (*v.0, i32::MAX)).collect();
    let e  = min_distances.entry((0, 0)).or_default();
    *e = 0;

    let mut queue = BinaryHeap::new();
    queue.push(Node { cost: 0, pos: (0, 0)});

    let target = cavern_dimensions(cavern);

    while let Some(Node { cost, pos }) = queue.pop() {
        if pos == target {
            break;
        }

        let neighbours: Vec<_> = [(1, 0), (0, 1), (-1, 0), (0, -1)].iter().map(|v| (v.0 + pos.0, v.1 + pos.1)).collect();

        for neighbour in &neighbours {
            match min_distances.get_mut(&neighbour) {
                Some(distance) => {
                    let new_distance = cavern.get(neighbour).unwrap() + cost;
                    if new_distance < *distance {
                        *distance = new_distance;
                        queue.push(Node { cost: *distance, pos: *neighbour});
                    }
                },
                _ => { }
            }
        }
    }

    *min_distances.get(&target).unwrap()
}

fn tile_cavern(cavern: &Cavern) -> Cavern {
    let dimensions = cavern_dimensions(cavern);
    let mut new_cavern = Cavern::new();

    let (width, height) = (dimensions.0 + 1, dimensions.1 + 1);

    for y in 0..height {
        for x in 0..width {
            let cost = *cavern.get(&(x, y)).unwrap();
            new_cavern.insert((x, y), cost);

            for j in 0..5 {
                for i in 0..5 {
                    if i != 0 || j != 0 {
                        let new_cost = (cost - 1 + i + j)%9 + 1;
                        let offset = (width*i + x, height*j + y);
                        new_cavern.insert(offset, new_cost);
                    }
                }
            }
        }
    }

    new_cavern
}

fn part_one(input: &Vec<String>) -> i32 {
    let cavern = parse_cavern(input);
    shortest_distance(&cavern)
}

fn part_two(input: &Vec<String>) -> i32 {
    let cavern = parse_cavern(input);
    let cavern = tile_cavern(&cavern);
    shortest_distance(&cavern)
}

fn main() {
    let input = read_input();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
