use std::fs::read_to_string;

fn main() {
    let input = read_lines("input.txt");

    p1(&input);
    p2(&input);
}

fn p1(input: &str) {
    let indices: Vec<_> = input.match_indices("mul(").map(|(i, _)| i).collect();

    let sum: i32 = indices
        .iter()
        .map(|i| {
            for x in 4..8 {
                let vec: Vec<_> = input.chars().skip(*i).take(x + 5).collect();
                let s: String = vec.into_iter().collect();
                // println!("s: {s}");
                if let Some((v1, v2)) = check_valid(&s) {
                    return get_multiple_value(v1, v2);
                }
            }
            0
        })
        .sum();
    println!("sum: {sum}");
}
const MAX_LENGTH: usize = "mul(123,123)".len();
fn p2(input: &str) {
    let mut enabled = true;
    let sum: i32 = input
        .chars()
        .enumerate()
        .map(|(i, _)| i)
        .map(|index| {
            for x in 4..=MAX_LENGTH {
                let vec: Vec<_> = input.chars().skip(index).take(x).collect();
                let s: String = vec.into_iter().collect();
                if s == "do()" {
                    enabled = true;
                } else if s == "don't()" {
                    enabled = false;
                }

                if let Some((v1, v2)) = check_valid(&s) {
                    println!("found valid {v1}, {v2}. enabled is: {enabled}");
                    if !enabled {
                        return 0;
                    }
                    return get_multiple_value(v1, v2);
                }
            }

            0
        })
        .sum();

    println!("p2 sum: {sum}");
}

fn get_multiple_value(v1: i32, v2: i32) -> i32 {
    v1 * v2
}

fn check_valid(str: &str) -> Option<(i32, i32)> {
    if !str.starts_with("mul(") {
        return None;
    }
    if !str.ends_with(')') {
        return None;
    }
    if !str.match_indices(',').count() == 1 {
        return None;
    }
    let vec: Vec<&str> = str.get(4..).unwrap().split(",").collect();
    if vec.len() != 2 {
        return None;
    }
    let (v1, v2) = (vec[0], vec[1]);
    if !v1.chars().all(|c| c.is_numeric()) {
        return None;
    };
    if !v2.chars().take(v2.len() - 1).all(|c| c.is_numeric()) {
        return None;
    }
    let v2 = v2.get(0..v2.len() - 1).unwrap();

    Some((v1.parse().unwrap(), v2.parse().unwrap()))
}

fn read_lines(filename: &str) -> String {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .fold("".to_string(), |acc, x| acc + &x)
}
