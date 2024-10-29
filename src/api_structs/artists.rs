use crate::api_structs;
use crate::api_structs::{Empty, Items, Serialize, Deserialize, ExternalUrls};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Followed{
    pub artists: FollowedArtistsItems,
}

impl Empty<Self> for Followed {
    fn empty() -> Self {
        Self {
            artists: FollowedArtistsItems { items: vec![], total: 0 }
        } 
    }
}

impl Items<Artist> for Followed {
    fn items(self) -> Vec<Artist> {
        self.artists.items
    }
}

/*
* ignored fields: next, cursors, limit, href
*/
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct FollowedArtistsItems {
    pub items: Vec<Artist>,
    pub total: u32,
}

/*
* ignored fields: images, popularity, uri
*/
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    pub external_urls: ExternalUrls, 
    pub genres: Option<Vec<String>>,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(alias = "type")] pub obj_type: String, 
}

pub fn debug_print_followed(artists: &Vec<Artist>) {
    println!("\n===== ARTISTS =====\ntotal: {}", artists.len());
    println!("{:<50} | {:<150}", "name", "genres");
    println!("{}", "-".repeat(200));

    for a in artists {
        let genres: &Vec<String> = if let Some(g) = &a.genres { g } else { &vec![] };
        println!("{:<50} | {:<150?}", a.name, genres);
    }
}

pub fn get_followed() -> Vec<Artist> {
    get_followed_p("./data/followed_artists.json")
}

pub fn get_followed_p(file_path: &str) -> Vec<Artist> {
    api_structs::deserialize_from_file::<Followed, Artist>(file_path)
}

