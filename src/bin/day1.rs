use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use itertools::Itertools;

#[derive(Clone, Default, Debug)]
struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    fn total(&self) -> u32 {
        self.calories.iter().sum()
    }
}
fn main() {
    let path = Path::new("./puzzle_inputs/day1.txt");
    let buffer = BufReader::new(File::open(path).unwrap());
    let elves: Vec<Elf> = buffer
        .lines()
        .into_iter()
        .filter_map(|x| x.ok())
        .group_by(|elem| !elem.is_empty())
        .into_iter()
        .filter(|(key, _)| *key)
        .map(|(_, group)| Elf {
            calories: group.filter_map(|s| s.parse::<u32>().ok()).collect(),
        })
        .collect();
    let top = elves.iter().map(|elf| elf.total()).max();
    let top_3: u32 = elves
        .iter()
        .map(|elf| elf.total())
        .sorted()
        .rev()
        .take(3)
        .sum();
    println!("Top elf carries {}", top.unwrap());
    println!("Top 3 elf carries {}", top_3);
}
