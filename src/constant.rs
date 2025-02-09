pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 192;
pub const MEMORY_SIZE:usize = 1<<16;
pub const SCREEN_MEMORY_SIZE:usize = WIDTH*HEIGHT/4;
pub const SCREEN_MEMORY_START:usize = MEMORY_SIZE-SCREEN_MEMORY_SIZE;
pub const SCREEN_MEMORY_END: usize = SCREEN_MEMORY_START+SCREEN_MEMORY_SIZE;