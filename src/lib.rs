use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
    case_sensetive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(v) => v,
            None => return Err("query expected as first argument"),
        };
        let filename = match args.next() {
            Some(v) => v,
            None => return Err("filename expected as second argument"),
        };
        let case_sensetive = env::var("CASE_INSENSETIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensetive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = if config.case_sensetive {
        search(&config.query, &contents)
    } else {
        search_case_insensetive(&config.query, &contents)
    };
    for r in results {
        println!("{}", r);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensetive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str()))
        .collect()
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
