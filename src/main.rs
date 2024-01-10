use std::env;
use tsvlook::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    print!("Searching for {} ", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = tsvlook::run(config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    }
}


