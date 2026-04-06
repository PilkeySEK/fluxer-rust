mod ready;

use std::sync::Arc;

pub use ready::*;

use crate::Cache;

pub(crate) trait FromNonCached {
    type NonCached;
    async fn from_noncached(non_cached: Self::NonCached, cache: &Arc<Cache>) -> Self;
}

macro_rules! cache_vec {
    ($input:expr, $cache:expr, $converter:expr) => {{
        let mut cached_vec = Vec::with_capacity($input.len());
        for elem in $input {
            cached_vec.push(
                $crate::traits::CacheValue::insert_and_return($converter(elem), $cache).await,
            );
        }
        cached_vec
    }};
}

pub(crate) use cache_vec;

macro_rules! cache_option_vec {
    ($input:expr, $cache:expr, $converter:expr) => {{
        if let Some(some_input) = $input {
            Some($crate::gateway::cached_payload::cache_vec!(
                some_input, $cache, $converter
            ))
        } else {
            None
        }
    }};
}

pub(crate) use cache_option_vec;
