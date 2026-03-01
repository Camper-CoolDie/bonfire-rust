use std::time::{Duration, Instant};

use tokio::sync::Mutex;

struct Bucket {
    capacity: f32,
    tokens: f32,
    last_refill: Instant,
    refill_rate: f32, // Tokens per second
}

pub(crate) struct RequestLimiter {
    bucket: Mutex<Bucket>,
}
impl RequestLimiter {
    // `limit`: requests per minute
    pub(crate) fn new(limit: u16) -> Self {
        let capacity = f32::from(limit);

        Self {
            bucket: Mutex::new(Bucket {
                capacity,
                tokens: capacity, // Start with a full bucket
                last_refill: Instant::now(),
                refill_rate: capacity / 60.,
            }),
        }
    }

    pub(crate) async fn wait_for_permit(&self) {
        let mut bucket = self.bucket.lock().await;

        // Refill tokens based on elapsed time
        let now = Instant::now();
        let elapsed = now.duration_since(bucket.last_refill).as_secs_f32();
        bucket.tokens = (bucket.tokens + elapsed * bucket.refill_rate).min(bucket.capacity);
        bucket.last_refill = now;

        // Check if a token is available
        if bucket.tokens >= 1. {
            bucket.tokens -= 1.;

            tracing::debug!(
                tokens = bucket.tokens,
                capacity = bucket.capacity,
                refill_rate = bucket.refill_rate,
                "consumed 1 token"
            );
        } else {
            tracing::warn!(
                tokens = bucket.tokens,
                refill_rate = bucket.refill_rate,
                "waiting until a token is available"
            );

            let wait_duration = Duration::from_secs_f32((1. - bucket.tokens) / bucket.refill_rate);
            drop(bucket);
            tokio::time::sleep(wait_duration).await;
        }
    }
}
