use import_gitlab_commits::contributions::{get_contributions, Contributions};
use import_gitlab_commits::params::{parse_args, Params};
use import_gitlab_commits::repository::create_new_repository;
use import_gitlab_commits::types::Result;

async fn run() -> Result<()> {
    let params: Params = parse_args(std::env::args())?;
    let contributions: Contributions = get_contributions(&params.gitlab_name).await?;

    create_new_repository(
        &params.gitlab_name,
        &params.name,
        &params.email,
        contributions,
    )?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
