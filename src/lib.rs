use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Couldn't parse config: not enough arguments passed.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    for r in search(&config.query, &contents) {
        println!("{}", r);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

fn search_case_insensetive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensetive() {
        let query = "не";
        let contents = "\
Мой дядя самых честных правил,
Когда не в шутку занемог,
Он уважать себя заставил
И лучше выдумать не мог.
Ещё одна строка
Чтобы проверить
что тесты НЕ сломались";
        let result = vec!["Когда не в шутку занемог,", "И лучше выдумать не мог."];
        assert_eq!(search(query, contents), result);
    }

    #[test]
    fn case_insensetive() {
        let query = "ДяДя";
        let contents = "\
Мой дядя самых честных правил,
Когда не в шутку занемог,
Он уважать себя заставил
И лучше выдумать не мог.";
        let result = vec!["Мой дядя самых честных правил,"];
        assert_eq!(search_case_insensetive(query, contents), result);
    }
}
