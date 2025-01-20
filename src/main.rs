mod color;
mod constant;
use minifb::{Scale, ScaleMode, Window, WindowOptions};
use constant::*;


fn main() {
    let mut memory:Box<[u16]>= vec![0;MEMORY_SIZE].into_boxed_slice();
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

    window.set_target_fps(30);

    while window.is_open(){
        for i in SCREEN_MEMORY_START..SCREEN_MEMORY_END {
            memory[i]= 0x2222
        }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&color::create_image_buffer(&memory[SCREEN_MEMORY_START..SCREEN_MEMORY_END]), WIDTH, HEIGHT)
            .unwrap();
    }
}