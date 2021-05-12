simple joystick for macroquad games

The joystick can be updated by touches or mouse.
Feel free to contribute!

## Warning
This library is currently not stable. Each version can introduce breaking changes!

# Example
```
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

        joystick.render();
        next_frame().await
    }
}
```
