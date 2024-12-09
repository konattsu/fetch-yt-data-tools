use super::super::id::{id_type::IdType, PlaylistId, VideoId};

pub(super) const BASE_URL: &str = "https://www.youtube.com";

pub(super) fn build_url(id: IdType) -> String {
    match id {
        IdType::Video(v) => build_video_url(v),
        IdType::Playlist(pl) => build_playlist_url(pl),
        IdType::VideoPlaylist(v, pl) => build_video_playlist_url(v, pl),
    }
}

fn build_video_url(v_id: VideoId) -> String {
    format!("{}/watch?v={}", BASE_URL, v_id)
}

fn build_playlist_url(pl_id: PlaylistId) -> String {
    format!("{}/playlist?list={}", BASE_URL, pl_id)
}

fn build_video_playlist_url(v_id: VideoId, pl_id: PlaylistId) -> String {
    format!("{}/watch?v={}&list={}", BASE_URL, v_id, pl_id)
}
