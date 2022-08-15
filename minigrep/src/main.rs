use std::env;
use std::error::Error;

use minigrep::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::build(env::args())?;
    if let Err(e) = minigrep::run(config) {
        println!("Application error {e}!");
        return Err(e);
    }

    Ok(())
}
