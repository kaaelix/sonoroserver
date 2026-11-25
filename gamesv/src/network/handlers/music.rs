use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    FavoriteMusicMotorCycleUpdateRequest, FavoriteMusicMotorCycleUpdateResponse,
    GetMusicInfoRequest, GetMusicInfoResponse, PhonographAlbumTimeInfo,
};

use super::State;

fn collect_unlocked_music_ids(state: &State) -> Vec<i32> {
    let mut ids = Vec::new();
    for music in &state.assets.tables.phonograph_music.items {
        if !ids.contains(&music.Id) {
            ids.push(music.Id);
        }
    }
    ids
}

pub fn handle_get_music_info_request(
    state: &mut State,
    _req: GetMusicInfoRequest,
) -> Result<GetMusicInfoResponse, String> {
    let unlocked_music_ids = collect_unlocked_music_ids(state);

    let mut album_ids = Vec::new();
    let mut album_music_lists: Vec<Vec<i32>> = Vec::new();
    for music in &state.assets.tables.phonograph_music.items {
        for &album_id in &music.Album {
            if let Some(pos) = album_ids.iter().position(|&id| id == album_id) {
                if !album_music_lists[pos].contains(&music.Id) {
                    album_music_lists[pos].push(music.Id);
                }
            } else {
                album_ids.push(album_id);
                album_music_lists.push(vec![music.Id]);
            }
        }
    }

    for album in &state.assets.tables.phonograph_album.items {
        if album.Title == "MotorMusicFavourite" && !album_ids.contains(&album.Id) {
            album_ids.push(album.Id);
            album_music_lists.push(Vec::new());
        }
    }

    let album_time_infos: Vec<PhonographAlbumTimeInfo> = album_ids
        .into_iter()
        .zip(album_music_lists)
        .map(|(album_id, music_ids)| PhonographAlbumTimeInfo {
            AlbumId: album_id,
            BeginTime: 0,
            EndTime: 0,
            MusicIds: music_ids,
        })
        .collect();

    let mut collect_music_ids = Vec::new();
    for &id in &state.player_components.music.info.favorite_music_ids {
        if unlocked_music_ids.contains(&id) && !collect_music_ids.contains(&id) {
            collect_music_ids.push(id);
        }
    }

    Ok(GetMusicInfoResponse {
        CurMusicId: 0,
        ErrorCode: Some(ErrorCode::Success),
        AlbumTimeInfos: album_time_infos,
        CollectMusicIds: collect_music_ids,
    })
}

pub fn handle_favorite_music_motorcycle_update_request(
    state: &mut State,
    req: FavoriteMusicMotorCycleUpdateRequest,
) -> Result<FavoriteMusicMotorCycleUpdateResponse, String> {
    let unlocked_music_ids = collect_unlocked_music_ids(state);
    let mut favorites = Vec::new();
    for id in req.FavoriteMusicList {
        if unlocked_music_ids.contains(&id) && !favorites.contains(&id) {
            favorites.push(id);
        }
    }
    state
        .player_components
        .music
        .info
        .replace_favorite_music_ids(favorites);
    state
        .player_components
        .music
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(FavoriteMusicMotorCycleUpdateResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
