use mongodb_gridfs::options::GridFSBucketOptions;
use mongodb_gridfs::GridFSBucket;

pub trait DatabaseExt {
    fn bucket(self, options: Option<GridFSBucketOptions>) -> GridFSBucket;
}

impl DatabaseExt for mongodb::Database {
    fn bucket(self, options: Option<GridFSBucketOptions>) -> GridFSBucket {
        GridFSBucket::new(self, options)
    }
}
