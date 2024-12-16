// external
use serde::Deserialize;
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tracing::Level;

// crate
use super::super::{
    api_request::ApiRequest, collections::quota, response::ApiResponse, ApiKey,
    ApiOptionsPart, FetchError, MaxIdNum,
};
use super::response::VideoApiResponse;
use crate::{id::VideoId, metadata::FullVideoData, url::UrlVideo};

#[derive(Debug)]
pub struct VideoApiClient {
    max_id: MaxIdNum,
    api_options: ApiOptionsPart,
    api_key: ApiKey,
    used_quota: Arc<AtomicUsize>,
}

impl ApiRequest for VideoApiClient {}

impl VideoApiClient {
    pub fn new_default(api_key: ApiKey, used_quota: Arc<AtomicUsize>) -> Self {
        Self {
            max_id: MaxIdNum::default(),
            api_options: ApiOptionsPart::default(),
            api_key,
            used_quota,
        }
    }

    pub async fn fetch_all_video_data(
        &self,
        mut urls: VecDeque<UrlVideo>,
    ) -> Result<VecDeque<Result<FullVideoData, UrlVideo>>, FetchError> {
        let mut fetched_data: VecDeque<Result<FullVideoData, UrlVideo>> =
            VecDeque::new();

        while !urls.is_empty() {
            // `max_id`の数までしか一度にリクエストを送れないので,送れる数を抽出
            let part_urls: VecDeque<UrlVideo> =
                urls.drain(..usize::from(self.max_id).min(urls.len())).collect();
            let video_data = self.process_video(&part_urls).await?;

            fetched_data.extend(self.map_fetched_data_to_urls(&video_data, &part_urls));
        }
        Ok(fetched_data)
    }

    pub async fn fetch_video_data(
        &self,
        url: UrlVideo,
    ) -> Result<Result<FullVideoData, UrlVideo>, FetchError> {
        let video_data = self.process_video(&vec![url.clone()].into()).await?;
        match video_data {
            Some(v) => Ok(Ok(v.get_item_by_id(&url.into()).unwrap())),
            None => Ok(Err(url)),
        }
    }

    /// part_urlsをまとめてデータをfetchする
    ///
    /// - 必須: part_urlsは`max_id`の値以下
    #[tracing::instrument(level = Level::TRACE)]
    async fn process_video(
        &self,
        part_urls: &VecDeque<UrlVideo>,
    ) -> Result<Option<VideoApiResponse>, FetchError> {
        let url = self.build_video_api_url(part_urls);
        self.used_quota.fetch_add(quota::VIDEO_INFO, Ordering::Relaxed);
        let response = self.api_call(&url).await?;
        let video_resp = ApiResponse::deserialize(response).map_err(|e| e.to_string());

        match video_resp {
            Ok(v) => {
                tracing::trace!("fetched content: {:?}", v);
                // `DataValue`=>`VideoDataValue`にパースできないのは異常なため
                // FetchErrorを返す
                Ok(Some(
                    VideoApiResponse::try_from(v).map_err(FetchError::Parse)?,
                ))
            }
            // `Value`=>`DataValue`にパースできない
            // eq 全動画の`video_id`が無効なとき
            // TODO 上本当か確かめる
            Err(_) => Ok(None),
        }
    }

    fn build_video_api_url(&self, video_urls: &VecDeque<UrlVideo>) -> String {
        let id: Vec<&VideoId> = video_urls.iter().map(Into::into).collect();
        format!(
            "{}videos?key={}&part={}&id={}",
            Self::BASE_API_URL,
            self.api_key.as_string(),
            self.api_options.join(","),
            id.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",")
        )
    }

    /// 引数の`urls`とfetchした情報を照らし合わせfetchに成功したかどうかを判断
    ///
    /// urls(id)は`VecDequeue`に格納されている
    /// - `url(id)`が無効: `Err(Url)`が格納
    /// - `url(id)`が有効: `Ok(Video)`が格納
    fn map_fetched_data_to_urls(
        &self,
        video_data: &Option<VideoApiResponse>,
        urls: &VecDeque<UrlVideo>,
    ) -> VecDeque<Result<FullVideoData, UrlVideo>> {
        let mut res: VecDeque<Result<FullVideoData, UrlVideo>> = VecDeque::new();
        if let Some(video_data) = video_data {
            for url in urls {
                let part_res = match video_data.get_item_by_id(&url.clone().into()) {
                    // `video_id`が有効で(存在しており)情報が取得できたとき
                    Some(data) => Ok(data),
                    // `video_id`が無効で情報が取得できなかったとき
                    None => Err(url.clone()),
                };
                res.push_back(part_res);
            }
        // 全動画の`video_id`が無効で動画情報が取得できなかったとき
        } else {
            res = urls.iter().map(|url| Err(url.clone())).collect();
        }
        res
    }
}
