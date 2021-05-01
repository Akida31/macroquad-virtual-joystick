use macroquad::prelude::*;
use macroquad_virtual_joystick::Joystick;

#[macroquad::main("Simple Joystick")]
async fn main() {
    let joystick = Joystick::new(100., 200., 50., true, None, None);
    loop {
        clear_background(WHITE);
        
        joystick.render();

        next_frame().await
    }
}
