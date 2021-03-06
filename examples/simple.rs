use macroquad::prelude::*;
use macroquad_virtual_joystick::Joystick;

#[macroquad::main("Simple Joystick")]
async fn main() {
    const SPEED: f32 = 2.5;
    let mut position = Vec2::new(screen_width() / 2.0, screen_height() / 4.0);
    let mut joystick = Joystick::new(100.0, 200.0, 50.0);
    loop {
        clear_background(WHITE);

        let joystick_event = joystick.update();
        position += joystick_event.direction.to_local() * joystick_event.intensity * SPEED;

        draw_circle(position.x, position.y, 50.0, YELLOW);

        // if you use a camera, reset the camera here with `set_default_camera()`
        joystick.render();
        next_frame().await
    }
}
