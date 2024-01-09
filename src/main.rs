use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let config = Config::new(&args);

    print!("Searching for {} ", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Shoudl be able to read the file");

    println!("With text:\n{}", contents)
}

struct Config{
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("参数数量不足，应有2个参数")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config {query, file_path}
    }
}
