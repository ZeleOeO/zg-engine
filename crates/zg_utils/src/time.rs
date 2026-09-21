use std::time::Instant;

#[derive(Debug)]
pub struct Time {
    time_delta: f32,
    last_frame: Instant,
}

impl Time {
    pub fn new() -> Self {
        Self {
            time_delta: 0.0,
            last_frame: Instant::now(),
        }
    }

    // This checks between the last time and now
    pub fn update(&mut self) -> f32 {
        let now = Instant::now();
        self.time_delta = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;
        self.time_delta
    }

    pub fn time_delta_secs(&self) -> f32 {
        self.time_delta
    }

    pub fn time_delta_ms(&self) -> f32 {
        self.time_delta * 1000.0
    }

    pub fn fps(&self) -> u32 {
        1000 / (self.time_delta_ms() as u32) as u32
    }
}

// So on first run, time is nothing and last_frame is the current one, all the updates will run
// I believe we would possibly face an issue with a 0 delta time if we run this before the camera update
// if we do, I'll either change it to run at the beginning of each frame orrrr, I update the
// intital time_delta to 0.1
// pub fn update_time_system(world: &mut World) {
//     let mut time = world.get_mut::<Time>();
//     // Every update, this checks the current time
//     // gets how much it is from the previous time
//     time.calculate_time_delta();
// }
