use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::id::VideoId;

/// 動画の基本的な情報
#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct BasicVideoData {
    pub id: VideoId,
    pub upload_at: DateTime<Utc>,
    pub title: String,
    pub description: String,
    pub channel_id: String,
    pub channel_title: String,
}

impl BasicVideoData {
    pub fn new(
        id: VideoId,
        upload_at: DateTime<Utc>,
        title: String,
        description: String,
        channel_id: String,
        channel_title: String,
    ) -> Self {
        Self {
            id,
            upload_at,
            title,
            description,
            channel_id,
            channel_title,
        }
    }
}

/// 動画の詳細な情報
///
/// 基本的な情報:`BasicVideoData`は全て内部に格納している
#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct FullVideoData {
    pub basic_v_data: BasicVideoData,
    /// 動画の公開状況
    pub live: Live,
}

impl FullVideoData {
    pub fn new(
        id: VideoId,
        upload_at: DateTime<Utc>,
        title: String,
        description: String,
        channel_id: String,
        channel_title: String,
        live: Live,
    ) -> Self {
        let basic_v_data = BasicVideoData::new(
            id,
            upload_at,
            title,
            description,
            channel_id,
            channel_title,
        );
        Self { basic_v_data, live }
    }
}

impl From<FullVideoData> for BasicVideoData {
    fn from(value: FullVideoData) -> Self {
        value.basic_v_data
    }
}

impl<'a> From<&'a FullVideoData> for &'a BasicVideoData {
    fn from(value: &'a FullVideoData) -> Self {
        &value.basic_v_data
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub enum Live {
    Live,
    Upcoming,
    Published,
}

#[cfg(test)]
use chrono::TimeZone;

#[cfg(test)]
impl BasicVideoData {
    pub(crate) fn self_1() -> Self {
        BasicVideoData::new(
            VideoId::all_1(),
            Utc.with_ymd_and_hms(2024, 1, 1, 1, 1, 1).unwrap(),
            "title_1".into(),
            "description_1".into(),
            "channel_id_1".into(),
            "channel_title_1".into(),
        )
    }

    pub(super) fn self_2() -> Self {
        BasicVideoData::new(
            VideoId::all_2(),
            Utc.with_ymd_and_hms(2024, 2, 2, 2, 2, 2).unwrap(),
            "title_2".into(),
            "description_2".into(),
            "channel_id_2".into(),
            "channel_title_2".into(),
        )
    }
}

#[cfg(test)]
impl FullVideoData {
    pub(crate) fn self_1() -> Self {
        let basic_data = BasicVideoData::self_1();
        Self::new(
            basic_data.id,
            basic_data.upload_at,
            basic_data.title,
            basic_data.description,
            basic_data.channel_id,
            basic_data.channel_title,
            Live::Live,
        )
    }

    pub(crate) fn self_2() -> Self {
        let basic_data = BasicVideoData::self_2();
        Self::new(
            basic_data.id,
            basic_data.upload_at,
            basic_data.title,
            basic_data.description,
            basic_data.channel_id,
            basic_data.channel_title,
            Live::Upcoming,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_video_data_generation_1() {
        let b_v_data = BasicVideoData::self_1();
        assert_eq!(b_v_data.id, VideoId::all_1());
        assert_eq!(b_v_data.title, "title_1");
        assert_eq!(b_v_data.description, "description_1");
        assert_eq!(b_v_data.channel_id, "channel_id_1");
        assert_eq!(b_v_data.channel_title, "channel_title_1");
        assert_eq!(
            b_v_data.upload_at,
            Utc.with_ymd_and_hms(2024, 1, 1, 1, 1, 1).unwrap()
        );
    }

    #[test]
    fn test_basic_video_data_generation_2() {
        let b_v_data = BasicVideoData::self_2();
        assert_eq!(b_v_data.id, VideoId::all_2());
        assert_eq!(b_v_data.title, "title_2");
        assert_eq!(b_v_data.description, "description_2");
        assert_eq!(b_v_data.channel_id, "channel_id_2");
        assert_eq!(b_v_data.channel_title, "channel_title_2");
        assert_eq!(
            b_v_data.upload_at,
            Utc.with_ymd_and_hms(2024, 2, 2, 2, 2, 2).unwrap()
        );
    }

    #[test]
    fn test_full_video_data_generation_1() {
        let f_v_data = FullVideoData::self_1();
        assert_eq!(f_v_data.live, Live::Live);
    }

    #[test]
    fn test_full_video_data_generation_2() {
        let f_v_data = FullVideoData::self_2();
        assert_eq!(f_v_data.live, Live::Upcoming);
    }
}
