

fn main() {
    let data = include_str!("../input/day04.txt");
    println!("Part 1: {}", search(data));
}

fn search(s: &str) -> i32 {
    let grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
    let r = grid.len() as usize;
    let c = grid[0].len() as usize;

    let mut count1 = 0;
    let mut count2 = 0;
    for i in 0.. r{ 
        for j in 0 .. c{
            if j+3 < c && grid[i][j] == 'X' && grid[i][j+1] == 'M' && grid[i][j+2] == 'A' && grid[i][j+3] == 'S' {
                count1 += 1;
            }
            if j+3 < c && grid[i][j] == 'S' && grid[i][j+1] == 'A' && grid[i][j+2] == 'M' && grid[i][j+3] == 'X' {
                count1 += 1;
            }
            if i+3 < r && grid[i][j] == 'X' && grid[i+1][j] == 'M' && grid[i+2][j] == 'A' && grid[i+3][j] == 'S' {
                count1 += 1;
            }
            if i+3 < r && grid[i][j] == 'S' && grid[i+1][j] == 'A' && grid[i+2][j] == 'M' && grid[i+3][j] == 'X' {
                count1 += 1;
            }
            if j+3 < c && i+3 < r && grid[i][j] == 'X' && grid[i+1][j+1] == 'M' && grid[i+2][j+2] == 'A' && grid[i+3][j+3] == 'S' {
                count1 += 1;
            }
            if j+3 < c && i+3 < r && grid[i][j] == 'S' && grid[i+1][j+1] == 'A' && grid[i+2][j+2] == 'M' && grid[i+3][j+3] == 'X' {
                count1 += 1;
            }
            if i >= 3 && j+3 < c && grid[i][j] == 'X' && grid[i-1][j+1] == 'M' && grid[i-2][j+2] == 'A' && grid[i-3][j+3] == 'S' {
                count1 += 1;
            }
            if i >= 3 && j+3 < c && grid[i][j] == 'S' && grid[i-1][j+1] == 'A' && grid[i-2][j+2] == 'M' && grid[i-3][j+3] == 'X' {
                count1 += 1;
            }
        }
    }
    count1
}

#[cfg(test)]
mod test {
    #[test]
    fn test_search() {
        let test_str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!( super::search(test_str), 18);
    }
}