use std::time;
pub const ClockFreqencyHz:u32 = 100000;
pub const ClockPeriod:time::Duration = time::Duration::from_nanos((1000000000 as f64/(ClockFreqencyHz as f64)) as u64);

pub fn StepClock(last_clock_step_time:time::Instant) -> bool {
    time::Instant::elapsed(&last_clock_step_time) >= ClockPeriod
}