#![deny(clippy::all)]
use std::fs::read_to_string;
struct Line(String, String);
impl Line {
    fn to_numbers(&self) -> (i32, i32) {
        (self.0.parse().unwrap(), self.1.parse().unwrap())
    }
}
#[derive(Debug)]
struct Lists {
    l1: Vec<i32>,
    l2: Vec<i32>,
}
impl Lists {
    fn insert(&mut self, v1: i32, v2: i32) {
        self.l1.push(v1);
        self.l2.push(v2);
    }
    fn sort(&mut self) {
        self.l1.sort();
        self.l2.sort();
    }
    fn calculate_total_distance(&self) -> i32 {
        let i1 = self.l1.iter();
        let mut i2 = self.l2.iter();
        let mut sum = 0;
        for v1 in i1 {
            let v2 = i2.next().unwrap();
            sum += (v1 - v2).abs()
        }
        sum
    }
}

fn read_lines(filename: &str) -> Vec<Line> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line: &str| {
            let vec: Vec<&str> = line.split_whitespace().collect();
            Line(vec[0].to_string(), vec[1].to_string())
        })
        .collect()
}
fn main() {
    let lines = read_lines("input.txt");
    let mut lists = Lists {
        l1: vec![],
        l2: vec![],
    };
    lines.iter().for_each(|line| {
        let (v1, v2) = line.to_numbers();
        lists.insert(v1, v2);
    });
    lists.sort();
    let total_distance = lists.calculate_total_distance();
    println!("total distance: {total_distance}");
    let similarity = part2(lists);
    println!("similarity: {similarity}");
}

fn part2(lists: Lists) -> i32 {
    let total = lists.l1.iter().fold(0, |acc, num| {
        let instances = count_instances(&lists.l2, num);
        acc + num * instances
    });
    total
}

fn count_instances(list: &[i32], number: &i32) -> i32 {
    list.iter()
        .filter(|num| num == &number)
        .count()
        .try_into()
        .unwrap()
}
