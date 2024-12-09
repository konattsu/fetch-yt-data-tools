use serde::Serialize;
use std::collections::VecDeque;

use super::video_data::{BasicVideoData, FullVideoData};
use crate::id::PlaylistId;

/// 再生リストの基本的な情報
///
/// 再生リスト内部の動画:`BasicVideoData`を複数保持に加え
/// 再生リスト自体の情報`PlaylistDataItself`を保持
///
/// 保持する動画が0のときもある
#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct BasicPlaylistData {
    pub videos: VecDeque<BasicVideoData>,
    pub playlist_data_itself: PlaylistDataItself,
}

impl BasicPlaylistData {
    pub fn new(videos: VecDeque<BasicVideoData>, id: PlaylistId) -> Self {
        let pl_itself = PlaylistDataItself::new(id, videos.len());
        Self {
            videos,
            playlist_data_itself: pl_itself,
        }
    }
}

/// 再生リストの詳細な情報
///
/// 基本的な`BasicPlaylistData`との違いは内部の動画の情報が
/// `DetailVideoData`であること
///
/// 保持する動画が0のときもある
#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct FullPlaylistData {
    pub videos: VecDeque<FullVideoData>,
    pub playlist_data_itself: PlaylistDataItself,
}

impl FullPlaylistData {
    pub fn new(videos: VecDeque<FullVideoData>, id: PlaylistId) -> Self {
        let pl_itself = PlaylistDataItself::new(id, videos.len());
        Self {
            videos,
            playlist_data_itself: pl_itself,
        }
    }
}

impl From<FullPlaylistData> for BasicPlaylistData {
    fn from(value: FullPlaylistData) -> Self {
        let pl_id = value.playlist_data_itself.id.clone();
        BasicPlaylistData::new(
            value.videos.into_iter().map(Into::into).collect(),
            pl_id,
        )
    }
}

/// 再生リスト自体の情報
#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct PlaylistDataItself {
    /// 再生リスト自体のid
    pub id: PlaylistId,
    /// その再生リストが保持している有効な動画の数, 非公開や削除済みは含まない
    pub total: usize,
}

impl PlaylistDataItself {
    pub fn new(id: PlaylistId, total: usize) -> Self {
        Self { id, total }
    }
}

#[cfg(test)]
impl BasicPlaylistData {
    pub(crate) fn self_1() -> Self {
        let videos = vec![BasicVideoData::self_1()].into();
        let playlist_data_itself = PlaylistDataItself::self_1();
        Self {
            videos,
            playlist_data_itself,
        }
    }

    pub(crate) fn self_2() -> Self {
        let videos = vec![BasicVideoData::self_1(), BasicVideoData::self_2()].into();
        let playlist_data_itself = PlaylistDataItself::self_2();
        Self {
            videos,
            playlist_data_itself,
        }
    }

    pub(crate) fn no_video() -> Self {
        let videos_length_0 = vec![].into();
        let playlist_data_itself = PlaylistDataItself::self_1();
        Self {
            videos: videos_length_0,
            playlist_data_itself,
        }
    }
}

#[cfg(test)]
impl FullPlaylistData {
    pub(crate) fn self_1() -> Self {
        let videos = vec![FullVideoData::self_1()].into();
        let playlist_data_itself = PlaylistDataItself::self_1();
        Self {
            videos,
            playlist_data_itself,
        }
    }

    pub(crate) fn self_2() -> Self {
        let videos = vec![FullVideoData::self_1(), FullVideoData::self_2()].into();
        let playlist_data_itself = PlaylistDataItself::self_2();
        Self {
            videos,
            playlist_data_itself,
        }
    }

    pub(crate) fn no_video() -> Self {
        let videos_length_0 = vec![].into();
        let playlist_data_itself = PlaylistDataItself::self_1();
        Self {
            videos: videos_length_0,
            playlist_data_itself,
        }
    }
}

#[cfg(test)]
impl PlaylistDataItself {
    pub(crate) fn self_1() -> Self {
        Self {
            id: PlaylistId::all_1(),
            total: 1,
        }
    }

    pub(crate) fn self_2() -> Self {
        Self {
            id: PlaylistId::all_2(),
            total: 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_playlist_data_generation_1() {
        let basic_pl_data = BasicPlaylistData::self_1();
        assert_eq!(basic_pl_data.playlist_data_itself.id, PlaylistId::all_1());
        assert_eq!(basic_pl_data.playlist_data_itself.total, 1);
        assert_eq!(basic_pl_data.videos[0], BasicVideoData::self_1());
    }

    #[test]
    fn test_basic_playlist_data_generation_2() {
        let basic_pl_data = BasicPlaylistData::self_2();
        assert_eq!(basic_pl_data.playlist_data_itself.id, PlaylistId::all_2());
        assert_eq!(basic_pl_data.playlist_data_itself.total, 2);
        assert_eq!(basic_pl_data.videos[0], BasicVideoData::self_1());
        assert_eq!(basic_pl_data.videos[1], BasicVideoData::self_2());
    }

    #[test]
    fn test_basic_playlist_data_holds_no_video() {
        let basic_pl_data = BasicPlaylistData::no_video();
        assert!(basic_pl_data.videos.is_empty());
    }

    #[test]
    fn test_full_playlist_data_generation_1() {
        let full_pl_data = FullPlaylistData::self_1();
        assert_eq!(full_pl_data.playlist_data_itself.id, PlaylistId::all_1());
        assert_eq!(full_pl_data.playlist_data_itself.total, 1);
        assert_eq!(full_pl_data.videos[0], FullVideoData::self_1());
    }

    #[test]
    fn test_full_playlist_data_generation_2() {
        let full_pl_data = FullPlaylistData::self_2();
        assert_eq!(full_pl_data.playlist_data_itself.id, PlaylistId::all_2());
        assert_eq!(full_pl_data.playlist_data_itself.total, 2);
        assert_eq!(full_pl_data.videos[0], FullVideoData::self_1());
        assert_eq!(full_pl_data.videos[1], FullVideoData::self_2());
    }

    #[test]
    fn test_full_playlist_data_holds_no_video() {
        let full_pl_data = FullPlaylistData::no_video();
        assert!(full_pl_data.videos.is_empty());
    }

    #[test]
    fn test_playlist_data_conversion_from_full_to_basic() {
        let full = FullPlaylistData::self_1();
        let basic = BasicPlaylistData::self_1();
        assert_eq!(BasicPlaylistData::from(full), basic);
    }
}
