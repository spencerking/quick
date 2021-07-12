use std::fs::File;
use std::io::{BufRead, BufReader};

// TODO: Finish implementing validation criteria
// Criteria: https://genome.sph.umich.edu/wiki/FastQ_Validation_Criteria

pub fn validate(file: &File) {
//pub fn validate(line: &str, lineNumber: usize, seqIndex: usize) {
    //println!("{}", lineNumber);
    //println!("{}", seqIndex);
    //println!("Validate");

    let reader = BufReader::new(file);

    let mut seqIndex = 1;
    for (lineNumber, line) in reader.lines().enumerate() {
	let mut line = line.unwrap();
	    
	if seqIndex == 1 {
	    if line.chars().count() < 2 {
		println!("Error: Line number {} is too short", lineNumber);
	    }

	    if line.chars().next().unwrap() != '@' {
		println!("Error: Line number {} must start with an @", lineNumber);
	    }
	} else if seqIndex == 2 {

	} else if seqIndex == 3 {
	    if line.chars().next().unwrap() != '+' {
		println!("Error: Line number {} must start with a +", lineNumber);
	    }
	} else if seqIndex == 4 {

	}

	seqIndex = seqIndex + 1;
	if seqIndex > 4 {
	    seqIndex = 1;
	}
    }
}

pub fn quality_scores() {
    println!("Quality scores");
}
