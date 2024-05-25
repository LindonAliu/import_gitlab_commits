use import_gitlab_commits::contributions::{get_contributions, Contributions};
use import_gitlab_commits::params::{parse_args, Params};
use import_gitlab_commits::types::Result;

async fn run() -> Result<()> {
    let params: Params = parse_args(std::env::args())?;
    let contributions: Contributions = get_contributions(&params.gitlab_name).await?;

    println!("contributions: {:?}", contributions);

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
