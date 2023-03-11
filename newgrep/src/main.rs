use newgrep::Config;
use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|e| {
        println!("Error parsing argurments: {e}");
        process::exit(1);
    });

    newgrep::run(&config).expect("Cannot read file ");

    
}
