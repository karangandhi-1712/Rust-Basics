use std::env;
use std::process;

use minigrep::Config;   

fn main() 
{
    let args: Vec<String> = env::args().collect();


    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    let mut results=Vec::new();
    for line in contents.lines()
    {
        if line.contains(query)
        {
            results.push(line);
        }
    }
    results 
}
