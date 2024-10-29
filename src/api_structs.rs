pub mod playlists;
pub mod artists;
pub mod albums;

use serde::{de::DeserializeOwned, Deserialize, Serialize};     // by declaring it in there, every submodule that uses crate::api_structs::* will have access to Deserialize
use std::{fs::File, io::BufReader};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalUrls {
    pub spotify: String,    // this is the url to be passed to zotify
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub display_name: String,
    pub href: String,
    pub id: String,
    #[serde(alias = "type")] pub obj_type: String,
}

pub trait Empty<T> {
    fn empty() -> T;
}

pub trait Items<T> {
    fn items(self) -> Vec<T>;
}

// TODO create the data folder if it doesn't exists (right now it would trigger the second error in the function)
pub fn serialize_to_file<T>(data: T, path: &str) where T: Serialize {
    let Ok(str) = serde_json::to_string_pretty(&data) else {
        eprintln!("could not serialize data of type {}", std::any::type_name_of_val(&data));
        return;
    };

    // a fancy way of checking if the data has been written to the specified file
    if matches!(std::fs::write(path, str), Ok(())) { } else {
        eprintln!("could not write serialized data to file with path: {path}");
    }

}

pub fn deserialize_from_file<T, R>(file_path: &str) -> Vec<R> where T: DeserializeOwned + Empty<T> + Items<R> {

    let Ok(file) = File::open(file_path) else { 
        eprintln!("couldn't open followed_artists.json");
        return vec![];
    };

    let reader = BufReader::new(file);

    let data: T = serde_json::from_reader(reader)
        .unwrap_or_else(|_| {
            eprintln!("couldn't deserialize followed artists from json file");
            T::empty()
    });

    data.items()
}
