use crate::api_structs;
use crate::api_structs::{Empty, Items, Image, Deserialize, Serialize, ExternalUrls};
use crate::api_structs::artists::Artist;

use crate::api_structs::SearchDataItem;

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchData {
    pub albums: api_structs::SearchDataItem<Album>,
}

/*
* ignored fields: href, limit, next, offset, previous
*/
#[derive(Debug, Deserialize, Serialize)]
pub struct Saved {
    pub total: u32,
    pub items: Vec<SavedAlbumsItem>,
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

impl Default for SearchDataItem<Album> {
    fn default() -> Self {
        Self {
            previous: None,
            offset: 0,
            total: 0,
            items: vec![],
            next: None,
            href: String::from(""),
            limit: 0,
        }
    }
}

impl Default for SearchData {
    fn default() -> Self {
        Self {
            albums: SearchDataItem::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SavedAlbumsItem {
    pub added_at: String,
    pub album: Album,
}

/*
* ignored fields: 
*/
#[derive(Debug, Deserialize, Serialize)]
pub struct Album {
    pub album_type: String,   // album or single
    pub artists: Vec<Artist>,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub total_tracks: u32,
    pub tracks: Option<Tracks>,
    #[serde(alias = "type")] pub obj_type: String,
}

impl Album {
    pub fn get_track_list(&self) -> Option<&Vec<TracksItem>> {
        let tracks = match &self.tracks {
            Some(t) => t,
            None => return None,
        };

        let track_list = match &tracks.items {
            Some(tl) => tl,
            None => return None,
        };

        Some(track_list)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tracks {
    pub items: Option<Vec<TracksItem>>,
    pub total: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TracksItem {
    pub name: String,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub track_number: u32,
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
