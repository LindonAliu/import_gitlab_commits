use crate::types::Result;
use reqwest;
use std::collections::HashMap;

/// Contributions is a map with the date in format "YYYY-MM-DD" as key and the number of contributions as value
pub type Contributions = HashMap<String, usize>;

pub async fn get_contributions(gitlab_name: &str) -> Result<Contributions> {
    let url = format!("https://gitlab.com/users/{}/calendar.json", gitlab_name);
    let response = reqwest::get(&url).await?.json::<Contributions>().await?;

    Ok(response)
}
