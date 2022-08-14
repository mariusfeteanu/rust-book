use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)?;

    println!("Will search for {} in {}.", config.query, config.file_path);

    let file_contents = fs::read_to_string(config.file_path).expect("Expected file did not exist.");

    println!("File contents: \n{file_contents}\n");

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        return Ok(Config { query, file_path });
    }
}
