use std::env;
use std::fs;
use std::process;

fn main() {
    //println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    //let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments:{err}");
        process::exit(1);
    } );
    //dbg!(args);
    // let query = &args[1];
    // let file_path = &args[2];
    println!("searching for: {}", config.query);
    println!("file path: {}", config.file_path);

    // let contents = fs::read_to_string(file_path)
    //                     .expect("should have been able to read file.");
    // println!("with text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String])-> Config{
        if args.len() < 3 {
            panic!("not enough arguments!")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config{query,file_path}
    }

    fn build(args: &[String])-> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("not enough arguments!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok( Config{query,file_path} )
    }
}
