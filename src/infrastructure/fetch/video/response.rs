use chrono::{DateTime, Utc};
use std::collections::VecDeque;

use crate::{
    id::VideoId,
    metadata::{FullVideoData, Live},
};

use super::super::response::{ApiResponse, SnippetVideo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct VideoApiResponse(VecDeque<Item>);

impl VideoApiResponse {
    pub fn new(data_value: ApiResponse) -> Result<Self, String> {
        match data_value.get_as_video() {
            Some(items) => Ok(Self(
                items
                    .iter()
                    .map(|item| Item::new_from_id_and_snippet(&item.id, &item.snippet))
                    .collect(),
            )),
            None => Err(
                format!("{}{}",
                "Contains information other tha video inside. ",
                "External api implementations has changed or there is an implementations error."
            )),
        }
    }

    pub fn get_item_by_id(&self, id: &VideoId) -> Option<FullVideoData> {
        let found_item: &Option<FullVideoData> =
            &self.0.iter().find(|item| item.id == *id).cloned().map(Into::into);
        found_item.clone()
    }
}

impl TryFrom<ApiResponse> for VideoApiResponse {
    type Error = String;
    fn try_from(value: ApiResponse) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<VideoApiResponse> for VecDeque<FullVideoData> {
    fn from(value: VideoApiResponse) -> Self {
        value.0.into_iter().map(Into::into).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Item {
    id: VideoId,
    published_at: DateTime<Utc>,
    title: String,
    description: String,
    channel_id: String,
    channel_title: String,
    live: Live,
}

impl Item {
    fn new_from_id_and_snippet(id: &VideoId, snippet: &SnippetVideo) -> Self {
        let common = &snippet.common_snippet;
        Self {
            id: id.clone(),
            published_at: common.publishedAt,
            title: common.title.clone(),
            description: common.description.clone(),
            channel_id: snippet.channelId.clone(),
            channel_title: snippet.channelTitle.clone(),
            live: snippet.liveBroadcastContent.into(),
        }
    }
}

impl From<Item> for FullVideoData {
    fn from(value: Item) -> Self {
        Self::new(
            value.id,
            value.published_at,
            value.title,
            value.description,
            value.channel_id,
            value.channel_title,
            value.live,
        )
    }
}

impl From<&Item> for FullVideoData {
    fn from(value: &Item) -> Self {
        Self::new(
            value.id.clone(),
            value.published_at,
            value.title.clone(),
            value.description.clone(),
            value.channel_id.clone(),
            value.channel_title.clone(),
            value.live,
        )
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use crate::metadata::BasicVideoData;

    use super::*;

    fn video_api_response() -> VideoApiResponse {
        let data_value = ApiResponse::v_dummy();
        VideoApiResponse::new(data_value).unwrap()
    }

    #[test]
    fn test_video_api_response_from() {
        // 作成できるか
        let _video_api_response = video_api_response();
    }

    #[test]
    fn test_video_api_response_get_item_by_id() {
        let id_to_be_found = VideoId::all_1();
        let full_video_data_to_be_found = FullVideoData {
            basic_v_data: BasicVideoData {
                id: VideoId::all_1(),
                upload_at: Utc.with_ymd_and_hms(2024, 6, 25, 18, 0, 0).unwrap(),
                title: "foo_title_1".into(),
                description: "foo_description_1".into(),
                channel_id: "UC7_11111111111111111111".into(),
                channel_title: "foo_channel_title_made_this_video_1".into(),
            },
            live: Live::Live,
        };

        assert_eq!(
            video_api_response().get_item_by_id(&id_to_be_found).unwrap(),
            full_video_data_to_be_found,
        );
    }

    #[test]
    fn test_video_data_value() {
        let video_api_response = video_api_response();
        let published_at = Utc.with_ymd_and_hms(2024, 6, 25, 18, 0, 0).unwrap();

        // 内部値の比較
        assert_eq!(
            video_api_response,
            VideoApiResponse(
                vec![
                    Item {
                        id: VideoId::all_0(),
                        published_at,
                        title: "foo_title_0".into(),
                        description: "foo_description_0".into(),
                        channel_id: "UC7_00000000000000000000".into(),
                        channel_title: "foo_channel_title_made_this_video_0".into(),
                        live: Live::Published,
                    },
                    Item {
                        id: VideoId::all_1(),
                        published_at,
                        title: "foo_title_1".into(),
                        description: "foo_description_1".into(),
                        channel_id: "UC7_11111111111111111111".into(),
                        channel_title: "foo_channel_title_made_this_video_1".into(),
                        live: Live::Live,
                    },
                    Item {
                        id: VideoId::all_2(),
                        published_at,
                        title: "foo_title_2".into(),
                        description: "foo_description_2".into(),
                        channel_id: "UC7_22222222222222222222".into(),
                        channel_title: "foo_channel_title_made_this_video_2".into(),
                        live: Live::Upcoming
                    }
                ]
                .into()
            )
        )
    }
}
