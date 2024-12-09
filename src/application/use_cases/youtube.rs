use std::{collections::VecDeque, fmt::Debug};
use tracing::Level;

use crate::domain::repositories::FetchBasicDataTrait;
use crate::{metadata::BasicData, url::Url};

#[derive(Debug)]
pub struct YouTubeService<T: FetchBasicDataTrait> {
    api: T,
}

impl<T> YouTubeService<T>
where
    T: FetchBasicDataTrait + Debug,
{
    pub fn new(api: T) -> Self {
        Self { api }
    }

    #[tracing::instrument(level = Level::DEBUG)]
    pub async fn using_urls(
        &self,
        urls: VecDeque<Url>,
    ) -> Result<VecDeque<Result<BasicData, Url>>, crate::Error> {
        self.api.fetch_basic_data_with_urls(urls).await
    }

    #[tracing::instrument(level = Level::DEBUG)]
    pub async fn using_url(
        &self,
        url: Url,
    ) -> Result<Result<BasicData, Url>, crate::Error> {
        self.api.fetch_basic_data_with_url(url).await
    }
}
