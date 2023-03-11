use std::fs;

#[derive(Debug)]
pub struct Config {
    pub path: String,
    pub query: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Three args required");
        }
        Ok(Config {
            path: args[1].to_string(),
            query: args[2].to_string(),
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(&config.path)?;

    for result in search(&text, &config.query) {
        println!("{result}");
    }
    Ok(())
}

fn search<'a>(text: &'a str, query: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();

    for line in text.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }

    res
}

#[cfg(test)]
mod test {
    use crate::search;

    #[test]
    fn test_search() {
        let text = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
No\
";
        let query = "no";

        assert_eq!(
            search(&text, &query),
            [
                "I'm nobody! Who are you?",
                "Are you nobody, too?",
                "They'd banish us, you know.",
            ]
        );
    }
}
