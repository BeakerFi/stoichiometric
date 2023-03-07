//! Utility functions

use scrypto::prelude::{Bucket, ResourceAddress, ScryptoBucket};

/// Sorts two token addresses
pub fn sort(
    token_a: ResourceAddress,
    token_b: ResourceAddress,
) -> (ResourceAddress, ResourceAddress) {
    if token_a > token_b {
        (token_a, token_b)
    } else {
        (token_b, token_a)
    }
}

/// Sorts two buckets
pub fn sort_buckets(bucket_a: Bucket, bucket_b: Bucket) -> (Bucket, Bucket) {
    if bucket_a.resource_address() > bucket_b.resource_address() {
        (bucket_a, bucket_b)
    } else {
        (bucket_b, bucket_a)
    }
}