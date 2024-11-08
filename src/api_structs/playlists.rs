use crate::api_structs;
use crate::api_structs::{Empty, Items, Image, Deserialize, Serialize, ExternalUrls, Owner};

use super::SearchDataItem;

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchData {
    pub playlists: api_structs::SearchDataItem<Playlist>,
}

impl Default for SearchDataItem<Playlist> {
    fn default() -> Self {
        Self {
            href: String::from(""),
            next: None,
            limit: 0,
            offset: 0,
            previous: None,
            items: vec![],
            total: 0,
        }
    }
}

impl Default for SearchData {
    fn default() -> Self {
        Self {
            playlists: SearchDataItem::default()
        }
    }
}

/*
* ignored fields: href, limit, next, offset, previous
*/
#[derive(Debug, Deserialize, Serialize)]
pub struct Followed {
    items: Vec<Playlist>,
    total : u32,
}

impl Empty<Self> for Followed {
    fn empty() -> Self {
        Self {
            items: vec![],
            total: 0,
        }
    }
}

impl Items<Playlist> for Followed {
    fn items(self) -> Vec<Playlist> {
        self.items
    }
}

/*
* ignored fields: images, primary-color, snapshot-id, tracks, uri
*/
#[derive(Debug, Deserialize, Serialize)]
pub struct Playlist {
    pub collaborative: bool,
    pub description: String,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: Option<Owner>,
    pub public: Option<bool>,
    pub tracks: Tracks,
    #[serde(alias = "type")] pub obj_type: String,
}

/*
* ignored fields: href
*/
#[derive(Debug, Deserialize, Serialize)]
pub struct Tracks {
    pub href: String,
    pub total: u32,
}

pub fn debug_print_followed(playlists: &Vec<Playlist>) {
    println!("\n===== PLAYLISTS =====\ntotal: {}", playlists.len());
    println!("{:<50} | {:>10} | {:>11} | {:>12}", "name", "pub.", "coll.", "tracks num.");
    println!("{}", "-".repeat(93));

    for p in playlists {
        if let Some(is_public) = p.public {
            println!("{:<50} | {:>10} | {:>11} | {:>12}", p.name, is_public, p.collaborative, p.tracks.total);
        }
    }
}

pub fn get_followed() -> Vec<Playlist> {
    get_followed_p("./data/followed_playlists.json")
}

pub fn get_followed_p(file_path: &str) -> Vec<Playlist> {
     api_structs::deserialize_from_file::<Followed, Playlist>(file_path)
}
