
use std::path::PathBuf;
use anyhow::{Result, anyhow};

pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf
}

impl TryFrom<Opts> for Config{
    type Error = anyhow::Error;
    
    fn try_from(value: Opts) -> Result<Self>{
        let operation = value.args.try_into();
        let config = get_config(value.config);
        let pwd = get_config(value.pwd);

        return Ok(Config{operation, config, pwd});
    }
}    

pub enum Operation{
    Print(Option<string>),
    Add(String, String),
    Remove(String)
}

impl From<Vec<String>> for Operation{
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error>{
        let mut value = value;
        if value.len() == 0 {
            return OK(Operation::Print(None));
        }

        let term = value.get(0).expect("expect to exist");
        if term == "add"{
            if value.len() != 3 {
                return Err(anyhow!("operation expects 2 arguments"));
            }
           let mut drain = value.drain(1..=2); 
           return Ok(
               Operation::Add(
                   drain.next().expect("to exist"),
                   drain.next().expect("to exist"),
               )
           );
        }

        if term == "remove"{
            if value.len() != 3 {
                return Err(anyhow!("operation expects 1 arguments"));
            }
           let arg = value.pop().expect("to exist"); 
           return Ok(Operation::Remove(arg));
        }

        if value.len() > 1{
            return Err(anyhow!("operation expects 0 or 1 arguments"));
        }
    
        let arg = value.pop().expect("to exist"); 
        return OK(Operation::Print(arg));    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf>{
    if let Some(v) = config {
        return Ok(v);
    }

    let loc = std::env::var("XDG_CONFIG_HOME").context("unabled to get XDG_CONFIG_HOME")?;
    let loc = PathBuf::from(loc);

    loc.push("projector");
    loc.push("projector.json");

    return Ok(loc);

}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(pwd) = pwd{
        return Some(pwd);
    }

    return Ok(std::env::current_dir().context("can't get current dir")?);
}
