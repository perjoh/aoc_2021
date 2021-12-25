use std::io::BufRead;
use std::collections::HashMap;

fn read_input() -> Vec<String> {
    std::io::stdin().lock().lines().collect::<std::io::Result<Vec<String>>>().unwrap()
}

type Paper = HashMap<(i32, i32), char>;

fn fold_paper(paper: &mut Paper, axis: char, pos: i32) {
    let mut points_remove = 
        paper
            .keys()
            .filter(|i| if axis == 'x' {i.0 > pos} else {i.1 > pos})
            .map(|i| *i)
            .collect::<Vec<_>>();

   for pt in points_remove {
       paper.remove(&pt);
       let pt_new = 
            if axis == 'x' {
                (pos - (pt.0 - pos), pt.1)
            } else {
                (pt.0, pos - (pt.1 - pos))
            };
       paper.entry(pt_new).or_insert('#');
   } 

}

fn load_paper(input: &Vec<String>) -> Paper {
    let mut paper: Paper = HashMap::new();

    input.iter()
        .take_while(|s| *s != "")
        .map(|s| s.split_once(',').unwrap())
        .map(|t|(t.0.parse::<i32>().unwrap(), t.1.parse::<i32>().unwrap()))
        .for_each(|pt| { paper.entry(pt).or_insert('#'); });

    paper
}

fn load_folds(input: &Vec<String>) -> Vec<(&str, i32)> {
    let folds =
        input.iter()
            .filter(|s| s.starts_with("fold along") )
            .map(|s| { 
                s.split_whitespace()
                    .skip(2)
                    .next().unwrap()
            })
            .map(|s| s.split_once("=").unwrap() )
            .map(|t| (t.0, t.1.parse::<i32>().unwrap()))
            .collect::<Vec<_>>();
    folds
}

fn part_one(input: &Vec<String>) -> usize {

    let mut paper = load_paper(input);
    let folds = load_folds(input);

    fold_paper(&mut paper, folds[0].0.chars().next().unwrap(), folds[0].1);

    paper.values().count()
}

fn part_two(input: &Vec<String>) -> usize {
    let mut paper = load_paper(input);
    let folds = load_folds(input);

    for fold in &folds {
        fold_paper(&mut paper, fold.0.chars().next().unwrap(), fold.1);
    }

    paper.keys().for_each(|pt| println!("{}, {}", pt.0, 10- pt.1));

    paper.values().count()
}

fn main() {
    let input = read_input();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
