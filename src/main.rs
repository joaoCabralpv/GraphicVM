mod color;
mod constant;
mod clock;

use std::time;

use minifb::{Scale, ScaleMode, Window, WindowOptions};
use constant::*;

fn main() {

    let mut memory:Box<[u16]>= vec![0;MEMORY_SIZE].into_boxed_slice();
    let mut last_clock_step_time = std::time::Instant::now();
    let mut step_clock:bool;
    let mut clock_cicles :u32 = 0;
    
    let mut window = Window::new(
        "GraphicVM",
        WIDTH,
        HEIGHT,
        WindowOptions{
            borderless:false,
            title:true,resize:
            true,scale:Scale::X1,
            scale_mode:ScaleMode::AspectRatioStretch,
            topmost:false,
            transparency:false,
            none:false}

    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(1);
    println!("{}",clock::ClockPeriod.as_nanos());
    let start = time::Instant::now();
    while window.is_open(){
        // execute n instructions
        step_clock = clock::StepClock(last_clock_step_time);

        if step_clock {
            last_clock_step_time = std::time::Instant::now();
            clock_cicles+=1
        }

        if clock_cicles == 1500000 {
            break;
        }

        //window
        //    .update_with_buffer(&color::create_image_buffer(&memory), WIDTH, HEIGHT)
        //    .unwrap();
    }
    println!("{}cicles in {}s",clock_cicles,(start.elapsed().as_secs_f32()))
}