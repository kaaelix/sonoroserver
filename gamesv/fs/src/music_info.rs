use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct MusicInfo {
    pub favorite_music_ids: Vec<i32>,
}

impl MusicInfo {
    pub const DATA_PATH: &'static str = "music_info";

    pub fn replace_favorite_music_ids(&mut self, ids: Vec<i32>) {
        self.favorite_music_ids = ids;
    }
}
