// basis
pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod util;

// re-import
pub use domain::{id, metadata, url, Error, Handle};

// auxiliary
pub mod auxiliary;

pub mod prelude {
    pub use crate::application::YouTubeService;
    pub use crate::infrastructure::fetch::ApiKey;
    pub use crate::url::Url;
}

use application::YouTubeService;
use infrastructure::fetch::{ApiClient, ApiKey};
/// この関数は、YouTubeServiceを作成するためのヘルパー関数です。
pub fn youtube_service(api_key: ApiKey) -> YouTubeService<ApiClient> {
    YouTubeService::new(ApiClient::new(api_key))
}
