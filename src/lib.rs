pub mod api_structs;

use reqwest::{self};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use crate::api_structs::{albums, artists, playlists};
use base64::{engine::general_purpose, Engine};

#[derive(Debug, Deserialize)]
struct RefreshTokenResponse {
    access_token: String,
    #[serde(alias = "token_type")] _token_type: String,
    #[serde(alias = "expires_in")] _expires_in: u32,
    refresh_token: Option<String>, 
    #[serde(alias = "scope")] _scope: String, 
}

pub struct SpotifyAPI { 
    client_id: String,
    client_secret: String,
    token: String,
    refresh_token: String,
    http_client: reqwest::blocking::Client,
}

// TODO expand to saved songs and episodes too
impl SpotifyAPI {

    pub fn new() -> Self {
        // TODO implement some sort of token caching system
        let client_id = read_from_env_file("CLIENT_ID");
        let client_secret = read_from_env_file("CLIENT_SECRET");
        let token = read_from_env_file("TOKEN");
        let refresh_token = read_from_env_file("REFRESH_TOKEN");

        let http_client = reqwest::blocking::Client::new();

        Self {
            client_id,
            client_secret,
            token,
            refresh_token,
            http_client,
        }
    }
 
    // TODO implement generic function where you just need to pass a struct, the request url and a path and it
    // automatically does the request and data serialization to json file
    pub fn update_data<T>(&self, url: &str, path: &str) where T: Serialize + DeserializeOwned {
        let token_header = format!("Bearer {}", self.token);

        let res = self.http_client
            .get(url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", token_header)
            .send();

        let Ok(res) = res else {
            eprintln!("could not receive response");
            // TODO add a Result return type
            return;
        };

        if !res.status().is_success() {
            eprintln!("unsuccessful request, status: {}", res.status());
            return;
        }

        let body = res.json::<T>();

        let Ok(data) = body else {
            eprintln!("coult not deserialize json body");
            return;
        };

        api_structs::serialize_to_file(&data, path);

    }

    pub fn update_followed_artists(&self) {
        self.update_data::<artists::Followed>("https://api.spotify.com/v1/me/following?type=artist", "./data/followed_artists.json");
    }

    pub fn update_followed_playlists(&self) {
        self.update_data::<playlists::Followed>("https://api.spotify.com/v1/me/playlists", "./data/followed_playlists.json");
    }

    pub fn update_saved_albums(&self) {
        self.update_data::<albums::Saved>("https://api.spotify.com/v1/me/albums", "./data/saved_albums.json");
    }

    // TODO add a way to call refresh_token only if the previous one has expired
    // for example save the timestamp in the .env file and read it before requesting a refresh
    pub fn refresh_token(&mut self) {
        let auth_value = format!("{}:{}", self.client_id, self.client_secret); 
        let encoded_auth = general_purpose::STANDARD.encode(auth_value);

        let mut form_data = HashMap::new();
        form_data.insert("grant_type", "refresh_token");
        form_data.insert("refresh_token", &self.refresh_token);

        let res = self.http_client
            .post("https://accounts.spotify.com/api/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Basic {encoded_auth}"))
            .form(&form_data)
            .send();

        let Ok(res) = res else {
            eprintln!("could not send request");
            return;
        };

        if !res.status().is_success() {
            eprintln!("unsuccessful request, status: {}", res.status());
            return;
        }

        let res_json = res.json::<RefreshTokenResponse>();

        let Ok(res_json) = res_json else {
            eprintln!("could not deserialize json body");
            return;
        };

        self.token = res_json.access_token;
        eprintln!("new access token received");

        match res_json.refresh_token {
            Some(token) => {
                self.refresh_token = token;
            },
            None => eprintln!("no refresh token received"),
        };

        eprintln!("access token: {}", self.token);
        eprintln!("refresh token: {}", self.refresh_token);

    }

    // TODO convert search_type into a struct
    pub fn search_data(&self, search_term: &str, search_type: &str, limit: u32) {
        let token_header = format!("Bearer {}", self.token);
        let url = format!("https://api.spotify.com/v1/search?q={search_term}&type={search_type}&limit={limit}");

        let res = self.http_client
            .get(url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", token_header)
            .send();

        let Ok(res) = res else {
            eprintln!("could not receive response");
            // TODO add a Result return type
            return;
        };

        if !res.status().is_success() {
            eprintln!("unsuccessful request, status: {}", res.status());
            return;
        }

        //let body = res.json();
        let body = res.text().unwrap();
        println!("{body}");

        /*let Ok(data) = body else {
            eprintln!("coult not deserialize json body");
            return;
        };*/


    }

    // TODO implement a way to write the new tokens directly to the .env file
    //pub fn save_to_env_file(&self) { }

}

fn read_from_env_file(var_name: &str) -> String {
    dotenvy::var(var_name).unwrap_or_else(|_| {
        eprintln!("could not read {var_name} from .env file");
        String::new()
    })
}


