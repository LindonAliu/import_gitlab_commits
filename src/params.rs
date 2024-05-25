use crate::types::Result;
use std::env::Args;

pub struct Params {
    pub gitlab_name: String,
}

pub fn parse_args(args: Args) -> Result<Params> {
    let mut gitlab_name: Option<String> = None;
    let mut it = args.skip(1);

    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--gitlab-name" | "-gn" => {
                gitlab_name = Some(it.next().ok_or("Missing argument for -gn")?);
            }
            "--help" | "-h" => {
                println!("Usage:");
                
                std::process::exit(0);
            }
            _ => {
                return Err(format!("Unknown argument: {}", arg).into());
            }
        }
    }

    let gitlab_name = gitlab_name.ok_or("Missing argument: -gn")?;

    Ok(Params { gitlab_name })
}
