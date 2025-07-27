mod lib;
use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);

    let mut source: Option<String> = None;

    if args.len() == 2 {
        let file_path = &args[1];
        source = Some(fs::read_to_string(file_path).unwrap()); 
    }
    
    let out = lib::run::run_script(source.unwrap());
    // println!("{:?}", out);
}
