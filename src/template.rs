// In dayX.rs (replace X with the day number)

use std::fs;

fn main() {
    // Read input file for the specific day
    let input = fs::read_to_string("input/dayX.txt").expect("Failed to read input file");
    
    // Parse and process the input
    let data: Vec<_> = input.lines().map(|line| line.trim()).collect();

    // Part 1: Solve the first part of the puzzle
    let part1_solution = part1(&data);
    println!("Part 1 Solution: {}", part1_solution);

    // Part 2: Solve the second part of the puzzle
    let part2_solution = part2(&data);
    println!("Part 2 Solution: {}", part2_solution);
}

fn part1(data: &[&str]) -> i32 {
    // Implement your Part 1 logic here
    // You can use the 'data' variable to process the input
    // Return the result of Part 1 as an i32
    0
}

fn part2(data: &[&str]) -> i32 {
    // Implement your Part 2 logic here
    // You can use the 'data' variable to process the input
    // Return the result of Part 2 as an i32
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/day1-test.txt").expect("Failed to read input file");
        let data: Vec<_> = input.lines().map(|line| line.trim()).collect();
        assert_eq!(part1(&data), 142);
    }
}