use macroquad::prelude::*;
use macroquad_virtual_joystick::Joystick;

fn render_background(x: f32, y: f32, radius: f32) {
    draw_circle(x, y, radius, RED);
}

fn render_knob(x: f32, y: f32, radius: f32) {
    draw_circle(x, y, radius, GREEN);
}

#[macroquad::main("Custom Joystick")]
async fn main() {
    const SPEED: f32 = 2.5;
    let mut position = Vec2::new(screen_width() / 2.0, screen_height() / 4.0);

    let background_size = 50.0;
    let knob_size = 32.0;

    let mut joystick = Joystick::from_custom_elements(
        100.0,
        200.0,
        background_size,
        knob_size,
        Box::new(render_background),
        Box::new(render_knob),
    );
    loop {
        clear_background(WHITE);

        let joystick_event = joystick.update();
        position += joystick_event.direction.to_local() * joystick_event.intensity * SPEED;

        draw_circle(position.x, position.y, 50.0, YELLOW);

        joystick.render();
        next_frame().await
    }
}
