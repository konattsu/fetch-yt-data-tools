use super::super::id::{id_type::IdType, PlaylistId, VideoId};
use super::build_url;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct UrlPlaylist(Option<VideoId>, PlaylistId);

impl UrlPlaylist {
    pub fn build_url(&self) -> String {
        let both_id: (Option<VideoId>, PlaylistId) = (self.0.clone(), self.1.clone());
        let id_type: IdType = both_id.into();
        build_url::build_url(id_type)
    }

    pub fn new(v_id: Option<VideoId>, pl_id: PlaylistId) -> Self {
        Self(v_id, pl_id)
    }

    pub fn get_playlist_id(&self) -> &PlaylistId {
        &self.1
    }

    pub fn get_video_id(&self) -> &Option<VideoId> {
        &self.0
    }
}

impl From<(Option<VideoId>, PlaylistId)> for UrlPlaylist {
    fn from(value: (Option<VideoId>, PlaylistId)) -> Self {
        Self(value.0, value.1)
    }
}
