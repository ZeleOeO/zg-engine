use std::time::Instant;

#[derive(Debug)]
pub struct Time {
    time_delta: f32,
    last_frame: Instant,
    frames: u32,
    elapsed: f32,
    fps: f32,
}

impl Time {
    pub fn new() -> Self {
        Self {
            time_delta: 0.0,
            last_frame: Instant::now(),
            frames: 0,
            elapsed: 0.0,
            fps: 0.0,
        }
    }

    // This checks between the last time and now
    pub fn update(&mut self) {
        self.frames += 1;
        let now = Instant::now();
        self.time_delta = now.duration_since(self.last_frame).as_secs_f32();
        self.elapsed += self.time_delta;
        self.last_frame = now;

        // This should be accurate regardless of what the time_delta says
        if self.elapsed >= 1.0 {
            self.fps = self.frames as f32 / self.elapsed;
            println!("FPS: {:#?}", self.fps().floor());
            self.elapsed = 0.0;
            self.frames = 0;
        }
    }

    pub fn time_delta_secs(&self) -> f32 {
        self.time_delta
    }

    pub fn time_delta_ms(&self) -> f32 {
        self.time_delta * 1000.0
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }
}
