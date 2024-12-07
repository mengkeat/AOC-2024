use regex::Regex;

fn main() {
    let data = include_str!("../input/day03.txt");
    println!("Part 1: {}", xmul(data));
    println!("Part 2: {}", xmul_do_dont(data));
}
fn xmul(s: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(s)
        .map(|cap| {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            a * b
        })
        .sum()
}

fn xmul_do_dont(s: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut sum = 0;
    let mut doit = true;
    for cap in re.captures_iter(s) {
        match cap.get(0).unwrap().as_str() {
            "do()" => doit = true,
            "don't()" => doit = false,
            _ if doit => {
                let a = cap[1].parse::<i32>().unwrap();
                let b = cap[2].parse::<i32>().unwrap();
                sum += a * b;
            }
            _ => {}
        }
    }
    sum
}

#[cfg(test)]
mod test {
    #[test]
    fn test_mul() {
        assert_eq!( super::xmul("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"), 
                    (2*4 + 5*5 + 11*8 + 8*5) );
    }

    #[test]
    fn test_xmul_do_dont() {
        assert_eq!( super::xmul_do_dont("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            2*4 + 8*5);
    }
}