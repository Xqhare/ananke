use std::{
    thread::sleep,
    time::{Duration, Instant},
};

pub fn fps_sleeper(last_frame: Instant) -> (Instant, u128) {
    let now = Instant::now();
    // While in debug, lets run basically uncapped to see what happens
    // For release, cap to 2k fps
    let fps_goal = if cfg!(debug_assertions) {
        60_000
    } else {
        2_000
    };
    let ms_goal = 1000 / fps_goal;
    let last_frame_dur = now.duration_since(last_frame).as_millis();
    if last_frame_dur < ms_goal {
        sleep(Duration::from_millis(
            ms_goal as u64 - now.duration_since(last_frame).as_millis() as u64,
        ));
    }
    (now, last_frame_dur)
}
