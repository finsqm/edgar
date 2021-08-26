use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub path: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't receive a path argument :("),
        };
        Ok(Config { path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;

    println!("Here's the bad stuff:\n{}", contents);

    Ok(())
}

pub struct SqlAST {
    pub root: ASTNode,
}

pub struct ASTNode {
    pub children: Vec<ASTNode>,
    pub label: String,
}

impl SqlAST {
    pub fn parse(sql_string: &String) -> SqlAST {
        SqlAST {
            root: ASTNode {
                children: Vec::new(),
                label: "root".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn name() {
        let sql_string = "SELECT * FROM t";
        let ast = SqlAST::parse(&sql_string.to_string());
        assert_eq!(ast.root.label, "root".to_string());
    }
}
