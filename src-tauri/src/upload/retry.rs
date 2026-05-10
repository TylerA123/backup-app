use std::time::Duration;

pub struct RetryPolicy {
    max_attempts: u32,
    base_delay_ms: u64,
}

impl RetryPolicy {
    pub fn new(max_attempts: u32, base_delay_ms: u64) -> Self {
        RetryPolicy {
            max_attempts,
            base_delay_ms,
        }
    }

    pub fn should_retry(&self, attempt: u32) -> bool {
        attempt < self.max_attempts
    }

    pub fn delay(&self, attempt: u32) -> Duration {
        let delay_ms = (self.base_delay_ms * 2u64.pow(attempt)).min(300_000);
        Duration::from_millis(delay_ms)
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        RetryPolicy {
            max_attempts: 5,
            base_delay_ms: 30_000,
        }
    }
}
