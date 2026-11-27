use sonoro_gamesv_logic::helpers::{energy, pb_kv};
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

    let now_s = energy::now_seconds();
    let end_s = now_s + 31_536_000;
    let album_time_infos: Vec<PhonographAlbumTimeInfo> = album_ids
        .into_iter()
        .zip(album_music_lists)
        .map(|(album_id, music_ids)| {
            let is_timed = state
                .assets
                .tables
                .phonograph_album
                .get_data_by_id(&album_id)
                .is_some_and(|a| a.TimeQuantum > 0);
            PhonographAlbumTimeInfo {
                AlbumId: album_id,
                BeginTime: if is_timed { 1 } else { 0 },
                EndTime: if is_timed { end_s } else { 0 },
                MusicIds: music_ids,
            }
        })
        .collect();

    let mut collect_music_ids = Vec::new();
    for &id in &state.player_components.music.info.favorite_music_ids {
        if unlocked_music_ids.contains(&id) && !collect_music_ids.contains(&id) {
            collect_music_ids.push(id);
        }
    }
    let stored_cur = state
        .player_components
        .progress
        .info
        .get_key_int("music:cur_id") as i32;
    let cur_music_id = if stored_cur > 0 && unlocked_music_ids.contains(&stored_cur) {
        stored_cur
    } else {
        0
    };

    Ok(GetMusicInfoResponse {
        CurMusicId: cur_music_id,
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
    if let Some(&first_fav) = state.player_components.music.info.favorite_music_ids.first() {
        let _ = pb_kv::persist_int(
            &state.fs,
            &mut state.player_components.progress,
            "music:cur_id",
            i64::from(first_fav),
        );
    }
    Ok(FavoriteMusicMotorCycleUpdateResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
