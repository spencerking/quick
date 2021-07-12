use std::env;
use std::fs::File;
// use std::io::{BufRead, BufReader};
// use std::collections::HashMap;

mod args;
mod validation;
mod trim;

fn main() {
    let inputs: Vec<String> = env::args().collect();
    let options = args::parse(&inputs);

    let mut in_file = "";
    match options.get("input") {
	Some(v) => {
	    in_file = v;
	}
	None => {
	    eprintln!("error: {:?}", "no input file provided");
	    std::process::exit(1);
	}
    }

    let mut skip_validation = false;
    match options.get("skip-validation") {
	Some(v) => {
	    skip_validation = true;
	}
	None => {
	}
    }
    
    let mut is_valid = true;
    if !skip_validation {
	let file = File::open(in_file).unwrap();
	is_valid = validation::validate(&file);
    }
    
    if is_valid {
	//println!("Now do other stuff");
	let file = File::open(in_file).unwrap();
	trim::trim(&file, 30);
    }
}
