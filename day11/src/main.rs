use std::io::BufRead;

fn read_input() -> Vec<String> {
    std::io::stdin().lock().lines().collect::<std::io::Result<Vec<String>>>().unwrap()
}

struct OctoMap {
    values: Vec<u8>,
    width: i32,
    height: i32,
}

fn read_octo_map(input: &Vec<String>) -> OctoMap {
    OctoMap {
        values: input.iter().map(|s|s.chars().map(|c|c.to_string().parse::<u8>().unwrap())).flatten().collect::<Vec<_>>(),
        width: input[0].len() as i32,
        height: input.len() as i32,
    }
}

fn octo_flash(om: &mut OctoMap, x: i32, y: i32) -> i32 {
    1 +
    octo_inc(om, x, y) + // inc again to prevent it from flashing a second time
    octo_inc(om, x+1, y-1) +
    octo_inc(om, x+1, y) +
    octo_inc(om, x+1, y+1) +
    octo_inc(om, x-1, y-1) +
    octo_inc(om, x-1, y) +
    octo_inc(om, x-1, y+1) +
    octo_inc(om, x, y-1) +
    octo_inc(om, x, y+1)
}

fn octo_inc(om: &mut OctoMap, x: i32, y: i32) -> i32 {
    if x >= 0 && x < om.width && y >= 0 && y < om.height {
        let v = &mut om.values[(x + y*om.width) as usize];
        *v += 1;
        if *v == 10 {
            return octo_flash(om, x, y)
        }
    }
    0
}

fn reset_energy(om: &mut OctoMap) {
    om.values = om.values.iter().map(|v| if *v < 10 { *v } else { 0 }).collect();
}

fn part_one(input: &Vec<String>) -> i32 {
    let mut om = read_octo_map(input);
    let mut flash_count = 0;
    for _ in 0..100 {
        for y in 0..om.height {
            for x in 0..om.width {
                flash_count += octo_inc(&mut om, x, y);
            }
        }

        reset_energy(&mut om);
    }

    flash_count
}

fn part_two(input: &Vec<String>) -> usize {
    let mut om = read_octo_map(input);
    let mut flash_count = 0;
    for i in 1.. {
        for y in 0..om.height {
            for x in 0..om.width {
                flash_count += octo_inc(&mut om, x, y);
            }
        }

        if om.values.iter().filter(|v| **v > 9).count() == 100 {
            return i
        }

        reset_energy(&mut om);
    }
    0
}

fn main() {
    let input = read_input();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
