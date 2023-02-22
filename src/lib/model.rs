use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AuthRequestBody {
    pub grant_type: String,
    pub username: String,
    pub password: String
}

#[derive(Deserialize,Debug)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64
}

#[derive(Deserialize,Debug)]
pub struct ListingResponse {
    pub kind: String,
    pub data: ListingResponseData
}

#[derive(Deserialize,Debug)]
pub struct ListingResponseData {
    pub before: Option<String>,
    pub after: Option<String>,
    pub children: Vec<PostResponse>
}

#[derive(Deserialize,Debug)]
pub struct PostResponse {
    pub kind: String,
    pub data: PostResponseData
}

#[derive(Deserialize,Serialize,Debug)]
pub struct PostResponseData {
    pub title: String,
    pub downs: i32,
    pub ups: i32,
    pub total_awards_received: i32
}
