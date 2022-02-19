use std::{env, fs, process};
use std::error::Error;

struct Config<'a> {
    query: &'a str,
    query_lowercase: String,
    file_name: &'a str,
    sensitive: bool,
}

impl<'a> Config<'a> {
    fn new(args: &'a [String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("args length at least be 3");
        }
        let query = args[1].as_str();
        let query_lowercase = query.to_lowercase();
        let file_name = args[2].as_str();
        let sensitive = env::var("SENSITIVE").is_ok();
        return Ok(Config { query, query_lowercase, file_name, sensitive });
    }

    fn get_query(&self) -> &str {
        if self.sensitive {
            return self.query;
        }
        return self.query_lowercase.as_str();
    }

    fn get_file_name(&self) -> &str {
        return self.file_name;
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.get_file_name())?;
    for result in search(config.get_query(), &content.as_str()) {
        println!("{}", result);
    }
    Ok(())
}

fn search<'a>(query: &str, content: &'a &str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(&query) {
            result.push(line);
        }
    }
    return result;
}

//cargo run --bin grep test 1234
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|msg| {
        eprintln!("{}", msg);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("{}", e);
        process::exit(1)
    }
}