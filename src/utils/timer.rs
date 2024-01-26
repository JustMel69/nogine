use std::time::Instant;

use crate::log_info;

pub struct Timer<'a> {
    creation: Instant,
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn start(name: &'a str) -> Self {
        return Self { creation: Instant::now(), name };
    }

    pub fn end(self) {
        let ms = self.creation.elapsed().as_secs_f64() * 1000.0;
        log_info!("Timer ({}): {ms} ms", self.name);
    }
}