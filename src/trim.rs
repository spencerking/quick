use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn trim(file: &File, cutoff: usize) {

    //println!("trim");
    
    let reader = BufReader::new(file);

    let mut seq_index = 1;

    let line1 = "".to_string();
    let mut line2 = "".to_string();
    let line3 = "".to_string();
    let mut line4 = "".to_string();
    
    for (line_number, line) in reader.lines().enumerate() {
	let line = line.unwrap();

	if seq_index == 1 || seq_index == 3 {
	    println!("{}", line.to_string());
	} else if seq_index == 2 {
	    line2 = line;
	} else if seq_index == 4 {
	    line4 = line;
	}

	seq_index = seq_index + 1;
	if seq_index > 4 {
	    let mut seq: Vec<char> = line2.chars().collect();
	    let mut quals: Vec<char> = line4.chars().collect();
	    let mut i = line4.chars().count() - 1;

	    while i >= 0 {
		//println!("{}", i);
		let value = quals[i] as usize - 33;
		//println!("{}", value);
		i = i - 1;
		if value < cutoff {
		    seq.pop();
		    quals.pop();
		} else {
		    break;
		}
	    }

	    let seq_str: String =  seq.into_iter().collect();
	    let qual_str: String = quals.into_iter().collect();

	    println!("{}", line1);
	    println!("{}", seq_str);
	    println!("{}", line3);
	    println!("{}", qual_str);
	    
	    seq_index = 1;
	}
    }
}
