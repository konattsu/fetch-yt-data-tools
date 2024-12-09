use super::{PlaylistId, VideoId};

/// domain層の内部でデータを相互変換しやすくするための型
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(in super::super) enum IdType {
    Video(VideoId),
    Playlist(PlaylistId),
    VideoPlaylist(VideoId, PlaylistId),
}

impl From<VideoId> for IdType {
    fn from(value: VideoId) -> Self {
        IdType::Video(value)
    }
}

impl From<PlaylistId> for IdType {
    fn from(value: PlaylistId) -> Self {
        IdType::Playlist(value)
    }
}

impl From<(VideoId, PlaylistId)> for IdType {
    fn from(value: (VideoId, PlaylistId)) -> Self {
        IdType::VideoPlaylist(value.0, value.1)
    }
}

impl From<(VideoId, Option<PlaylistId>)> for IdType {
    fn from(value: (VideoId, Option<PlaylistId>)) -> Self {
        match value.1 {
            Some(pl) => (value.0, pl).into(),
            None => value.0.into(),
        }
    }
}

impl From<(Option<VideoId>, PlaylistId)> for IdType {
    fn from(value: (Option<VideoId>, PlaylistId)) -> Self {
        match value.0 {
            Some(v) => (v, value.1).into(),
            None => value.1.into(),
        }
    }
}

impl TryFrom<(Option<VideoId>, Option<PlaylistId>)> for IdType {
    type Error = crate::Error;
    fn try_from(
        value: (Option<VideoId>, Option<PlaylistId>),
    ) -> Result<Self, Self::Error> {
        match value {
            (Some(v), None) => Ok(Self::Video(v)),
            (None, Some(pl)) => Ok(Self::Playlist(pl)),
            (Some(v), Some(pl)) => Ok(Self::VideoPlaylist(v, pl)),
            (None, None) => Err(crate::Error::InvalidInput),
        }
    }
}

impl IdType {
    pub fn new(
        v_id: Option<VideoId>,
        pl_id: Option<PlaylistId>,
    ) -> Result<Self, crate::Error> {
        (v_id, pl_id).try_into()
    }
}
