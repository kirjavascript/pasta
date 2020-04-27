static URLCHARS: &str = "mDctrCgv7BJoNhdOWRlesYI28FfKEnx1uP0pGzb9jQUaH6-iLV_4Z5kqTSM3ywAX~";
// . is also a valid url char but looks weird

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone)]
pub struct Urls(Arc<AtomicUsize>);

impl Urls {
    pub fn new() -> Self {
        Self(Arc::new(AtomicUsize::new(0)))
    }
    pub fn next(&self) -> String {
        hash(self.0.fetch_add(1, Ordering::SeqCst), 1)
    }
}

fn hash(index: usize, max_length: u32) -> String {
    let length = URLCHARS.len();
    let mut index = index % length.pow(max_length);

    let indices = if index == 0 {
        vec![0]
    } else {
        let mut indices = Vec::new();
        while index > 0 {
            let cur = index % length;
            indices.push(cur);
            index -= cur;
            index /= length;
        }
        indices
    };

    indices
        .iter()
        .map(|i| &URLCHARS[*i..=*i])
        .collect()
}
