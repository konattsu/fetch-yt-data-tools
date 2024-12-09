use serde::{de::Error, Deserialize, Deserializer};
use std::str::FromStr;
use tracing::level_filters::LevelFilter;

pub fn deserialize_option_level_filter<'de, D>(
    deserializer: D,
) -> Result<Option<LevelFilter>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(level) => Ok(Some(
            LevelFilter::from_str(&level).map_err(D::Error::custom)?,
        )),
        None => Ok(None),
    }
}
