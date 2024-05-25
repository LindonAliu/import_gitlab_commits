use crate::types::Result;
use reqwest;
use std::collections::HashMap;

/// Contributions is a map with the date in format "YYYY-MM-DD" as key and the number of contributions as value
pub type Contributions = HashMap<String, usize>;

// call this API to get the contributions for a given GitLab user
// curl "https://gitlab.com/users/<username>/calendar.json"
// Example:
// curl "https://gitlab.com/users/username/calendar.json"
// {
//   "2021-01-01": 1,
//   "2021-01-02": 2,
//   "2021-01-03": 3
// }
// The API returns a JSON object with the date in format "YYYY-MM-DD" as key and the number of contributions as value
// Transform the JSON object into a Contributions map whith serde and return it
pub async fn get_contributions(gitlab_name: &str) -> Result<Contributions> {
    let url = format!("https://gitlab.com/users/{}/calendar.json", gitlab_name);
    let response = reqwest::get(&url).await?.json::<Contributions>().await?;

    Ok(response)
}
