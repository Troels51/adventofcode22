use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use itertools::Itertools;


type SectionAssignment = ((u32,u32), (u32,u32));

fn is_contained_in(assignment1: (u32,u32), assignment2: (u32, u32)) -> bool
{
	(assignment1.0 <= assignment2.0) && (assignment1.1 >= assignment2.1)  
}

fn has_containment(assignments: &SectionAssignment) -> bool
{
	// One is contained in the other or the other is contained in the on
	is_contained_in(assignments.0, assignments.1) || is_contained_in(assignments.1, assignments.0)
}

fn has_overlap(assignments: &SectionAssignment) -> bool 
{
	assignments.0.0 >= assignments.1.0 && assignments.0.0 <= assignments.1.1 ||
	assignments.0.1 >= assignments.1.0 && assignments.0.1 <= assignments.1.1 ||
	assignments.1.0 >= assignments.0.0 && assignments.1.0 <= assignments.0.1 ||
	assignments.1.1 >= assignments.0.0 && assignments.1.1 <= assignments.0.1 
}


fn main() {
    let path = Path::new("./puzzle_inputs/day4.txt");
    let buffer = BufReader::new(File::open(path).unwrap());
    let assignments: Vec<SectionAssignment> = buffer
        .lines()
        .into_iter()
        .filter_map_ok(|line| -> Option<SectionAssignment>{
        		let (first, second) = line.split_once(',')?;
        		let first = first.split_once('-')?;
        		let second = second.split_once('-')?;
        			Some( ( (first.0.parse::<u32>().ok()?, first.1.parse::<u32>().ok()?),
        			(second.0.parse::<u32>().ok()?,second.1.parse::<u32>().ok()?)
        			))

    		})
        .filter_map(|x| x.ok())
        .collect();

    let contain_count = assignments.iter()
    	.filter(|assign| has_containment(assign))
    	.count();
    println!("The assignments contains {} amount of fully contained ranges", contain_count);

    let overlap_count = assignments.iter()
    	.filter(|assign| has_overlap(assign))
    	.count();
    println!("The assignments contains {} amount of overlapping ranges", overlap_count);

}
