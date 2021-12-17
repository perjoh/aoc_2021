use std::io::BufRead;

type Values = Vec<i64>;

fn read_input() -> Values {
    //std::io::stdin().lock().lines().collect::<std::io::Result<Vec<String>>>().unwrap()
    let s = std::io::stdin().lock().lines().nth(0).unwrap().unwrap();
    let mut values = s.split(',').map(|s| s.parse::<i64>().unwrap()).collect::<Values>();
    values.sort();
    values
}

fn calc_cost(positions: &Values, i: usize) -> i64 {
    let new_pos = positions[i];
    let cost = positions.iter().fold(0, |s, v| s + (new_pos - v).abs());
    cost
}

fn bisect(cost: i64, positions: &Values, l: usize, r: usize) -> i64 {
    if l != r {
        let mid = (r - l)/2;
        let l_mid = (mid - l)/2 + l;

        if l_mid != l {
            let l_cost = calc_cost(positions, l_mid);
            if l_cost < cost {
                return bisect(l_cost, positions, l, mid);
            } 
        }

        let r_mid = (r - mid)/2 + mid;
        if r_mid != mid {
            let r_cost = calc_cost(positions, r_mid);
            if r_cost < cost {
                return bisect(r_cost, positions, mid, r);
            }
        }
    }
    cost
}

fn part_one(input: &Values) -> i64 {
    let cost = bisect(calc_cost(input, input.len()/2), &input, 0, input.len());
    cost
}

fn part_two(input: &Values) -> i64 {
    let max = input.iter().max().unwrap();
    (0..*max).map(|i| {
        input.iter().fold(0, |s, v| {
            let n = (v - i).abs();
            s + n*(n+1)/2
        })
    }).min().unwrap()
}

fn main() {
    let input = read_input();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
