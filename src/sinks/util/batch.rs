use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct BatchBytesConfig {
    pub max_size: Option<usize>,
    pub timeout_secs: Option<u64>,
}

impl BatchBytesConfig {
    pub fn unwrap_or(&self, size: u64, timeout: u64) -> BatchSettings {
        BatchSettings {
            size: self.max_size.unwrap_or(size as usize),
            timeout: Duration::from_secs(self.timeout_secs.unwrap_or(timeout)),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct BatchEventsConfig {
    pub max_events: Option<usize>,
    pub timeout_secs: Option<u64>,
}

impl BatchEventsConfig {
    pub fn unwrap_or(&self, size: u64, timeout: u64) -> BatchSettings {
        BatchSettings {
            size: self.max_events.unwrap_or(size as usize),
            timeout: Duration::from_secs(self.timeout_secs.unwrap_or(timeout)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BatchSettings {
    pub size: usize,
    pub timeout: Duration,
}

pub trait Batch {
    type Input;
    type Output;

    fn push(&mut self, item: Self::Input);
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
    fn fresh(&self) -> Self;
    fn finish(self) -> Self::Output;
    fn num_items(&self) -> usize;

    /// Replace the current batch with a fresh one, returning the old one.
    fn fresh_replace(&mut self) -> Self
    where
        Self: Sized,
    {
        let fresh = self.fresh();
        std::mem::replace(self, fresh)
    }
}
