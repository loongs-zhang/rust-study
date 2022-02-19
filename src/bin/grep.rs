use std::{env, fs, process};
use std::error::Error;

struct Config<'a> {
    query: &'a str,
    file_name: &'a str,
    sensitive: bool,
}

impl<'a> Config<'a> {
    fn new(args: &'a [String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("args length at least be 3");
        }
        let query = args[1].as_str();
        let file_name = args[2].as_str();
        //默认大小写不敏感，即不加环境变量
        let sensitive = env::var("SENSITIVE").is_ok();
        return Ok(Config { query, file_name, sensitive });
    }

    fn get_query(&self) -> &str {
        return self.query;
    }

    fn get_file_name(&self) -> &str {
        return self.file_name;
    }

    fn is_sensitive(&self) -> bool {
        return self.sensitive;
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.get_file_name())?;
    for result in search(config.get_query(), &content.as_str(),
                         config.is_sensitive()) {
        println!("{}", result);
    }
    Ok(())
}

fn search<'a>(query: &str, content: &'a &str, sensitive: bool) -> Vec<&'a str> {
    let mut result = Vec::new();
    if sensitive {
        //大小写敏感
        for line in content.lines() {
            if line.contains(query) {
                result.push(line);
            }
        }
    } else {
        //默认大小写不敏感
        let query = query.to_lowercase();
        for line in content.lines() {
            if line.to_lowercase().contains(&query) {
                result.push(line);
            }
        }
    }
    return result;
}

//SENSITIVE=0 cargo run --bin grep To poem.txt
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