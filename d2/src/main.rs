use std::fs::read_to_string;
#[derive(Debug)]
struct Line {
    reports: Vec<i32>,
}
impl Line {
    fn remove_index(&self, index: usize) -> Line {
        let mut vec = self.reports.clone();
        vec.remove(index);
        Line { reports: vec }
    }
}

fn main() {
    let lines = read_lines("input.txt");
    p1(&lines);

    p2(&lines);
}

fn p1(lines: &[Line]) {
    let safe_count: usize = lines
        .iter()
        .filter(|line| match check_direction(line) {
            "asc" => true,
            "desc" => true,
            &_ => false,
        })
        .filter(|line| distances_safe(line))
        .count();
    println!("p1 count: {safe_count}");
}

fn p2(lines: &[Line]) {
    let count = lines
        .iter()
        .filter(|line| {
            if check_line_safe(line) {
                return true;
            }
            let possible_lines = create_possible_lines(line);

            if possible_lines.iter().any(check_line_safe) {
                return true;
            }
            false
        })
        .count();
    println!("p2 count: {count}");
}

fn create_possible_lines(line: &Line) -> Vec<Line> {
    let mut vec: Vec<Line> = vec![];
    for n in 0..=line.reports.len() - 1 {
        let new_line = line.remove_index(n);
        vec.push(new_line);
    }
    vec
}

fn check_line_safe(line: &Line) -> bool {
    check_direction_safe(line) && distances_safe(line)
}
fn check_direction_safe(line: &Line) -> bool {
    let direction = check_direction(line);
    match direction {
        "asc" => true,
        "desc" => true,
        &_ => false,
    }
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
