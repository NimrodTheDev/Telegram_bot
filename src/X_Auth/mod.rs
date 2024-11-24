use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Follower {
    id: String,
    name: String,
    username: String,
}

#[derive(Deserialize, Debug)]
struct FollowerResponse {
    data: Vec<Follower>,
    // Optional fields like meta can be included as needed
}

pub async fn twitter() -> Result<(), Box<dyn std::error::Error>> {
    // Replace with your Bearer token
    let bearer_token = "YOUR_BEARER_TOKEN";
    let user_id = "YOUR_USER_ID"; // Replace with the user ID whose followers you want to fetch

    let url = format!("https://api.twitter.com/2/users/{}/followers", user_id);
    let client = Client::new();

    let response = client
        .get(&url)
        .bearer_auth(bearer_token)
        .send()
        .await?;

    if response.status().is_success() {
        let followers: FollowerResponse = response.json().await?;
        println!("{:?}", followers);
    } else {
        eprintln!("Failed to fetch followers: {:?}", response.text().await?);
    }

    Ok(())
}