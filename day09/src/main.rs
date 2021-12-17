use std::io::BufRead;

fn read_input() -> Vec<String> {
    std::io::stdin().lock().lines().collect::<std::io::Result<Vec<String>>>().unwrap()
}

#[derive(Clone)]
struct HeightMap {
    values: Vec<u8>,
    width: i32,
    height: i32,
}

fn read_height_map(input: &Vec<String>) -> HeightMap {
    let width = input.first().unwrap().len();
    let height_map = input.iter().map(|s| s.chars().map(|c| c.to_string().parse::<u8>().unwrap())).flatten().collect::<Vec<_>>();
    let height = height_map.len()/width;
    HeightMap { width: width as i32, height: height as i32, values: height_map }
}


fn get_height(hm: &HeightMap, x: i32, y: i32) -> u8 {
    if x >= 0 && x < hm.width && y >= 0 && y < hm.height {
        return hm.values[(x+y*hm.width) as usize]
    }
    10
}

fn is_sunk(hm: &HeightMap, x: i32, y: i32) -> bool {
    let h = get_height(hm, x, y);
    if h == 10 {
        return false
    }

    let n = get_height(hm, x, y-1);
    let s = get_height(hm, x, y+1);
    let w = get_height(hm, x-1, y);
    let e = get_height(hm, x+1, y);

    (n == 10 || h < n) && 
    (s == 10 || h < s) && 
    (w == 10 || h < w) && 
    (e == 10 || h < e) && 
    (n + s + w + e < 40) // all surrounding cannot be sunk
}

fn part_one(input: &Vec<String>) -> usize {
    let hm = read_height_map(input);

    let mut risk: usize = 0;
    for y in 0..hm.height {
        for x in 0..hm.width {
            if is_sunk(&hm, x, y) {
                let h = get_height(&hm, x, y) as usize;
                risk += h + 1;
            }
        }
    }
    risk
}

fn mark_sunk(hm: &mut HeightMap, x: i32, y: i32) -> i32 {
    if get_height(&hm, x, y) < 9 {
        hm.values[(x+y*hm.width) as usize] = 10;

        return 
            1 +
            mark_sunk(hm, x+1, y) +
            mark_sunk(hm, x-1, y) +
            mark_sunk(hm, x, y+1) +
            mark_sunk(hm, x, y-1) 
    }
    0
}

fn part_two(input: &Vec<String>) -> i32 {
    let mut hm = read_height_map(input);
    let mut sizes: Vec<i32> = Vec::new();
    for y in 0..hm.height {
        for x in 0..hm.width {
            if is_sunk(&hm, x, y) {
                sizes.push(mark_sunk(&mut hm, x, y));
            }
        }
    }

    sizes.sort();
    sizes.iter().rev().take(3).fold(1, |s, v| s*v)
}

fn main() {
    let input = read_input();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
