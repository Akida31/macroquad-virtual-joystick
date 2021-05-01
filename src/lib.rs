#![allow(dead_code)]

use macroquad::prelude::*;

pub struct Joystick {
    x: f32,
    y: f32,
    size: f32,
    is_fixed: bool,
    background: JoystickElement,
    knob: JoystickElement,
    dragging: bool,
    drag_position: Vec2,
}

impl Joystick {
    pub fn new(
        x: f32,
        y: f32,
        size: f32,
        is_fixed: bool,
        background: Option<JoystickElement>,
        knob: Option<JoystickElement>,
    ) -> Self {
        let radius = size / 2.;
        let center_x = x + radius;
        let center_y = y + radius;

        let background = background.unwrap_or_else(|| {
            JoystickElement::new(
                center_x,
                center_y,
                radius,
                Color::from_rgba(96, 125, 139, 128),
            )
        });
        let knob = knob.unwrap_or_else(|| {
            JoystickElement::new(
                center_x,
                center_y,
                radius / 2.,
                Color::from_rgba(96, 125, 139, 168),
            )
        });
        Self {
            x,
            y,
            size,
            is_fixed,
            background,
            knob,
            dragging: false,
            drag_position: Vec2::new(0., 0.),
        }
    }

    pub fn render(&self) {
        self.background.render();
        self.knob.render();
    }

    pub fn update(&mut self) {
        if self.dragging {
        } else {
        }
    }
}

pub struct JoystickElement {
    x: f32,
    y: f32,
    radius: f32,
    color: Color,
}

impl JoystickElement {
    pub fn new(x: f32, y: f32, radius: f32, color: Color) -> Self {
        Self {
            x,
            y,
            radius,
            color,
        }
    }

    pub fn render(&self) {
        draw_circle(self.x, self.y, self.radius, self.color);
    }
}

enum JoystickDirection {
    Up,
    UpLeft,
    Left,
    DownLeft,
    Down,
    DownRight,
    Right,
    UpRight,
    Idle,
}

impl JoystickDirection {
    pub fn from_degrees(degrees: f64) -> Self {
        if degrees > -22.5 && degrees <= 22.5 {
            Self::Right
        } else if degrees > 22.5 && degrees <= 67.5 {
            Self::DownRight
        } else if degrees > 67.5 && degrees <= 112.5 {
            Self::Down
        } else if degrees > 112.5 && degrees <= 157.5 {
            Self::DownLeft
        } else if degrees > 157.5 && degrees <= 180. {
            Self::Left
        } else if degrees > -157.5 && degrees <= -112.5 {
            Self::UpLeft
        } else if degrees > -112.5 && degrees <= -67.5 {
            Self::Up
        } else if degrees > -67.5 && degrees <= -22.5 {
            Self::UpRight
        } else {
            Self::Idle
        }
    }
}

enum JoystickActionKind {
    Down,
    Up,
    Move,
    Cancel,
}

pub struct JoystickDirectionalEvent {
    /// the direction to which the knob was moved
    direction: Option<JoystickDirection>,
    /// the intensity of the knob move, from 0 (center) to 1 (edge)
    intensity: f64,

    /// the angle of the knob (in radians)
    ///
    /// starting on the positive x-axis and rotating counter-clockwise
    angle: f64,
}
