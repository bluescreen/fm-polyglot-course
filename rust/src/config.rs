use crate::opts::Opts;
use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self> {
        let operation = value.args.try_into()?;
        let config = get_config(value.config)?;
        let pwd = get_pwd(value.pwd)?;

        return Ok(Config {
            operation,
            config,
            pwd,
        });
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut value = value;
        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }

        let term = value.get(0).expect("expect to exist");
        if term == "add" {
            if value.len() != 3 {
                return Err(anyhow!("operation expects 2 arguments"));
            }
            let mut drain = value.drain(1..=2);
            return Ok(Operation::Add(
                drain.next().expect("to exist"),
                drain.next().expect("to exist"),
            ));
        }

        if term == "remove" {
            if value.len() != 3 {
                return Err(anyhow!("operation expects 1 arguments"));
            }
            let arg = value.pop().expect("to exist");
            return Ok(Operation::Remove(arg));
        }

        if value.len() > 1 {
            return Err(anyhow!("operation expects 0 or 1 arguments"));
        }

        let arg = value.pop().expect("to exist");
        return Ok(Operation::Print(Some(arg)));
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }

    let loc = std::env::var("XDG_CONFIG_HOME").context("unabled to get XDG_CONFIG_HOME")?;
    let mut loc = PathBuf::from(loc);

    loc.push("projector");
    loc.push("projector.json");

    return Ok(loc);
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(pwd) = pwd {
        return Ok(pwd);
    }

    return Ok(std::env::current_dir().context("can't get current dir")?);
}

#[cfg(test)]
mod test {
    use super::Config;
    use crate::{config::Operation, opts::Opts};
    use anyhow::Result;

    #[test]
    fn test_it_should_print_all() -> Result<()> {
        let opts: Config = Opts {
            args: vec![],
            pwd: None,
            config: None,
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Print(None));
        Ok(())
    }

    // #[test]
    // fn test_it_should_print_key() -> Result<()>{
    //     let opts: Config= Opts{
    //         args: vec!["foo".into()],
    //         pwd: None,
    //         config: None
    //     }.try_into()?;

    //     assert_eq!(opts.operation, Operation::Print(Some("foo".into())));
    //     Ok(())
    // }

    // #[test]
    // fn test_it_should_add_key_value() -> Result<()>{
    //     let opts: Config= Opts{
    //         args: vec!["add".into(),"foo".into(),"bar".into()],
    //         pwd: None,
    //         config: None
    //     }.try_into()?;

    //     assert_eq!(opts.operation, Operation::Add(
    //             String::from("add"),
    //             String::from("foo"),
    //     ));
    //     Ok(())
    // }

    // #[test]
    // fn test_it_should_remove_key_value() -> Result<()>{
    //     let opts: Config= Opts{
    //         args: vec![String::from("rm"),String::from("foo")],
    //         pwd: None,
    //         config: None
    //     }.try_into()?;

    //     assert_eq!(opts.operation, Operation::Remove(
    //             String::from("remove"),
    //     ));
    //     Ok(())
    // }
}
