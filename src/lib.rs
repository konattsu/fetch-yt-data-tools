// basis
pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod util;

// re-import
pub use domain::{id, metadata, url, Error, Handle};

// auxiliary
pub mod auxiliary;
