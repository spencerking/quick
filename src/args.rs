use std::collections::HashMap;

pub fn parse(args: &Vec<String>) -> HashMap<String, String> {
    let mut options: HashMap<String, String> = HashMap::new();

    //println!("{:?}", args);
    //println!("parse args");

    let mut i = 1;
    while i < args.len() {
	if args[i] == "-i" {
	    i = i + 1;
	    options.insert("input".to_string(), args[i].to_string());
	}
	i = i + 1
    }
    
    return options;
}
