use crate::api_structs;
use crate::api_structs::{Empty, Items, Deserialize, Serialize, ExternalUrls};
use crate::api_structs::artists::Artist;

/*
* ignored fields: href, limit, next, offset, previous
*/
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Saved {
    total: u32,
    items: Vec<SavedAlbumsItem>,
} 

impl Empty<Self> for Saved {
    fn empty() -> Self {
        Self {
            total: 0,
            items: vec![],
        }
    }
}

impl Items<SavedAlbumsItem> for Saved {
    fn items(self) -> Vec<SavedAlbumsItem> {
        self.items
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct SavedAlbumsItem {
    added_at: String,
    album: Album,
}

/*
* ignored fields: 
*/
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Album {
    #[serde(alias = "album_type")]
    _type: String,   // album or single
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
    name: String,
    total_tracks: u32,
    tracks: Tracks,

    #[serde(alias = "type")]
    obj_type: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct Tracks {
    items: Vec<TracksItem>,
    total: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct TracksItem {
    external_urls: ExternalUrls, 
    name: String,
}

pub fn debug_print_saved(albums: &Vec<SavedAlbumsItem>) {
    println!("\n===== ALBUMS =====\ntotal: {}", albums.len());
    println!("{:<50} | {:<11}", "name", "tot. tracks");
    println!("{}", "-".repeat(200));

    for a in albums {
        let a = &a.album;
        println!("{:<50} | {}", a.name, a.total_tracks);
    }
}

pub fn get_saved() -> Vec<SavedAlbumsItem> {
    get_saved_p("./data/saved_albums.json")
}

pub fn get_saved_p(file_path: &str) -> Vec<SavedAlbumsItem> {
    api_structs::deserialize_from_file::<Saved, SavedAlbumsItem>(file_path)
}
