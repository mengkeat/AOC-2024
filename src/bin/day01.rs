
fn main() {
    let data = include_str!("../input/day01.txt");

    let comb: Vec<_> = data 
        .lines()
        .map(|l| l.split_once("   ").unwrap())
        .collect();

    let mut left: Vec<i32> = comb.iter().map(|(l, _)| l.parse::<i32>().unwrap()).collect();
    let mut right: Vec<i32> = comb.iter().map(|(_, r)| r.parse::<i32>().unwrap()).collect();

    left.sort();
    right.sort();

    let res: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();
    println!("Part 1: {}", res);

    // ---- Part 2 -----
    let res: i32 = left
        .iter()
        .map(|l| right.iter().filter(|&r| r == l).count() as i32 * l)
        .sum();
    println!("Part 2: {}", res);
}