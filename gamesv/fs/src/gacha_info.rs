use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Banner {
    pub id: i32,
    pub selected_pool_id: i32,
    pub daily_pulls: i32,
    pub total_pulls: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Bucket {
    pub id: i32,
    pub pull_count: i32,
    pub pity_four: i32,
    pub guarantee_five: bool,
    pub guarantee_four: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct GachaInfo {
    pub banners: Vec<Banner>,
    pub buckets: Vec<Bucket>,
}

impl GachaInfo {
    pub const DATA_PATH: &'static str = "gacha_info";

    pub fn get_banner(&mut self, gacha_id: i32) -> Option<&mut Banner> {
        self.banners.iter_mut().find(|b| b.id == gacha_id)
    }

    pub fn ensure_banner(&mut self, gacha_id: i32, selected_pool_id: i32) -> &mut Banner {
        if let Some(pos) = self.banners.iter().position(|b| b.id == gacha_id) {
            if self.banners[pos].selected_pool_id == 0 {
                self.banners[pos].selected_pool_id = selected_pool_id;
            }
            return &mut self.banners[pos];
        }
        self.banners.push(Banner {
            id: gacha_id,
            selected_pool_id,
            daily_pulls: 0,
            total_pulls: 0,
        });
        self.banners.last_mut().expect("just pushed")
    }

    pub fn get_bucket(&mut self, bucket_id: i32) -> Option<&mut Bucket> {
        self.buckets.iter_mut().find(|b| b.id == bucket_id)
    }

    pub fn ensure_bucket(&mut self, bucket_id: i32) -> &mut Bucket {
        if let Some(pos) = self.buckets.iter().position(|b| b.id == bucket_id) {
            return &mut self.buckets[pos];
        }
        self.buckets.push(Bucket {
            id: bucket_id,
            ..Default::default()
        });
        self.buckets.last_mut().expect("just pushed")
    }
}
