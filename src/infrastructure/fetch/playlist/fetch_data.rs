use futures::{stream::FuturesOrdered, StreamExt};
use serde::Deserialize;
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tracing::Level;

use super::super::{
    api_request::ApiRequest, collections::quota, response::ApiResponse,
    response::PageToken, ApiKey, ApiOptionKey, FetchError, MaxIdNum,
};
use super::response::PlaylistApiResponse;
use crate::{metadata::BasicPlaylistData, url::UrlPlaylist};

#[derive(Debug)]
pub struct PlaylistApiClient {
    max_id: MaxIdNum,
    api_options: ApiOptionKey,
    api_key: ApiKey,
    used_quota: Arc<AtomicUsize>,
}

impl ApiRequest for PlaylistApiClient {}

impl PlaylistApiClient {
    pub fn new_default(api_key: ApiKey, used_quota: Arc<AtomicUsize>) -> Self {
        Self {
            max_id: MaxIdNum::default(),
            api_options: ApiOptionKey::default(),
            api_key,
            used_quota,
        }
    }

    // pub async fn fetch_video_ids(
    //     &self,
    //     urls_pl: VecDeque<UrlPlaylist>,
    // ) -> Result<VecDeque<Result<VideosInPlaylist, UrlPlaylist>>, FetchError> {
    //     let mut tasks = FuturesOrdered::new();
    //     for url_pl in urls_pl {
    //         tasks.push_back(self.process_single_pl(url_pl));
    //     }
    //     let mut fetched_data: VecDeque<Result<VideosInPlaylist, UrlPlaylist>> =
    //         VecDeque::new();
    //     while let Some(res) = tasks.next().await {
    //         fetched_data.push_back(res?.map(Into::into));
    //     }
    //     Ok(fetched_data)
    // }

    pub async fn fetch_all_playlist_data(
        &self,
        urls_pl: VecDeque<UrlPlaylist>,
    ) -> Result<VecDeque<Result<BasicPlaylistData, UrlPlaylist>>, crate::Error> {
        let mut tasks = FuturesOrdered::new();
        for url_pl in urls_pl {
            tasks.push_back(self.process_playlist(url_pl));
        }
        let mut fetched_data: VecDeque<Result<BasicPlaylistData, UrlPlaylist>> =
            VecDeque::new();
        while let Some(res) = tasks.next().await {
            fetched_data.push_back(res?.map(Into::into));
        }
        Ok(fetched_data)
    }

    pub async fn fetch_playlist_data(
        &self,
        url_pl: UrlPlaylist,
    ) -> Result<Result<BasicPlaylistData, UrlPlaylist>, crate::Error> {
        Ok(self.process_playlist(url_pl).await?.map(Into::into))
    }

    #[tracing::instrument(level = Level::DEBUG, ret)]
    async fn process_playlist(
        &self,
        url_pl: UrlPlaylist,
    ) -> Result<Result<PlaylistApiResponse, UrlPlaylist>, FetchError> {
        let mut fetched_pl_data: Option<PlaylistApiResponse> = None;
        let mut url_string = self.build_playlist_api_url(&url_pl);
        loop {
            let pl_resp = match self.call_playlist_api(&url_pl, &url_string).await? {
                Some(pl_resp) => pl_resp,
                None => return Ok(Err(url_pl)),
            };
            let next_page_token = pl_resp.get_next_page_token();
            Self::merge_playlist_data(&mut fetched_pl_data, pl_resp)?;
            url_string = match self.build_next_page_url(&url_pl, next_page_token) {
                Some(url_string) => url_string,
                None => break,
            }
        }
        Ok(Ok(fetched_pl_data.unwrap()))
    }

    fn build_playlist_api_url(&self, pl_url: &UrlPlaylist) -> String {
        format!(
            "{}playlistItems?key={}&part={}&playlistId={}&maxResults={}",
            Self::BASE_API_URL,
            self.api_key.as_string(),
            self.api_options.join(","),
            pl_url.get_playlist_id(),
            self.max_id
        )
    }

    #[tracing::instrument(level = Level::TRACE, ret, skip(url_string))]
    async fn call_playlist_api(
        &self,
        url_pl: &UrlPlaylist,
        url_string: &str,
    ) -> Result<Option<PlaylistApiResponse>, FetchError> {
        self.used_quota.fetch_add(quota::VIDEO_INFO, Ordering::Relaxed);

        // for debug
        let response = self.api_call(url_string).await?;
        let deserialized = ApiResponse::deserialize(response);
        // for debug end

        match deserialized {
            Ok(resp) => Ok(Some(
                // data_value.remove_unavailable_videos();
                // `DataValue`=>`PlaylistDataValue`にparseできないのは異常なため
                // FetchErrorを返す
                PlaylistApiResponse::new(resp, url_pl).map_err(FetchError::Parse)?,
            )),
            // `Value`=>`DataValue`にパースできない
            // eq `playlist_id`が無効なとき
            // TODO 上本当か確かめる
            Err(e) => {
                tracing::trace!(
                    "response(Value) cannot parse to `DataValue`, reason: {:?}",
                    e
                );
                Ok(None)
            }
        }
    }

    fn merge_playlist_data(
        old_pl_data: &mut Option<PlaylistApiResponse>,
        new_pl_data: PlaylistApiResponse,
    ) -> Result<(), FetchError> {
        if let Some(old_pl_data) = old_pl_data {
            // TODO エラー型要検討
            old_pl_data.merge(new_pl_data).map_err(FetchError::Parse)?;
        } else {
            *old_pl_data = Some(new_pl_data);
        }
        Ok(())
    }

    fn build_next_page_url(
        &self,
        url_pl: &UrlPlaylist,
        next_page_token: Option<PageToken>,
    ) -> Option<String> {
        next_page_token.map(|token| {
            format!(
                "{}&pageToken={}",
                self.build_playlist_api_url(url_pl),
                *token
            )
        })
    }
}
