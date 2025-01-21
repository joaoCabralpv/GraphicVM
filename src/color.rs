use crate::constant::*;

pub const BLACK_VM        :u8  = 0x0;
pub const DARK_GREY_VM    :u8  = 0x1;
pub const RED_VM          :u8  = 0x2;
pub const DARK_RED_VM     :u8  = 0x3;
pub const GREEN_VM        :u8  = 0x4;
pub const DARK_GREEN_VM   :u8  = 0x5;
pub const BLUE_VM         :u8  = 0x6;
pub const DARK_BLUE_VM    :u8  = 0x7;
pub const YELLOW_VM       :u8  = 0x8;
pub const DARK_YELLOW_VM  :u8  = 0x9;
pub const PURPLE_VM       :u8  = 0xA;
pub const DARK_PURPLE_VM  :u8  = 0xB;
pub const CYAN_VM         :u8  = 0xC;
pub const DARK_CYAN_VM    :u8  = 0xD;
pub const GREY_VM         :u8  = 0xE;
pub const WHITE_VM        :u8  = 0xF;

pub const BLACK_RGB       :u32 = 0x000000;
pub const DARK_GREY_RGB   :u32 = 0x404040;
pub const RED_RGB         :u32 = 0xFF0000;
pub const DARK_RED_RGB    :u32 = 0x800000;
pub const GREEN_RGB       :u32 = 0x00FF00;
pub const DARK_GREEN_RGB  :u32 = 0x008000;
pub const BLUE_RGB        :u32 = 0x0000FF;
pub const DARK_BLUE_RGB   :u32 = 0x000080;
pub const YELLOW_RGB      :u32 = 0xFFFF00;
pub const DARK_YELLOW_RGB :u32 = 0x808000;
pub const PURPLE_RGB      :u32 = 0xFF00FF;
pub const DARK_PURPLE_RGB :u32 = 0x800080;
pub const CYAN_RGB        :u32 = 0x00FFFF;
pub const DARK_CYAN_RGB   :u32 = 0x008080;
pub const GREY_RGB        :u32 = 0x808080;
pub const WHITE_RGB       :u32 = 0xFFFFFF;

pub fn vm_color_to_rgb_color(color:u8)->u32{
    match color {
        BLACK_VM => BLACK_RGB,
        DARK_GREY_VM => DARK_GREY_RGB,
        RED_VM => RED_RGB,
        DARK_RED_VM => DARK_RED_RGB,
        GREEN_VM => GREEN_RGB,
        DARK_GREEN_VM => DARK_GREEN_RGB,
        BLUE_VM => BLUE_RGB,
        DARK_BLUE_VM => DARK_BLUE_RGB,
        YELLOW_VM => YELLOW_RGB,
        DARK_YELLOW_VM => DARK_YELLOW_RGB,
        PURPLE_VM => PURPLE_RGB,
        DARK_PURPLE_VM => DARK_PURPLE_RGB,
        CYAN_VM => CYAN_RGB,
        DARK_CYAN_VM => DARK_CYAN_RGB,
        GREY_VM => GREY_RGB,
        WHITE_VM => WHITE_RGB,
        n=>panic!("Invalid color {n}")
    }
}

pub fn create_image_buffer(memory:&[u16]) -> [u32;SCREEN_MEMORY_SIZE*4]
{
    let mut image_buffer:[u32;SCREEN_MEMORY_SIZE*4] = [0;SCREEN_MEMORY_SIZE*4];
    for i in 0..SCREEN_MEMORY_SIZE{
        //println!("{}",screen_memory[i]);
        let pixel1:u8 = (memory[i+SCREEN_MEMORY_START]&0xF).try_into().unwrap();
        let pixel2:u8 = ((memory[i+SCREEN_MEMORY_START]>>4)&0xF).try_into().unwrap();
        let pixel3:u8 = ((memory[i+SCREEN_MEMORY_START]>>8)&0xF).try_into().unwrap();
        let pixel4:u8 = ((memory[i+SCREEN_MEMORY_START]>>12)&0xF).try_into().unwrap();
        image_buffer[4*i]=vm_color_to_rgb_color(pixel1);
        image_buffer[(4*i)+1]=vm_color_to_rgb_color(pixel2);
        image_buffer[(4*i)+2]=vm_color_to_rgb_color(pixel3);
        image_buffer[(4*i)+3]=vm_color_to_rgb_color(pixel4);
    }
    image_buffer
}