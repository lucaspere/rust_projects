//! Minigrep crate

//!

//! Essa biblioteca simula a funcionalidade básica do `grep program`

use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Não recebi a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Não recebi um nome de arquivo"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

/// ### Descrição
/// Essa função executa a busca de acordo com o argumento [`Config`] passado
///
/// Retorna um [`Result<(), Box<dyn Error>>`]

/// ### Exemplo
/// ```
/// fn main() {
///     let config = minigrep::Config{
///            case_sensitive: true,
///            filename: "cronograma.txt".to_string(),
///            query: "feriado".to_string()
///        };

///     let result = minigrep::run(config).is_ok();

///     assert!(result);

/// }
/// ```

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_match_case_sensitive() {
        let query = "teste";
        let contents = r#"
9 03/03 Teste Funcional
10 09/03 Teste Estrutural
11 10/03 teste Estrutural
12 16/03 Apresentação TP 1
13 17/03 Apresentação TP 1
14 23/03 Revisão Prova 1
15 24/03 Prova 1"#;

        assert_eq!(vec!["11 10/03 teste Estrutural"], search(query, contents));
    }

    #[test]
    fn should_match_case_insensitive() {
        let query = "teste";
        let contents = r#"
9 03/03 Teste Funcional
10 09/03 Teste Estrutural
11 10/03 teste Estrutural
12 16/03 Apresentação TP 1
13 17/03 Apresentação TP 1
14 23/03 Revisão Prova 1
15 24/03 Prova 1"#;

        assert_eq!(
            vec![
                "9 03/03 Teste Funcional",
                "10 09/03 Teste Estrutural",
                "11 10/03 teste Estrutural"
            ],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    #[ignore]
    fn should_false() {
        assert_eq!(true, false);
    }
}
