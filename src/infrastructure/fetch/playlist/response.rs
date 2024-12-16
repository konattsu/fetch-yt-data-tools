use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use super::super::response::{ApiResponse, PageToken, SnippetPlaylist};
use crate::metadata::{BasicPlaylistData, BasicVideoData, PlaylistDataItself};
use crate::{
    id::{PlaylistId, VideoId},
    url::UrlPlaylist,
};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub(super) struct PlaylistApiResponse(VecDeque<Item>, PlaylistData);

impl PlaylistApiResponse {
    pub fn new(
        data_value: ApiResponse,
        url_playlist: &UrlPlaylist,
    ) -> Result<Self, String> {
        let next_page_token = data_value.nextPageToken.clone();

        // generate self(0)
        let items: VecDeque<Item> = match data_value.get_as_playlist() {
            Some(items) => items
                .into_iter()
                // 非公開動画を消す
                .filter_map(|item| item.available())
                .map(Into::into)
                .collect(),
            None => return Err("".into()),
        };

        // generate self(1)
        let total = items.len();
        let pl_data = PlaylistData::new(
            url_playlist.get_playlist_id().clone(),
            total,
            url_playlist.get_video_id().clone(),
            next_page_token,
        );
        Ok(Self(items, pl_data))
    }

    pub fn get_next_page_token(&self) -> Option<PageToken> {
        self.1.next_page_token.clone()
    }

    /// 別(next_page_tokenを使用して)でfetchしたものと結合
    ///
    /// `old.merge(new)`のように呼び出す
    ///
    /// `Err`: 異なる再生リストを渡した(i.e. `playlist_id`が異なる)とき
    pub fn merge(&mut self, new: Self) -> Result<(), String> {
        if self.1.id != new.1.id {
            return Err("cannot merge different playlist".into());
        }
        self.1.total += new.1.total;
        self.1.next_page_token = new.1.next_page_token;
        self.0.extend(new.0);
        Ok(())
    }
}

// 従来のようにFrom<これ> for 最終的な奴はできないのでvideoのapiを呼ぶ必要がある。
// このとき、どのように値を返すか要件等。
// また、二つ具体作ってもいい (eq: 処理時間を短くする時用)

impl From<PlaylistApiResponse> for VecDeque<VideoId> {
    fn from(value: PlaylistApiResponse) -> Self {
        value.0.into_iter().map(|item| item.id).collect()
    }
}

impl From<&PlaylistApiResponse> for PlaylistDataItself {
    fn from(value: &PlaylistApiResponse) -> Self {
        Self::new(value.1.id.clone(), value.1.total)
    }
}

impl From<PlaylistApiResponse> for BasicPlaylistData {
    fn from(value: PlaylistApiResponse) -> Self {
        let pl_id = value.1.id.clone();
        BasicPlaylistData::new(value.0.into_iter().map(Into::into).collect(), pl_id)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
struct Item {
    id: VideoId,
    published_at: DateTime<Utc>,
    title: String,
    description: String,
    channel_id: String,
    channel_title: String,
}

impl From<SnippetPlaylist> for Item {
    fn from(value: SnippetPlaylist) -> Self {
        let common = value.common_snippet;
        Self {
            id: value.resourceId.videoId,
            published_at: common.publishedAt,
            title: common.title,
            description: common.description,
            channel_id: value.videoOwnerChannelId,
            channel_title: value.videoOwnerChannelTitle,
        }
    }
}

impl From<Item> for BasicVideoData {
    fn from(value: Item) -> Self {
        BasicVideoData {
            id: value.id,
            upload_at: value.published_at,
            title: value.title,
            description: value.description,
            channel_id: value.channel_id,
            channel_title: value.channel_title,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
struct PlaylistData {
    id: PlaylistId,
    total: usize,
    specified_directly: Option<VideoId>,
    next_page_token: Option<PageToken>,
}

impl PlaylistData {
    fn new(
        id: PlaylistId,
        total: usize,
        specified_directly: Option<VideoId>,
        next_page_token: Option<PageToken>,
    ) -> Self {
        Self {
            id,
            total,
            specified_directly,
            next_page_token,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn test_playlist_data_value() {
        let data_value = ApiResponse::pl_dummy();
        let pl_id = PlaylistId::inc_from_1();
        let url_pl = UrlPlaylist::new(Some(VideoId::all_1()), pl_id.clone());
        // 正常にパースできるか

        let playlist_data_value =
            PlaylistApiResponse::new(data_value, &url_pl).unwrap();

        // 内部値の比較
        let published_at = Utc.with_ymd_and_hms(2024, 6, 25, 18, 0, 0).unwrap();
        assert_eq!(
            playlist_data_value,
            PlaylistApiResponse(
                vec![
                    Item {
                        id: VideoId::all_0(),
                        published_at,
                        title: "foo_title_0".into(),
                        description: "foo_description_0".into(),
                        channel_id: "UC7_00000000000000000000".into(),
                        channel_title: "foo_channel_title_made_this_video_0".into()
                    },
                    Item {
                        id: VideoId::all_1(),
                        published_at,
                        title: "foo_title_1".into(),
                        description: "foo_description_1".into(),
                        channel_id: "UC7_11111111111111111111".into(),
                        channel_title: "foo_channel_title_made_this_video_1".into()
                    }
                ]
                .into(),
                PlaylistData {
                    id: pl_id,
                    total: 2,
                    specified_directly: Some(VideoId::all_1()),
                    next_page_token: Some(PageToken::new_for_test(
                        "next_page_token".into()
                    ))
                }
            )
        )
    }
}
