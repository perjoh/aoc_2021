use std::{io::BufRead, collections::HashSet};

fn read_input() -> Vec<String> {
    std::io::stdin().lock().lines().collect::<std::io::Result<Vec<String>>>().unwrap()
}

type Probes = HashSet<(i32, i32, i32)>;

fn print_scanners(scanners: &Vec<Probes>) {
    for s in scanners {
        println!("scanner:");

        for p in s {
            println!("{}, {}, {}", p.0, p.1, p.2);
        }

        println!("");
    }
}

type Vector = (i32, i32, i32);
type Matrix = (Vector, Vector, Vector);

fn rotate(mat: &Matrix, vec: &Vector) -> Vector {
    (mat.0.0*vec.0 + mat.0.1*vec.1 + mat.0.2*vec.2,
    mat.1.0*vec.0 + mat.1.1*vec.1 + mat.1.2*vec.2,
    mat.2.0*vec.0 + mat.2.1*vec.1 + mat.2.2*vec.2)
}

fn rotate_probe(probes: &Probes, mat: &Matrix) -> Probes {
    probes.iter().map(|v| rotate(mat, v)).collect()
}

fn rotate_probe_n(probes: &Probes, mat: &Matrix, n: usize) -> Probes {
    let mut probes = probes.clone();
    for _ in 0..n {
        probes = rotate_probe(&probes, mat);
    }
    probes
}

fn offset(p: &Probes) -> Probes {
    let min = p.iter().min().unwrap();
    p.iter().map(|v| (v.0 - min.0, v.1 - min.1, v.2 - min.2)).collect()
}

fn permutations(probes: &Probes) -> Vec<Probes> {
    let rot_x = ((1, 0, 0), (0, 0, -1), (0, 1, 0));
    let rot_y = ((0, 0, 1), (0, 1, 0), (-1, 0, 0));
    let rot_z = ((0, -1, 0), (1, 0, 0), (0, 0, 1));

    let mut perm = Vec::new();
    perm.push(offset(&probes.clone()));
    for _ in 0..3 {
        perm.push(offset(&rotate_probe(perm.last().unwrap(), &rot_x)));
    }

    perm.push(offset(&rotate_probe_n(perm.first().unwrap(), &rot_y, 2)));
    for _ in 0..3 {
        perm.push(offset(&rotate_probe(perm.last().unwrap(), &rot_x)));
    }

    // y
    perm.push(offset(&rotate_probe(perm.first().unwrap(), &rot_z)));
    for _ in 0..3 {
        perm.push(offset(&rotate_probe(perm.last().unwrap(), &rot_y)));
    }

    perm.push(offset(&rotate_probe_n(&perm[8], &rot_x, 2)));
    for _ in 0..3 {
        perm.push(offset(&rotate_probe(perm.last().unwrap(), &rot_y)));
    }

    // z
    perm.push(offset(&rotate_probe(perm.first().unwrap(), &rot_y)));
    for _ in 0..3 {
        perm.push(offset(&rotate_probe(perm.last().unwrap(), &rot_z)));
    }

    perm.push(offset(&rotate_probe_n(&perm[16], &rot_x, 2)));
    for _ in 0..3 {
        perm.push(offset(&rotate_probe(perm.last().unwrap(), &rot_z)));
    }

    perm
}

fn check_overlap(lhs: &Vec<Probes>, rhs: &Probes) -> usize {
    let mut overlaps = Vec::new();
    for p in lhs {
        let mut count: usize = 0;
        for v in p {
            if rhs.contains(v) {
                count += 1;
            }
        }
        overlaps.push(count);
    }

    *overlaps.iter().max().unwrap()
}

fn part_one(input: &Vec<String>) -> usize {
    let scanners: Vec<Probes> = 
    input.split(|s| s.is_empty())
        .map(|f| 
            f[1..].into_iter().map(|s| {
                let v: Vec<_> = 
                    s.splitn(3, ',')
                        .map(|v| i32::from_str_radix(v, 10).unwrap())
                        .collect();
                (v[0], v[1], v[2])
            }).collect())
        .collect();

    //print_scanners(&scanners);

    let permutations: Vec<_> = scanners.iter().map(|p| {
        permutations(p)
    }).collect();

    0
}


fn part_two(input: &Vec<String>) -> usize {

    0
}

fn main() {
    let input = read_input();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
