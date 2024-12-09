use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tracing::Level;

use super::ApiKey;
use crate::{domain::repositories::FetchBasicDataTrait, metadata::BasicData, url::Url};

use crate::infrastructure::fetch::{
    playlist::fetch_data::PlaylistApiClient, video::fetch_data::VideoApiClient,
};

#[derive(Debug)]
pub struct ApiClient {
    api_key: ApiKey,
    used_quota: Arc<AtomicUsize>,
}

impl ApiClient {
    pub fn new(api_key: ApiKey) -> Self {
        Self {
            api_key,
            used_quota: Arc::new(AtomicUsize::new(0)),
        }
    }
}

#[async_trait::async_trait]
impl FetchBasicDataTrait for ApiClient {
    #[tracing::instrument(level = Level::DEBUG)]
    async fn fetch_basic_data_with_urls(
        &self,
        urls: VecDeque<Url>,
    ) -> Result<VecDeque<Result<BasicData, Url>>, crate::Error> {
        tracing::debug!("async fn:`using_urls` is called");
        let (urls_v, urls_pl) = Url::separate_urls(urls);
        let mut res: VecDeque<Result<BasicData, Url>> = VecDeque::new();

        let video_api_call = VideoApiClient::new_default(
            self.api_key.clone(),
            Arc::clone(&self.used_quota),
        );
        let pl_api_call = PlaylistApiClient::new_default(
            self.api_key.clone(),
            Arc::clone(&self.used_quota),
        );

        let (video_res, pl_res) = tokio::join!(
            video_api_call.fetch_all_video_data(urls_v),
            pl_api_call.fetch_all_playlist_data(urls_pl)
        );

        res.extend(transform_vec_result(video_res?));
        res.extend(transform_vec_result(pl_res?));

        tracing::debug!("used quota:`{}`", self.used_quota.load(Ordering::Relaxed));
        Ok(res)
    }

    async fn fetch_basic_data_with_url(
        &self,
        url: Url,
    ) -> Result<Result<BasicData, Url>, crate::Error> {
        match url {
            Url::Video(v) => {
                let video_api_call = VideoApiClient::new_default(
                    self.api_key.clone(),
                    Arc::clone(&self.used_quota),
                );
                Ok(transform_result(video_api_call.fetch_video_data(v).await?))
            }
            Url::Playlist(pl) => {
                let pl_api_call = PlaylistApiClient::new_default(
                    self.api_key.clone(),
                    Arc::clone(&self.used_quota),
                );
                Ok(transform_result(pl_api_call.fetch_playlist_data(pl).await?))
            }
        }
    }
}

fn transform_vec_result<T, E, A, B>(
    input: VecDeque<Result<T, E>>,
) -> VecDeque<Result<A, B>>
where
    T: Into<A>,
    E: Into<B>,
{
    input.into_iter().map(|res| res.map(Into::into).map_err(Into::into)).collect()
}

fn transform_result<T, E, A, B>(input: Result<T, E>) -> Result<A, B>
where
    T: Into<A>,
    E: Into<B>,
{
    input.map(Into::into).map_err(Into::into)
}

// impl ApiCall {
//     #[allow(unused)]
//     #[tracing::instrument(level = Level::DEBUG)]
//     async fn refetch_for_v_data_in_pl(
//         &self,
//         video_ids_from_multi_pl: VecDeque<Result<VideosInPlaylist, UrlPlaylist>>,
//         video_api_call: &VideoApiCall,
//     ) -> Result<VecDeque<Result<BasicPlaylistData, UrlPlaylist>>, FetchError> {
//         // 最後に無効な`url_playlist`を追加しているので順番が一部失われる
//         let mut invalid_url_playlist: VecDeque<UrlPlaylist> = VecDeque::new();
//         let mut res: VecDeque<Result<BasicPlaylistData, UrlPlaylist>> = VecDeque::new();
//         let mut tasks = FuturesOrdered::new();
//         let mut pl_data_itself = VecDeque::new();

//         for v_ids_from_pl in video_ids_from_multi_pl {
//             match v_ids_from_pl {
//                 Ok(v) => {
//                     pl_data_itself.push_back(v.playlist_data_itself.clone());
//                     let args: VecDeque<UrlVideo> =
//                         v.video_ids.into_iter().map(Into::into).collect();
//                     tasks.push_back(video_api_call.fetch_data(args));
//                 }
//                 Err(url) => invalid_url_playlist.push_back(url),
//             }
//         }
//         while let Some(res_v) = tasks.next().await {
//             let playlist_data = match res_v {
//                 Ok(v) => {
//                     let a: VecDeque<BasicVideoData> = v
//                         .iter()
//                         .flat_map(|item| match item {
//                             Ok(single_video_data) => Some(single_video_data.clone()),
//                             Err(_) => None,
//                         })
//                         .collect();
//                     into_fetch_data(a, pl_data_itself.pop_front().unwrap())
//                 }
//                 Err(e) => return Err(e),
//             };
//             res.push_back(Ok(playlist_data));
//         }
//         res.extend(
//             invalid_url_playlist
//                 .into_iter()
//                 .map(Err)
//                 .collect::<VecDeque<Result<BasicPlaylistData, UrlPlaylist>>>(),
//         );
//         Ok(res)
//     }
// }
