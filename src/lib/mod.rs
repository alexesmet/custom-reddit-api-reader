
use std::collections::HashMap;

use configuration::Configuration;
use reqwest;

pub mod configuration;
mod model;

#[derive(Debug)]
pub enum AppError {
    AuthRequestFailed(reqwest::Error),
    AuthResponseUnreadable(reqwest::Error),
    ReadRequestFailed(reqwest::Error),
    ReadResponseUnreadable(reqwest::Error),
    OutputFailed(csv::Error),
    IO(std::io::Error)
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

    let auth_response: model::AuthResponse = client.post("https://www.reddit.com/api/v1/access_token")
         .basic_auth(configuration.client_id, Some(configuration.secret_token))
         .form(&params)
         .send().await
         .map_err(|e| AppError::AuthRequestFailed(e))?
         .json().await
         .map_err(|e| AppError::AuthResponseUnreadable(e))?;

    let mut next_page = None;
    let mut writer = csv::Writer::from_path(configuration.output_path)
        .map_err(|e| AppError::OutputFailed(e))?;

    loop {
        let mut request_builder = client.get("https://oauth.reddit.com/r/unixporn/top")
            .header("Authorization", format!("Bearer {}", auth_response.access_token))
            .query(&[("t","year")]);

        if let Some(after) = next_page.take() {
            request_builder = request_builder.query(&[("after", after)]);
        }

        let response: model::ListingResponse = request_builder.send().await
            .map_err(|e| AppError::ReadRequestFailed(e))?
            .json().await
            .map_err(|e| AppError::ReadResponseUnreadable(e))?;

        next_page = response.data.after;

        for record in response.data.children.iter() {
            writer.serialize(&record.data)
                .map_err(|e| AppError::OutputFailed(e))?;
        }
        writer.flush() .map_err(|e| AppError::IO(e))?;
        if next_page.is_none() {
            break;
        }
    }

    Ok(())
}
