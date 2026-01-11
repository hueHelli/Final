use std::time::{Duration, Instant};

/// Timekeeping for the engine.
/// - `delta`: time between the last two update() calls.
/// - `elapsed`: total time since Time was created
pub struct Time {
    last_instant: Instant,
    pub delta: Duration,
    pub elapsed: Duration,
}

impl Time {
    /// Create  a new Time tracker starting at "now".
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            last_instant: now,
            delta: Duration::from_secs(0),
            elapsed: Duration::from_secs(0),
        }
    }

    /// Update the time values for this frame.
    /// Call this once per frame (e. g., in Engine::update()).
    pub fn update(&mut self) {
        let now = Instant::now();
        let frame_delta = now.duration_since(self.last_instant);
        self.last_instant = now;

        self.delta = frame_delta;
        self.elapsed += frame_delta;
    }

    /// Convenient helper for getting delta as seconds (f32).
    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    pub fn elapsed_seconds(&self) -> f32 {
        self.elapsed.as_secs_f32()
    }
}
