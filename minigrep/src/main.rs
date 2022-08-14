use std::env;
use std::error::Error;

use minigrep::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)?;
    if let Err(e) = minigrep::run(config) {
        println!("Application error {e}!");
        return Err(e);
    }

    Ok(())
}
