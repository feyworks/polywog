use crate::core::{Monitor, TimeState};
use std::time::{Duration, Instant};

const COMMON_FRAMERATES: [f64; 6] = [360.0, 240.0, 144.0, 120.0, 60.0, 30.0];
const SNAP_THRESHOLD: f64 = 0.0002;

/// Manages values for tracking time in the app loop.
#[derive(Debug)]
pub(crate) struct FrameTimer {
    pub target_fps: Option<f64>,
    pub time: TimeState,
    pub prev_time: Option<Instant>,
    pub accum: Duration,
    pub prev_frame: Option<Instant>,
    pub snapshots: Vec<f64>,
}

impl FrameTimer {
    pub fn new(target_fps: Option<f64>) -> Self {
        Self {
            target_fps,
            time: TimeState::default(),
            prev_time: None,
            accum: Duration::from_secs(0),
            prev_frame: None,
            snapshots: Vec::new(),
        }
    }

    pub fn tick<F: FnMut(TimeState)>(&mut self, display: Option<Monitor>, mut update_fn: F) {
        let refresh_rate = display
            .and_then(|monitor| monitor.refresh_rate_mhz())
            .unwrap_or(60000);

        let target_fps = self.target_fps.unwrap_or((refresh_rate as f64) / 1000.0);
        let frame_duration = Duration::from_secs_f64(1.0 / target_fps);

        // check how much time has passed since the last render
        let curr_time = Instant::now();
        let mut delta = (curr_time - self.prev_time.unwrap_or(curr_time)).as_secs_f64();
        self.prev_time = Some(curr_time);

        // if the delta is close to a nice framerate, snap it
        for fps in COMMON_FRAMERATES {
            delta = ((delta - 1.0 / fps).abs() < SNAP_THRESHOLD)
                .then(|| 1.0 / fps)
                .unwrap_or(delta)
        }

        // accumulate time so we know when to trigger a frame
        self.accum = self.accum + Duration::from_secs_f64(delta); //.min(max_duration);

        // when accumulator exceeds our target frame time, perform a frame
        if self.accum >= frame_duration {
            while self.accum >= frame_duration {
                self.accum -= frame_duration;

                // update the timer
                let delta = frame_duration.as_secs_f32();
                self.time.delta.set(delta);
                self.time.since_startup.update(|t| t + delta);
                self.time.frame.update(|f| f + 1);

                // notify that an update happened
                update_fn(self.time.clone());
            }
        }

        // track the FPS over the last 60 frames
        let curr_frame = Instant::now();
        let frame_diff = (curr_frame - self.prev_frame.unwrap_or(curr_frame)).as_secs_f64();
        self.prev_frame = Some(curr_frame);
        self.snapshots.push(frame_diff);
        if self.snapshots.len() > 60 {
            self.snapshots.rotate_left(1);
            self.snapshots.pop();
        }
        self.time.fps.set(
            (1.0 / (self.snapshots.iter().sum::<f64>() / (self.snapshots.len() as f64))).round()
                as u32,
        );
    }
}
