
use std::collections::HashMap;

use configuration::Configuration;
use reqwest;
use serde::{Deserialize, Serialize};

pub mod configuration;


#[derive(Debug)]
pub enum AppError {
    AuthRequestFailed(reqwest::Error),
    AuthResponseUnreadable(reqwest::Error),
    ReadRequestFailed(reqwest::Error),
    ReadResponseUnreadable(reqwest::Error)
}


#[derive(Serialize)]
struct AuthRequestBody {
    grant_type: String,
    username: String,
    password: String
}

#[derive(Deserialize,Debug)]
struct AuthResponse {
    access_token: String,
    token_type: String,
    expires_in: i64
}


pub async fn run(configuration: Configuration) -> Result<(), AppError> {
    let client = reqwest::Client::builder()
        .user_agent("custom-api-reader/0.1.0")
        .build()
        .unwrap(); // unwrap beacuse this code is static

    let mut params = HashMap::new();
    params.insert("grant_type", "password");
    params.insert("username", &configuration.username);
    params.insert("password", &configuration.password);

    let auth_response: AuthResponse = client.post("https://www.reddit.com/api/v1/access_token")
         .basic_auth(configuration.client_id, Some(configuration.secret_token))
         .form(&params)
         .send().await
         .map_err(|e| AppError::AuthRequestFailed(e))?
         .json().await
         .map_err(|e| AppError::AuthResponseUnreadable(e))?;

    let response = client.get("https://oauth.reddit.com/r/unixporn/top")
        .header("Authorization", format!("Bearer {}", auth_response.access_token))
        .query(&[("t","year")])
        .send().await
        .map_err(|e| AppError::ReadRequestFailed(e))?
        .text().await
        .map_err(|e| AppError::ReadResponseUnreadable(e))?;

    println!("{}", response);


    Ok(())
}
