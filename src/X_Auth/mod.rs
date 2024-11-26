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
    let bearer_token = "AAAAAAAAAAAAAAAAAAAAAFqOxAEAAAAAupE3V9%2F1xD8%2BTKIa5JhQT6MxU%2Bs%3DCeoQtP9gJMFisvT4krqg3InItjY4WizoFPutOdbnUJi4j64nxC";
    let user_id = "MartinNkemakol1"; // Replace with the user ID whose followers you want to fetch
    let url = format!("https://api.x.com/1.1/followers/ids.json?screen_name={}", user_id);
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