use hello_world::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = hello_world::run(config) {
        println!("application error:{}", e);
        process::exit(1);
    }
}
