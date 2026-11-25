use serde::Deserialize;
use sonoro_common::fs::FileSystem;

#[derive(Debug, Deserialize, Default)]
#[allow(non_snake_case)]
struct AutoPilotCircle {
    #[serde(default)]
    MapId: i32,
    #[serde(default)]
    WaySplines: Vec<i32>,
}

pub fn road_ids_for_map(fs: &FileSystem, map_id: i32) -> Result<Vec<i32>, String> {
    let bytes = fs
        .read_file("assets/BinData/AutoPilotCircles.json")
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "AutoPilotCircles.json not found".to_string())?;
    let circles: Vec<AutoPilotCircle> =
        serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;

    let mut roads = Vec::new();
    for circle in circles {
        if circle.MapId != map_id {
            continue;
        }
        for road_id in circle.WaySplines {
            if !roads.contains(&road_id) {
                roads.push(road_id);
            }
        }
    }
    Ok(roads)
}
