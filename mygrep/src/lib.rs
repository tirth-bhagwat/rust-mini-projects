use std::fs;
#[derive(Debug)]
pub struct Config {
    pub file_path: String,
    pub query: String,
}
impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Min. 3 args required");
        }
        Ok(Config {
            file_path: args[1].clone(),
            query: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let file_str = fs::read_to_string(&config.file_path)?;
    // println!("{file_str}");

    let res = search(&config.query, &file_str);

    for line in res {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.cfg
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
