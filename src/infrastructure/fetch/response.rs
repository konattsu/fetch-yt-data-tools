use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::VecDeque, ops::Deref};

use crate::{id::VideoId, metadata::Live};

/// depth: **0**
///
/// jsonの深さを分かりやすくするためにidx(0, 1...)を付与
///
/// https://developers.google.com/youtube/v3/docs/videos/list
#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
pub(super) struct ApiResponse {
    pub kind: String,
    /// 大量に曲がある再生リストなどは分割してリクエストを送信
    /// そのときにクエリに含める
    pub nextPageToken: Option<PageToken>,
    /// 基本は`nextPageToken`を使用するが、前の値を取得したいときはこれ
    pub prevPageToken: Option<PageToken>,
    /// 個々の動画の情報
    pub items: VecDeque<Item>,
}

impl ApiResponse {
    #[allow(unused)]
    pub(super) fn is_for_video(&self) -> bool {
        self.items.iter().all(|item| matches!(item, Item::Video { .. }))
    }

    pub(super) fn get_as_video(self) -> Option<VecDeque<ItemVideo>> {
        let mut res: VecDeque<ItemVideo> = VecDeque::new();
        for item in self.items {
            if let Item::Video(item_video) = item {
                res.push_back(item_video);
            } else {
                return None;
            }
        }
        Some(res)
    }

    #[allow(unused)]
    pub(super) fn is_for_playlist(&self) -> bool {
        self.items.iter().all(|item| matches!(item, Item::Playlist { .. }))
    }

    pub(super) fn get_as_playlist(self) -> Option<VecDeque<ItemPlaylist>> {
        let mut res: VecDeque<ItemPlaylist> = VecDeque::new();
        for item in self.items {
            if let Item::Playlist(item_pl) = item {
                res.push_back(item_pl);
            } else {
                return None;
            }
        }
        Some(res)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub(super) struct PageToken(String);

impl Deref for PageToken {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
impl PageToken {
    pub(crate) fn new_for_test(s: String) -> Self {
        Self(s)
    }
}

/// depth: **1**
///
/// https://developers.google.com/youtube/v3/docs/videos
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "kind")]
pub(super) enum Item {
    #[serde(rename = "youtube#video")]
    Video(ItemVideo),
    #[serde(rename = "youtube#playlistItem")]
    Playlist(ItemPlaylist),
}

/// depth: **1**
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub(super) struct ItemVideo {
    pub id: VideoId,
    pub snippet: SnippetVideo,
}

/// depth: **1**
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub(super) enum ItemPlaylist {
    Unavailable,
    Available(SnippetPlaylist),
}

// カスタムデシリアライズを適用している理由(ItemPlaylistとして分けた理由)
//
// 再生リスト内の動画で非公開や削除されているとき
// - 動画作成者チャンネル名`videoOwnerChannelTitle`
// - 動画作成者のチャンネルid`videoOwnerChannelId`
// この2つがレスポンスに含まれない eq デシリアライズできない eq カスタムデシリアライズ必要
//
// 上記のことをレスポンスから判断するには`channelTitle`,`description`に注目する
// 非公開,削除に応じて同じ文字列が使用されるため(具体的な値は定数参照)

impl<'de> Deserialize<'de> for ItemPlaylist {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// serde_valueからキーを基に値を取得,無いと`missing_field`
        fn get_value_as_str_by_key<'a, 'de, D: serde::Deserializer<'de>>(
            value: &'a Value,
            key: &'static str,
        ) -> Result<&'a str, D::Error> {
            value
                .get(key)
                .and_then(Value::as_str)
                .ok_or_else(|| serde::de::Error::missing_field(key))
        }

        let value: Value = Value::deserialize(deserializer)?;
        // `description`と`title`は`snippet`配下にあるので`snippet`から取得
        let snippet = value
            .get("snippet")
            .ok_or_else(|| serde::de::Error::missing_field("snippet"))?;
        let desc = get_value_as_str_by_key::<D>(snippet, "description")?;
        let title = get_value_as_str_by_key::<D>(snippet, "title")?;

        // 非公開動画など利用できない動画が含まれているとき
        if is_unavailable_video(title, desc) {
            Ok(Self::Unavailable)
        } else {
            // ここで`snippet`を基にdeserialize <== めっちゃ大事
            let snippet_playlist: SnippetPlaylist =
                serde_json::from_value(snippet.clone())
                    .map_err(serde::de::Error::custom)?;
            Ok(Self::Available(snippet_playlist))
        }
    }
}

impl ItemPlaylist {
    pub(super) fn available(self) -> Option<SnippetPlaylist> {
        match self {
            Self::Unavailable => None,
            Self::Available(snippet) => Some(snippet),
        }
    }
}

/// depth: **2**
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[allow(non_snake_case)]
pub(super) struct CommonSnippet {
    /// 公開時刻; 配信, プレミア公開は枠立ての時刻
    pub publishedAt: DateTime<Utc>,
    pub title: String,
    pub description: String,
}

/// depth: **2**
///
///  Only `Video`
///
/// https://developers.google.com/youtube/v3/docs/videos?hl=ja#snippet
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[allow(non_snake_case)]
pub(super) struct SnippetVideo {
    #[serde(flatten)]
    pub common_snippet: CommonSnippet,
    /// 動画の作成者のid
    // TODO このidを`domain`で型定義してもいい
    pub channelId: String,
    /// 動画の作成者
    pub channelTitle: String,
    pub liveBroadcastContent: LiveBroadcast,
}

/// depth: **3**
///
/// Only `Video`
///
/// https://developers.google.com/youtube/v3/docs/videos?hl=ja#snippet.liveBroadcastContent
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub(super) enum LiveBroadcast {
    /// プレミア公開中, 配信中
    live,
    /// 枠のみ
    upcoming,
    /// 公開された後
    none,
}

impl From<LiveBroadcast> for Live {
    fn from(value: LiveBroadcast) -> Self {
        match value {
            LiveBroadcast::live => Self::Live,
            LiveBroadcast::upcoming => Self::Upcoming,
            LiveBroadcast::none => Self::Published,
        }
    }
}

/// depth: **2**
///
/// Only `Playlist`
///
/// https://developers.google.com/youtube/v3/docs/playlists?hl=ja#snippet
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[allow(non_snake_case)]
pub(super) struct SnippetPlaylist {
    #[serde(flatten)]
    pub common_snippet: CommonSnippet,
    pub resourceId: ResourceId,
    /// 動画の作成者
    pub videoOwnerChannelTitle: String,
    /// 動画の作成者のid
    pub videoOwnerChannelId: String,
}

/// depth: **3**
///
/// Only `Playlist`
///
/// (ドキュメント化されてない)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[allow(non_snake_case)]
pub(super) struct ResourceId {
    pub videoId: VideoId,
}

#[cfg(test)]
impl ApiResponse {
    pub(super) fn v_dummy() -> Self {
        let a = r##"{
            "kind": "youtube#videoListResponse",
            "items": [
                {
                    "kind": "youtube#video",
                    "etag": "0DqY1M-Z0u2b8DkoL0j4fDYL1wM",
                    "id": "00000000000",
                    "snippet": {
                        "publishedAt": "2024-06-25T18:00:00Z",
                        "title": "foo_title_0",
                        "description": "foo_description_0",
                        "channelId": "UC7_00000000000000000000",
                        "channelTitle": "foo_channel_title_made_this_video_0",
                        "liveBroadcastContent": "none"
                    }
                },
                {
                    "kind": "youtube#video",
                    "id": "11111111111",
                    "snippet": {
                        "publishedAt": "2024-06-25T18:00:00Z",
                        "title": "foo_title_1",
                        "description": "foo_description_1",
                        "channelId": "UC7_11111111111111111111",
                        "channelTitle": "foo_channel_title_made_this_video_1",
                        "liveBroadcastContent": "live"
                    }
                },
                {
                    "kind": "youtube#video",
                    "id": "22222222222",
                    "snippet": {
                        "publishedAt": "2024-06-25T18:00:00Z",
                        "title": "foo_title_2",
                        "description": "foo_description_2",
                        "channelId": "UC7_22222222222222222222",
                        "channelTitle": "foo_channel_title_made_this_video_2",
                        "liveBroadcastContent": "upcoming"
                    }
                }
            ]
        }"##;
        serde_json::from_str(a).unwrap()
    }

    pub(super) fn pl_dummy() -> Self {
        let a = r##"{
            "kind": "youtube#playlistItemListResponse",
            "nextPageToken": "next_page_token",
            "items": [
                {
                    "kind": "youtube#playlistItem",
                    "snippet": {
                        "publishedAt": "2024-06-25T18:00:00Z",
                        "title": "foo_title_0",
                        "description": "foo_description_0",
                        "videoOwnerChannelTitle": "foo_channel_title_made_this_video_0",
                        "videoOwnerChannelId": "UC7_00000000000000000000",
                        "resourceId": {
                            "videoId": "00000000000"
                        }
                    }
                },
                {
                    "kind": "youtube#playlistItem",
                    "snippet": {
                        "publishedAt": "2024-06-25T18:00:00Z",
                        "title": "foo_title_1",
                        "description": "foo_description_1",
                        "videoOwnerChannelTitle": "foo_channel_title_made_this_video_1",
                        "videoOwnerChannelId": "UC7_11111111111111111111",
                        "resourceId": {
                            "videoId": "11111111111"
                        }
                    }
                }
            ]
        }"##;
        serde_json::from_str(a).unwrap()
    }
}

pub(super) const UNAVAILABLE_VIDEOS_DESCRIPTION: [&str; 2] =
    ["This video is private.", "This video is unavailable."];

pub(super) const UNAVAILABLE_VIDEOS_TITLE: [&str; 2] =
    ["Private video", "Deleted video"];

pub(super) fn is_unavailable_video(title: &str, desc: &str) -> bool {
    UNAVAILABLE_VIDEOS_TITLE.iter().any(|a| *a == title)
        && UNAVAILABLE_VIDEOS_DESCRIPTION.iter().any(|a| *a == desc)
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn test_deserialize_video() {
        let _video = ApiResponse::v_dummy();
    }

    #[test]
    fn test_deserialize_playlist() {
        let _pl = ApiResponse::pl_dummy();
    }

    #[test]
    fn test_deserialize_snippet_playlist() {
        let content = r##"
            {
                "publishedAt": "2024-01-10T15:00:00Z",
                "title": "title_valid_video",
                "description": "description_valid_video",
                "videoOwnerChannelTitle": "foo_channel_title_made_this_video_1",
                "videoOwnerChannelId": "UC7_11111111111111111111",
                "resourceId": {
                    "videoId": "11111111111"
                }
            }
        "##;
        let _a: SnippetPlaylist = serde_json::from_str(content).unwrap();
    }

    #[test]
    fn test_deserialize_playlist_contains_unavailable_video() {
        let resp = r##"
        {
            "kind": "youtube#playlistItemListResponse",
            "items": [
                {
                    "kind": "youtube#playlistItem",
                    "snippet": {
                        "publishedAt": "2024-01-10T14:00:00Z",
                        "title": "Private video",
                        "description": "This video is private.",
                        "resourceId": {
                            "videoId": "00000000000"
                        }
                    }
                },
                {
                    "kind": "youtube#playlistItem",
                    "snippet": {
                        "publishedAt": "2024-01-10T15:00:00Z",
                        "title": "title_valid_video",
                        "description": "description_valid_video",
                        "videoOwnerChannelTitle": "foo_channel_title_made_this_video_1",
                        "videoOwnerChannelId": "UC7_11111111111111111111",
                        "resourceId": {
                            "videoId": "11111111111"
                        }
                    }
                }
            ]
        }"##;

        let api_response: ApiResponse = serde_json::from_str(resp).unwrap();

        assert!(!api_response.is_for_video());
        assert!(api_response.is_for_playlist());

        let mut item_pl = api_response.get_as_playlist().unwrap().into_iter();

        assert_eq!(item_pl.next().unwrap(), ItemPlaylist::Unavailable);
        assert_eq!(
            item_pl.next().unwrap(),
            ItemPlaylist::Available(SnippetPlaylist {
                common_snippet: CommonSnippet {
                    publishedAt: Utc.with_ymd_and_hms(2024, 1, 10, 15, 0, 0).unwrap(),
                    title: "title_valid_video".into(),
                    description: "description_valid_video".into()
                },
                resourceId: ResourceId {
                    videoId: VideoId::all_1()
                },
                videoOwnerChannelTitle: "foo_channel_title_made_this_video_1".into(),
                videoOwnerChannelId: "UC7_11111111111111111111".into()
            })
        );
    }
}
