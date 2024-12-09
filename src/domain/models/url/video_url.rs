use super::super::id::{id_type::IdType, VideoId};
use super::build_url;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct UrlVideo(VideoId);

impl UrlVideo {
    pub fn build_url(&self) -> String {
        let id_type: IdType = self.0.clone().into();
        build_url::build_url(id_type)
    }
}

impl From<VideoId> for UrlVideo {
    fn from(value: VideoId) -> Self {
        Self(value)
    }
}

impl From<UrlVideo> for VideoId {
    fn from(value: UrlVideo) -> Self {
        value.0
    }
}

impl<'a> From<&'a UrlVideo> for &'a VideoId {
    fn from(value: &'a UrlVideo) -> Self {
        &value.0
    }
}
