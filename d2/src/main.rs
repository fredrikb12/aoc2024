use std::fs::read_to_string;

struct Line {
    reports: Vec<i32>,
}
impl Line {}

fn main() {
    let lines = read_lines("input.txt");
    let safe_count: usize = lines
        .iter()
        .filter(|line| match check_direction(line) {
            "asc" => true,
            "desc" => true,
            &_ => false,
        })
        .filter(|line| distances_safe(line))
        .count();
    println!("safe count: {safe_count}")
}

fn check_direction(line: &Line) -> &str {
    let increasing: bool = create_iterator(line).all(|(a, b)| a > b);
    let decreasing: bool = create_iterator(line).all(|(a, b)| a < b);

    if increasing {
        "asc"
    } else if decreasing {
        return "desc";
    } else {
        return "unsafe";
    }
}

fn distances_safe(line: &Line) -> bool {
    create_iterator(line).all(|(a, b)| (a - b).abs() <= 3)
}

fn create_iterator(line: &Line) -> impl Iterator<Item = (&i32, &i32)> {
    line.reports.windows(2).map(|w| {
        let mut iter = w.iter();

        let (a, b) = (iter.next().unwrap(), iter.next().unwrap());
        (a, b)
    })
}

fn read_lines(filename: &str) -> Vec<Line> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .map(|line| Line {
            reports: line
                .split_whitespace()
                .map(|s| s.to_string().parse().unwrap())
                .collect(),
        })
        .collect()
}
