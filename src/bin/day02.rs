

fn is_safe(row: &[i32]) -> bool {
    let in_range = row.windows(2).all(|w| ((w[1]-w[0]).abs() >= 1 && (w[1]-w[0]).abs() <= 3));
    let strictly_increase = row.windows(2).all(|w| w[0] < w[1]);
    let strictly_decrease = row.windows(2).all(|w| w[0] > w[1]);
    return in_range && (strictly_increase || strictly_decrease);
}

fn main() {
    let data = include_str!("../input/day02.txt");

    let rec: Vec<Vec<i32>> = data
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();

    let num_safe = rec.iter().filter(|r| is_safe(r)).count();
    println!("Number of safe rows: {}", num_safe);

    // for each rec , drop 1 element and check if it is safe
    let num_safe2 = rec.iter().filter(|r| {
        for i in 0..r.len() {
            let mut r1 = (*r).clone();
            r1.remove(i);
            if is_safe(&r1) {
                return true;
            }
        }
        return false;
    }).count();
    println!("Number of safe rows with one element removed: {}", num_safe2);
}


#[cfg(test)]
mod test {

    #[test]
    fn safety_test() {
        assert!(super::is_safe(&[1, 2, 3, 4, 5, 6]));
        assert!(super::is_safe(&[7, 6, 4, 2, 1]));

        assert!(!super::is_safe(&[1, 3, 2, 4, 5]));
        assert!(!super::is_safe(&[8, 6, 4, 4, 1]));
        assert!(!super::is_safe(&[1, 7, 3, 4, 5]));
    }

}