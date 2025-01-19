use minifb::{Scale, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 640 ;
const HEIGHT: usize = 420;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

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

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    while window.is_open(){
        for i in buffer.iter_mut() {
            *i = 0x5F0F0F; // write something more funny here!
        }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}