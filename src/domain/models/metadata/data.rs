use serde::Serialize;

use super::{
    playlist_data::{BasicPlaylistData, FullPlaylistData},
    video_data::{BasicVideoData, FullVideoData},
};

/// 動画の基本的なデータ
#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub enum BasicData {
    Video(BasicVideoData),
    Playlist(BasicPlaylistData),
}

impl From<BasicVideoData> for BasicData {
    fn from(value: BasicVideoData) -> Self {
        Self::Video(value)
    }
}

impl From<BasicPlaylistData> for BasicData {
    fn from(value: BasicPlaylistData) -> Self {
        Self::Playlist(value)
    }
}

impl From<FullVideoData> for BasicData {
    fn from(value: FullVideoData) -> Self {
        Self::Video(value.into())
    }
}

impl From<FullPlaylistData> for BasicData {
    fn from(value: FullPlaylistData) -> Self {
        Self::Playlist(value.into())
    }
}

/// 動画の詳細なデータ
#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub enum FullData {
    Video(FullVideoData),
    Playlist(FullPlaylistData),
}

impl From<FullVideoData> for FullData {
    fn from(value: FullVideoData) -> Self {
        Self::Video(value)
    }
}

impl From<FullPlaylistData> for FullData {
    fn from(value: FullPlaylistData) -> Self {
        Self::Playlist(value)
    }
}

impl From<FullData> for BasicData {
    fn from(value: FullData) -> Self {
        match value {
            FullData::Video(v) => v.basic_v_data.into(),
            FullData::Playlist(pl) => {
                let playlist_data_itself = pl.playlist_data_itself;
                // ここのIntoは中身に対して`DetailedVideoData`=>`BasicVideoData`
                Self::Playlist(BasicPlaylistData {
                    videos: pl.videos.into_iter().map(Into::into).collect(),
                    playlist_data_itself,
                })
            }
        }
    }
}

#[cfg(test)]
impl BasicData {
    pub(crate) fn holds_video() -> Self {
        BasicData::Video(BasicVideoData::self_1())
    }

    pub(crate) fn holds_playlist() -> Self {
        BasicData::Playlist(BasicPlaylistData::self_2())
    }
}

#[cfg(test)]
impl FullData {
    pub(crate) fn holds_video() -> Self {
        FullData::Video(FullVideoData::self_1())
    }

    pub(crate) fn holds_playlist() -> Self {
        FullData::Playlist(FullPlaylistData::self_2())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_data_generation_1() {
        let basic_data = BasicData::holds_video();
        let basic_video_data = match basic_data {
            BasicData::Video(v) => v,
            _ => panic!(
                "expected BasicData::Video, but given: {:?}",
                dbg!(basic_data)
            ),
        };
        assert_eq!(basic_video_data, BasicVideoData::self_1());
    }

    #[test]
    fn test_basic_data_generation_2() {
        let basic_data = BasicData::holds_playlist();
        let basic_playlist_data = match basic_data {
            BasicData::Playlist(pl) => pl,
            _ => panic!(
                "expected BasicData::Playlist, but given: {:?}",
                dbg!(basic_data)
            ),
        };
        assert_eq!(basic_playlist_data, BasicPlaylistData::self_2());
    }

    #[test]
    fn test_full_data_generation_1() {
        let full_data = FullData::holds_video();
        let full_video_data = match full_data {
            FullData::Video(v) => v,
            _ => panic!("expected FullData::Video, but given: {:?}", dbg!(full_data)),
        };
        assert_eq!(full_video_data, FullVideoData::self_1())
    }

    #[test]
    fn test_full_data_generation_2() {
        let full_data = FullData::holds_playlist();
        let full_playlist_data = match full_data {
            FullData::Playlist(pl) => pl,
            _ => panic!(
                "expected FullData::Playlist, but given: {:?}",
                dbg!(full_data)
            ),
        };
        assert_eq!(full_playlist_data, FullPlaylistData::self_2());
    }
}
