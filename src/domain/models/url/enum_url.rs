use std::collections::VecDeque;
use std::str::FromStr;

use regex::Regex;

use super::super::id::{id_type::IdType, PlaylistId, VideoId};
use super::{UrlPlaylist, UrlVideo};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Url {
    Video(UrlVideo),
    Playlist(UrlPlaylist),
}

impl Url {
    pub fn new(s: String) -> Result<Self, crate::Error> {
        // `v=`や`list=`などの指定に対応する`id`が取得できなければエラーを返す
        //
        // e.g. ~.com/watch?v=1234567890
        // このとき`id`の文字数が足りず、`id`を求める正規表現にはマッチしないが
        // `v=`が存在するので`id`が無いのはurlに誤りがあると判断
        let must_include_id = |id: bool, must_include: bool| {
            if must_include && !id {
                Err(crate::Error::InvalidInput)
            } else {
                Ok(())
            }
        };

        // [0] https://www.youtube.com/watch?v=(id)
        // [1] https://www.youtube.com/playlist?list=(pl)
        // [2] https://www.youtube.com/watch?v=(id)&list=(pl)
        // [3] https://youtu.be/(id)
        // [4] https://youtu.be/(id)&list=(pl)
        // [5] https://www.youtube.com/shorts/(id)

        // [0][2..=5]から抽出
        let re_video_id = Regex::new(Self::RE_V_ID_VAL).unwrap();
        let v_id: Option<VideoId> = Self::extract_id(re_video_id, &s)?;
        let re_must_include_video_id = Regex::new(Self::RE_V_ID_KEY).unwrap();
        must_include_id(v_id.is_some(), re_must_include_video_id.is_match(&s))?;

        // [1][4]から抽出
        let re_pl_id = Regex::new(Self::RE_PL_ID_VAL).unwrap();
        let pl_id: Option<PlaylistId> = Self::extract_id(re_pl_id, &s)?;
        let re_must_include_pl_id = Regex::new(Self::RE_PL_ID_KEY).unwrap();
        must_include_id(pl_id.is_some(), re_must_include_pl_id.is_match(&s))?;

        // urlに誤りがあり、どちらのidも含まれていない(どちらの正規表現にもマッチしない)とき
        let id_type = IdType::new(v_id, pl_id)?;

        // NOTE `id`の文字数が規定数**以上**のとき、判定できないが一旦無視
        Ok(id_type.into())
    }

    const RE_V_ID_VAL: &str =
        r"(?:(?:shorts/)|(?:v=)|(?:youtu\.be/))([0-9A-Za-z_-]{11})";
    const RE_V_ID_KEY: &str = r"shorts/|v=|youtu.be/";
    const RE_PL_ID_VAL: &str = r"(?:list=)([0-9A-Za-z_-]{34})";
    const RE_PL_ID_KEY: &str = r"list=";

    pub fn build_url(&self) -> String {
        match self {
            Self::Video(v) => v.build_url(),
            Self::Playlist(pl) => pl.build_url(),
        }
    }

    /// 引数のurlに引数の正規表現を使用し、キャプチャグループの1番目の要素を返す
    ///
    /// 見つからないと`Ok(None)`を返す
    ///
    /// 見つかると`try_into`を適用し、`try_into`の戻り値の`Result`を露出させて返す
    fn extract_id<T>(re: Regex, url: &str) -> Result<Option<T>, crate::Error>
    where
        T: FromStr<Err = crate::Error>,
    {
        re.captures(url)
            .and_then(|caps| caps.get(1).map(|m| m.as_str()))
            .map(|s| T::from_str(s))
            .transpose()
    }

    pub fn separate_urls(
        mut urls: VecDeque<Url>,
    ) -> (VecDeque<UrlVideo>, VecDeque<UrlPlaylist>) {
        let mut multi_v: VecDeque<UrlVideo> = VecDeque::new();
        let mut multi_pl: VecDeque<UrlPlaylist> = VecDeque::new();
        if let Some(url) = urls.pop_front() {
            match url {
                Self::Video(v) => multi_v.push_back(v),
                Self::Playlist(pl) => multi_pl.push_back(pl),
            }
        }
        (multi_v, multi_pl)
    }
}

impl From<IdType> for Url {
    fn from(value: IdType) -> Self {
        match value {
            IdType::Video(v) => UrlVideo::from(v).into(),
            IdType::Playlist(pl) => UrlPlaylist::from((None, pl)).into(),
            IdType::VideoPlaylist(v, pl) => UrlPlaylist::from((Some(v), pl)).into(),
        }
    }
}

impl From<UrlVideo> for Url {
    fn from(value: UrlVideo) -> Self {
        Url::Video(value)
    }
}

impl From<UrlPlaylist> for Url {
    fn from(value: UrlPlaylist) -> Self {
        Self::Playlist(value)
    }
}

#[cfg(test)]
impl Url {
    /// test only, return:`Self::VideoId(a)`
    ///
    /// `a:VideoId(12345678901)`
    pub(crate) fn v_inc_from_1() -> Self {
        IdType::from(VideoId::inc_from_1()).into()
    }
    /// test only, return:`Self::PlaylistId(b)`
    ///
    /// `b:PlaylistId(1234567890123...1234`
    pub(crate) fn pl_inc_from_1() -> Self {
        IdType::from(PlaylistId::inc_from_1()).into()
    }
    /// test only, return:`Self::VideoAndPlaylist(inner)`
    ///
    /// - `VideoId(12345678901)`
    /// - `PlaylistId(1234567890123...1234)`
    pub(crate) fn v_pl_inc_from_1() -> Self {
        let v_id = VideoId::inc_from_1();
        let pl_id = PlaylistId::inc_from_1();
        IdType::from((v_id, pl_id)).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_for_test_fn() {
        let _a = Url::v_inc_from_1();
        let _b = Url::pl_inc_from_1();
        let _c = Url::v_pl_inc_from_1();
    }

    /// `video_id` のみを持つとき
    #[test]
    fn test_url_gives_valid_1() {
        let expect_url = Url::v_inc_from_1();

        let url = "https://www.youtube.com/watch?v=12345678901".to_string();
        assert_eq!(Url::new(url.clone()), Ok(expect_url.clone()));

        let url = "https://youtu.be/12345678901".to_string();
        assert_eq!(Url::new(url), Ok(expect_url.clone()));

        let url = "https://www.youtube.com/shorts/12345678901".to_string();
        assert_eq!(Url::new(url), Ok(expect_url));
    }

    /// `playlist_id` のみを持つとき
    #[test]
    fn test_url_playlist_gives_valid() {
        let expect_url = Url::pl_inc_from_1();
        let url = format!(
            "{}{}",
            "https://www.youtube.com/playlist?list=",
            "1234567890123456789012345678901234"
        );
        assert_eq!(Url::new(url.clone()), Ok(expect_url));
    }

    /// `video_id`と`playlist_id` を持つとき
    #[test]
    fn test_url_video_and_playlist_gives_valid() {
        let expect_url = Url::v_pl_inc_from_1();
        let url = format!(
            "{}{}",
            "https://www.youtube.com/watch?v=12345678901",
            "&list=1234567890123456789012345678901234"
        );
        assert_eq!(Url::new(url.clone()), Ok(expect_url.clone()));
        let url = format!(
            "{}{}",
            "https://youtu.be/12345678901", "&list=1234567890123456789012345678901234"
        );
        assert_eq!(Url::new(url.clone()), Ok(expect_url.clone()));
    }

    #[test]
    fn test_url_gives_invalid() {
        // 動画idの指定が空文字列
        let invalid_url = "https://www.youtube.com/watch?v=".to_string();
        assert!(Url::new(invalid_url).is_err());
        let invalid_url = "https://youtu.be/".to_string();
        assert!(Url::new(invalid_url).is_err());

        // 動画idの文字数が少ない
        let invalid_url = "https://www.youtube.com/watch?v=123".to_string();
        assert!(Url::new(invalid_url).is_err());
        let invalid_url = "https://youtu.be/123".to_string();
        assert!(Url::new(invalid_url).is_err());

        // 動画idと再生リストidの両方の指定がないとき
        let invalid_url = "https://www.youtube.com".to_string();
        assert!(Url::new(invalid_url).is_err());
        let invalid_url = "https://youtu.be".to_string();
        assert!(Url::new(invalid_url).is_err());
    }
}
