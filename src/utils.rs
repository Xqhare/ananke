use std::{
    thread::sleep,
    time::{Duration, Instant},
};

pub fn fps_sleeper(last_frame: Instant) -> (Instant, u128) {
    let fps_goal = 2_000;
    let ms_goal = 1_000_000 / fps_goal;
    let now = Instant::now();
    let last_frame_dur = now.duration_since(last_frame).as_micros();
    if last_frame_dur < ms_goal {
        sleep(Duration::from_micros((ms_goal - last_frame_dur) as u64));
    }
    (now, last_frame_dur)
}
