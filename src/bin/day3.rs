use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use itertools::Itertools;

type RuckSack = (String, String);

type ElfGroup = (String, String, String);


fn common_type(rucksack: &RuckSack) -> Option<char> {
    for c in rucksack.0.chars() {
        if rucksack.1.contains(c) {
            return Some(c);
        }
    }
    None
}

fn common_type_groups(elf_group: &ElfGroup) ->Option<char>
{
    for c in elf_group.0.chars() {
        if elf_group.1.contains(c) && elf_group.2.contains(c) {
            return Some(c);
        }
    }
    None

}
fn priority(item: char) -> u32 {
    let value = item as u32;
    if item.is_lowercase() {
        return value - 96;
    }
    if item.is_uppercase() {
        return value - 65 + 27
    }
    0
}

fn main() {
    let path = Path::new("./puzzle_inputs/day3.txt");
    let buffer = BufReader::new(File::open(path).unwrap());
    let rucksacks: Vec<RuckSack> = buffer
        .lines()
        .map_ok(|line| {
            let split = line.split_at(line.len()/2);
            (String::from(split.0), String::from(split.1))
        })
        .filter_map(|i| i.ok())
        .collect();
    let sum_prio : u32 = rucksacks.iter()
        .map(common_type)
        .filter_map(|c| c)
        .map(priority)
        .sum();
    println!("The sum of priorities of items is: {}", sum_prio);

    // Part 2
    let buffer = BufReader::new(File::open(path).unwrap());
    let elf_groups: Vec<ElfGroup> = buffer
        .lines()
        .filter_map(|i| i.ok())
        .tuples()
        .collect();
    let sum_prio : u32 = elf_groups.iter()
        .map(common_type_groups)
        .filter_map(|c| c)
        .map(priority)
        .sum();
    println!("The sum of priorities of elf groups is: {}", sum_prio);

}

#[test]
fn given_test() {
    let test_sets = [("vJrwpWtwJgWrhcsFMMfFFhFp", 16), ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 38), ("PmmdzqPrVvPwwTWBwg", 42)];
    for (test_set, res) in test_sets {
        let rucksack = test_set.split_at(test_set.len()/2);
        let rucksack = (String::from(rucksack.0), String::from(rucksack.1));
        let common_type = common_type(&rucksack);
        let prio = priority(common_type.unwrap());
        assert!(prio == res);        
    }
}
#[test]
fn prio_test(){
    assert!(priority('p') == 16);
    assert!(priority('v') == 22);
    assert!(priority('t') == 20);
    assert!(priority('s') == 19);
    assert!(priority('P') == 42);
    assert!(priority('L') == 38);

}