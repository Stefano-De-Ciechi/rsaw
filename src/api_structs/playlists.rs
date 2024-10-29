use crate::api_structs;
use crate::api_structs::{Empty, Items, Deserialize, Serialize, ExternalUrls, Owner};

/*
* ignored fields: href, limit, next, offset, previous
*/
#[allow(dead_code)]
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
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Playlist {
    pub collaborative: bool,
    pub description: String,
    #[serde(alias = "external-urls")] pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    pub owner: Owner,
    pub public: bool,
    pub tracks: Tracks,
    #[serde(alias = "type")] pub obj_type: String,
}

/*
* ignored fields: href
*/
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Tracks {
    pub total: u32,
}

pub fn debug_print_followed(playlists: &Vec<Playlist>) {
    println!("\n===== PLAYLISTS =====\ntotal: {}", playlists.len());
    println!("{:<50} | {:>10} | {:>11} | {:>12}", "name", "pub.", "coll.", "tracks num.");
    println!("{}", "-".repeat(93));

    for p in playlists {
        println!("{:<50} | {:>10} | {:>11} | {:>12}", p.name, p.public, p.collaborative, p.tracks.total);
    }
}

pub fn get_followed() -> Vec<Playlist> {
    get_followed_p("./data/followed_playlists.json")
}

pub fn get_followed_p(file_path: &str) -> Vec<Playlist> {
     api_structs::deserialize_from_file::<Followed, Playlist>(file_path)
}
