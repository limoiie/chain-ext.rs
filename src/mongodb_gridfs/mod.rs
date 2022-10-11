#[cfg(feature = "mongodb")]
pub use database_ext::DatabaseExt;

#[cfg(feature = "mongodb")]
mod database_ext;
