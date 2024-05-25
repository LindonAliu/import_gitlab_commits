use crate::contributions::Contributions;
use crate::types::Result;
use chrono::NaiveDate;
use git2::{Repository, Signature, Time};

fn add_contributions_to_repository(
    name: &str,
    email: &str,
    repo: &Repository,
    contributions: Contributions,
) -> Result<()> {
    for contribution in contributions {
        for min in 0..contribution.1 {
            let naive_date = NaiveDate::parse_from_str(contribution.0.as_str(), "%Y-%m-%d")?;
            let nb = min as u32 + 1;
            let datetime = naive_date.and_hms_opt(0, nb, 0).ok_or("Invalid time")?;
            let timestamp = datetime.and_utc().timestamp();
            let time = Time::new(timestamp, 0);
            let author = Signature::new(name, email, &time)?;
            let committer = author.clone();

            let tree_id = {
                let mut index = repo.index()?;
                index.write_tree()?
            };

            let tree = repo.find_tree(tree_id)?;
            let parent = repo.head()?.peel_to_commit()?;
            repo.commit(
                Some("HEAD"),
                &author,
                &committer,
                format!("Commit {}", nb).as_str(),
                &tree,
                &[&parent],
            )?;
        }
    }

    Ok(())
}

fn create_initial_commit(repo: &Repository) -> Result<()> {
    let sig = repo.signature()?;

    let tree_id = {
        let mut index = repo.index()?;

        index.write_tree()?
    };

    let tree = repo.find_tree(tree_id)?;

    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;

    Ok(())
}

pub fn create_new_repository(
    gitlab_name: &str,
    name: &str,
    email: &str,
    contributions: Contributions,
) -> Result<()> {
    let repository_name = format!("{}-contributions", gitlab_name);

    let repo = Repository::init(repository_name)?;

    create_initial_commit(&repo)?;
    add_contributions_to_repository(name, email, &repo, contributions)?;
    Ok(())
}
