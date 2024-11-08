pub mod playlists;
pub mod artists;
pub mod albums;

// by declaring it in there, every submodule that uses crate::api_structs::* will have access to Deserialize
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use core::fmt;
use std::{fs::File, io::BufReader, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchDataItem<T> {
    pub href: String,
    pub items: Vec<T>,
    pub limit: u32,
    pub next: Option<String>,
    pub offset: u32,
    pub previous: Option<String>,
    pub total: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub display_name: String,
    pub href: String,
    pub id: String,
    #[serde(alias = "type")] pub obj_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub url: String,
}

pub trait Empty<T> {
    fn empty() -> T;
}

pub trait Items<T> {
    fn items(self) -> Vec<T>;
}

pub enum SearchType {
    Playlist,
    Album,
}

impl fmt::Display for SearchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Playlist => write!(f, "playlist"),
            Self::Album => write!(f, "album"),
        }
    }
}

pub fn serialize_to_file<T>(data: T, path: &str) where T: Serialize {
    // create the data/ folder if it does not exists
    if !Path::new("data").is_dir() {
        match std::fs::create_dir("data") {
            Ok(()) => println!("created data/ folder"),
            Err(err) => eprintln!("{err}"),
        };
    }

    // serialize data to a formatted and readable json string
    let Ok(str) = serde_json::to_string_pretty(&data) else {
        eprintln!("could not serialize data of type {}", std::any::type_name_of_val(&data));
        return;
    };

    // a fancy way of checking if the data has been written to the specified file
    if matches!(std::fs::write(path, str), Ok(())) { } else {
        eprintln!("could not write serialized data to file with path: {path}");
    }

}

// generic function to serialize a struct of type T to a file that implements the Empty trait and has an internal vector of structs of type R
pub fn deserialize_from_file<T, R>(file_path: &str) -> Vec<R> where T: DeserializeOwned + Empty<T> + Items<R>, R: DeserializeOwned {

    let Ok(file) = File::open(file_path) else { 
        eprintln!("couldn't open file at path: {file_path}");
        return vec![];
    };

    let reader = BufReader::new(file);

    let data: T = serde_json::from_reader(reader)
        .unwrap_or_else(|_| {
            eprintln!("couldn't deserialize data from json file");
            T::empty()
    });

    data.items()
}
