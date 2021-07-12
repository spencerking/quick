use std::fs::File;
use std::io::{BufRead, BufReader};

// TODO: Finish implementing validation criteria
// Criteria: https://genome.sph.umich.edu/wiki/FastQ_Validation_Criteria

pub fn validate(file: &File) -> bool {
    let mut is_valid = true;
    let valid_bases = valid_bases();
    let mut seq_length = 0;
    
    let reader = BufReader::new(file);

    let mut seq_index = 1;
    for (line_number, line) in reader.lines().enumerate() {
	let line = line.unwrap();

	if seq_index == 1 {
	    if line.chars().count() < 2 {
		is_valid = false;
		println!("Error: Line number {} is too short", line_number);
	    }

	    if line.chars().next().unwrap() != '@' {
		is_valid = false;
		println!("Error: Line number {} must start with an @", line_number);
	    }
	} else if seq_index == 2 {
	    if line.chars().count() <= 0 {
		is_valid = false;
		println!("Error: Line number {} is too short", line_number);
	    }

	    seq_length = line.chars().count();

	    for c in line.chars() {
		if !valid_bases.contains(&c) {
		    is_valid = false;
		    println!("Error: Line number {} contains an invalid base {}", line_number, c);
		}
	    }
	    
	} else if seq_index == 3 {
	    if line.chars().next().unwrap() != '+' {
		is_valid = false;
		println!("Error: Line number {} must start with a +", line_number);
	    }
	} else if seq_index == 4 {
	    if line.chars().count() != seq_length {
		is_valid = false;
		println!("Error: Line number {}'s length does not match line number {}", line_number, line_number-2);
	    }

	    for c in line.chars() {
		if !is_valid_quality(c) {
		    is_valid = false;
		    println!("Error: Line number {} contains an invalid quality {}", line_number, c);
		}
	    }
	}

	seq_index = seq_index + 1;
	if seq_index > 4 {
	    seq_index = 1;
	}
    }

    return is_valid;
}

fn valid_bases() -> Vec<char> {
     return ['A', 'C', 'T', 'G', 'N', 'a', 'c', 't', 'g', 'n'].to_vec();
}

fn is_valid_quality(c: char) -> bool {
    if c as usize > 32 {
	return true;
    }
    return false;
}

pub fn quality_scores() {
    println!("Quality scores");
}
