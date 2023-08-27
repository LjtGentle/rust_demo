use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub fn demo() {
    test03();
}

fn test01() {
    let args: Vec<String> = env::args().collect();
    println!("args:{:?}", args);
}

fn test02() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("args len less 3");
        return;
    }
    let query = &args[1];
    let filename = &args[2];

    println!("searching for {}", query);
    println!("in file {}", filename);

    let contents = fs::read_to_string(filename).expect("get some want reading in file");
    println!("test:{contents}");
}

fn test03() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        // println!("problem parsing args:{}", err);
        // 将错误信息写入标准错误
        eprintln!("problem parsing args:{}", err);
        process::exit(1);
    });
    if let Err(e) = config.run() {
        // println!("app err:{}", e);
        eprintln!("app err:{}", e);
        process::exit(1);
    }
}

fn test04() {
    let config = Config::new02(env::args()).unwrap_or_else(|err| {
        // 将错误信息写入标准错误
        eprintln!("problem parsing args:{}", err);
        process::exit(1);
    });
    if let Err(e) = config.run() {
        eprintln!("app err:{}", e);
        process::exit(1);
    }
}
struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
    pub fn new02(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Don't get query param"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Don't get filename param"),
        };
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(self.filename.clone())?;

        let result = if self.case_sensitive {
            search(&self.query, &contents)
        } else {
            search_case_insenitive02(&self.query, &contents)
        };
        for line in result {
            println!("line :{line}");
        }
        Ok(())
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

// ignore letter case
pub fn search_case_insenitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();
    // let contents = contents.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line)
        }
    }
    result
}

pub fn search_case_insenitive02<'a>(query: &str, cotents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    cotents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
