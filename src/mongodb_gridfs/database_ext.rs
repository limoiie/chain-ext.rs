use mongodb_gridfs::options::GridFSBucketOptions;
use mongodb_gridfs::GridFSBucket;

/// Extend [mongodb::Database] with methods that construct objects with a database instance.
pub trait DatabaseExt {
    /// Create a bucket with the given options.
    fn bucket(self, options: Option<GridFSBucketOptions>) -> GridFSBucket;
}

impl DatabaseExt for mongodb::Database {
    fn bucket(self, options: Option<GridFSBucketOptions>) -> GridFSBucket {
        GridFSBucket::new(self, options)
    }
}
