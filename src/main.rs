use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::collections::HashMap;

mod args;
mod validation;

fn main() {
    let inputs: Vec<String> = env::args().collect();
    let options = args::parse(&inputs);

    let mut in_file = "".to_string();
    match options.get("input") {
	Some(v) => {
	    in_file = v.to_string();
	}
	None => {
	    eprintln!("error: {:?}", "no input file provided");
	    std::process::exit(1);
	}
    }
    
    let file = File::open(in_file).unwrap();
    //let reader = BufReader::new(file);

    validation::validate(&file);

    /*
    let mut seqInd = 1;
    for (index, line) in reader.lines().enumerate() {
	let mut line = line.unwrap();
	//println!("{}", line);

	validation::validate(&line, index, seqInd);
	
	seqInd = seqInd + 1;
	if seqInd > 4 {
	    seqInd = 1;
	}
    }
     */
}
