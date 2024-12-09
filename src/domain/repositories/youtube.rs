use std::collections::VecDeque;

use crate::{
    metadata::{BasicData, FullData},
    url::Url,
};

/// VideoIdを色々な方法で抽出
#[async_trait::async_trait]
pub trait FetchBasicDataTrait {
    /// 動画の基本的な情報を複数の`url(id)`から取得
    ///
    /// 内側の`Result<Video, Url>`:
    /// - Ok(Video): 正常に動画の情報を取得できたとき
    /// - Err(Url): `url(id)`が存在せず取得できなかったとき,
    /// その存在しない`url(id)`を含む
    ///
    /// 外側の`Result`: ネットワークエラーなどのエラー
    async fn fetch_basic_data_with_urls(
        &self,
        urls: VecDeque<Url>,
    ) -> Result<VecDeque<Result<BasicData, Url>>, crate::Error>;
    // 動画の基本的な情報を一つの`url(id)`から取得
    //
    // 内側の`Result<Video, Url>`:
    // - Ok(Video): 正常に動画の情報を取得できたとき
    // - Err(Url): `url(id)`が存在せず取得できなかったとき,
    // その存在しない`url(id)`を含む
    //
    // 外側の`Result`: ネットワークエラーなどのエラー
    async fn fetch_basic_data_with_url(
        &self,
        url: Url,
    ) -> Result<Result<BasicData, Url>, crate::Error>;
}

// FIXME いつか使うので #[allow(unused)]
#[allow(unused)]
#[async_trait::async_trait]
pub trait FetchDetailedDataTrait {
    /// 動画の詳細な情報を複数の`url(id)`から取得
    ///
    /// 内側の`Result<Video, Url>`:
    /// - Ok(Video): 正常に動画の情報を取得できたとき
    /// - Err(Url): `url(id)`が存在せず取得できなかったとき,
    /// その存在しない`url(id)`を含む
    ///
    /// 外側の`Result`: ネットワークエラーなどのエラー
    async fn fetch_detailed_with_urls(
        &self,
        url: VecDeque<Url>,
    ) -> Result<VecDeque<Result<FullData, Url>>, crate::Error>;

    /// 動画の詳細な情報を一つの`url(id)`から取得
    ///
    /// 内側の`Result<Video, Url>`:
    /// - Ok(Video): 正常に動画の情報を取得できたとき
    /// - Err(Url): `url(id)`が存在せず取得できなかったとき,
    /// その存在しない`url(id)`を含む
    ///
    /// 外側の`Result`: ネットワークエラーなどのエラー
    async fn fetch_detailed_with_url(
        &self,
        url: Url,
    ) -> Result<Result<FullData, Url>, crate::Error>;
}

// 難しそうなので今度 (0.2.0とかで追加)
// async fn using_handle_id(id: HandleId) -> Vec<Video>;
