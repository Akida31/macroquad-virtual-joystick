//! A simple joystick for macroquad games
//!
//! The joystick can be updated by touches or mouse
//!
//! # Example
//! ```
//! use macroquad::prelude::*;
//! use macroquad_virtual_joystick::Joystick;
//!
//! #[macroquad::main("Simple Joystick")]
//! async fn main() {
//!     const SPEED: f32 = 2.5;
//!     let mut position = Vec2::new(screen_width() / 2.0, screen_height() / 4.);
//!     let mut joystick = Joystick::new(100.0, 200.0, 50.0);
//!     loop {
//!         clear_background(WHITE);
//!
//!         let joystick_event = joystick.update();
//!         position += joystick_event.direction.to_local() * joystick_event.intensity * SPEED;
//!
//!         draw_circle(position.x, position.y, 50., YELLOW);
//!
//!         //next_frame().await
//!         break;
//!     }
//! }
//! ```

use macroquad::prelude::{
    draw_circle, is_mouse_button_down, mouse_position, touches, Color, MouseButton, TouchPhase,
    Vec2,
};

/// The joystick component
///
/// # Examples
/// ```no_run
/// use macroquad_virtual_joystick::Joystick;
/// let center_x = 100.0;
/// let center_y = 50.0;
/// let size = 50.0;
/// // create a new joystick
/// let mut joystick = Joystick::new(center_x, center_y, size);
/// // render the joystick and determine the action
/// let joystick_action = joystick.update();
/// ```
pub struct Joystick {
    center: Vec2,
    size: f32,
    background: JoystickElement,
    knob: JoystickElement,
    dragging: bool,
    touch_id: u64,
    event: JoystickDirectionalEvent,
}

impl Joystick {
    /// create a new joystick
    ///
    /// # Arguments
    /// * `x`, `y`: center of the joystick
    /// * `size`: diameter of the joystick
    ///
    /// # Examples
    /// ```
    /// use macroquad_virtual_joystick::Joystick;
    /// let center_x = 100.0;
    /// let center_y = 50.0;
    /// let size = 50.0;
    /// let joystick = Joystick::new(center_x, center_y, size);
    /// ```
    pub fn new(x: f32, y: f32, size: f32) -> Self {
        Self::from_custom_elements(x, y, size, None, None)
    }

    /// create a new [`Joystick`] with custom elements for background and knob
    ///
    /// # Arguments
    /// * `x`, `y`: center of the joystick
    /// `size`: diameter of the joystick, should have the same size as the background element
    /// * `background`, `knob`: custom elements
    ///
    /// # Examples
    /// ```
    /// use macroquad::prelude::Color;
    /// use macroquad_virtual_joystick::{Joystick, JoystickElement};
    ///
    /// let center_x = 100.0;
    /// let center_y = 50.0;
    /// let radius = 50.0;
    /// let background_color = Color::from_rgba(255, 0, 0, 255);
    /// let knob_color = Color::from_rgba(0, 255, 0, 255);
    ///
    /// // create the background element
    /// let background = JoystickElement::new(
    ///     center_x,
    ///     center_y,
    ///     radius,
    ///     background_color,
    /// );
    ///
    /// // create the knob element
    /// let knob = JoystickElement::new(
    ///     center_x,
    ///     center_y,
    ///     radius * 0.75,
    ///     knob_color,
    /// );
    ///
    /// let joystick = Joystick::from_custom_elements(
    ///     center_x,
    ///     center_y,
    ///     radius * 2.0,
    ///     Some(background),
    ///     Some(knob),
    /// );
    /// ```
    pub fn from_custom_elements(
        x: f32,
        y: f32,
        size: f32,
        background: Option<JoystickElement>,
        knob: Option<JoystickElement>,
    ) -> Self {
        let radius = size / 2.;
        let center = Vec2::new(x, y);

        let background = background.unwrap_or_else(|| {
            JoystickElement::new(x, y, radius, Color::from_rgba(96, 125, 139, 128))
        });
        let knob = knob.unwrap_or_else(|| {
            JoystickElement::new(x, y, radius / 2., Color::from_rgba(96, 125, 139, 168))
        });

        Self {
            center,
            size,
            background,
            knob,
            dragging: false,
            touch_id: 0,
            event: JoystickDirectionalEvent::default(),
        }
    }

    /// renders the background and knob
    fn render(&self) {
        self.background.render();
        self.knob.render();
    }

    /// update the joystick from touch
    fn update_touch(&mut self) {
        for touch in touches() {
            match touch.phase {
                TouchPhase::Started => {
                    // a touch starts in the joystick
                    if (touch.position - self.center).length() < (self.size / 2.) {
                        self.dragging = true;
                        self.touch_id = touch.id;
                        self.moving(touch.position);
                    }
                }
                TouchPhase::Moved => {
                    if self.dragging && touch.id == self.touch_id {
                        self.moving(touch.position);
                    }
                }
                TouchPhase::Ended | TouchPhase::Cancelled => {
                    if self.dragging && touch.id == self.touch_id {
                        self.reset();
                    }
                }
                _ => {}
            }
        }
    }

    /// update the joystick from mouse drag
    fn update_mouse(&mut self) {
        let (mouse_x, mouse_y) = mouse_position();
        let mouse = Vec2::new(mouse_x, mouse_y);
        let mouse_down = is_mouse_button_down(MouseButton::Left);
        if self.dragging {
            if mouse_down {
                self.moving(mouse)
            } else {
                self.reset();
            }
        } else if mouse_down && (self.center - mouse).length() < (self.size / 2.) {
            self.dragging = true;
            self.moving(mouse)
        }
    }

    /// reset the joystick
    fn reset(&mut self) {
        self.dragging = false;
        self.knob.x = self.center.x;
        self.knob.y = self.center.y;
        self.event = JoystickDirectionalEvent::default();
    }

    /// update the joystick
    ///
    /// this updates the joystick and renders it
    /// returns the current [`JoystickDirectionalEvent`]
    ///
    /// # Examples
    /// see [`Joystick`]
    pub fn update(&mut self) -> JoystickDirectionalEvent {
        if touches().len() > 0 {
            self.update_touch();
        } else {
            self.update_mouse();
        }
        self.render();
        self.event
    }

    /// move the knob according to the drag position and update the [`self.event`]
    fn moving(&mut self, position: Vec2) {
        let delta = position - self.center;
        let angle = delta.y.atan2(delta.x);
        let angle_degrees = angle.to_degrees();
        let radius = self.size / 2.;

        // maximum distance for the knob
        let dist = f32::min(delta.length(), radius);

        self.knob.x = self.center.x + dist * angle.cos();
        self.knob.y = self.center.y + dist * angle.sin();

        let intensity = dist / radius;
        let direction = if intensity == 0. {
            JoystickDirection::Idle
        } else {
            JoystickDirection::from_degrees(angle_degrees as f64)
        };
        self.event = JoystickDirectionalEvent::new(direction, intensity, angle);
    }
}

/// element of the [`Joystick`]
///
/// can be used for the background or the knob
pub struct JoystickElement {
    x: f32,
    y: f32,
    radius: f32,
    color: Color,
}

impl JoystickElement {
    /// create a new [`JoystickElement`]
    ///
    /// # Examples
    /// ```
    /// use macroquad::prelude::Color;
    /// use macroquad_virtual_joystick::JoystickElement;
    ///
    /// let center_x = 50.0;
    /// let center_y = 100.0;
    /// let radius = 30.0;
    /// let background_color = Color::from_rgba(255, 0, 0, 255);
    ///
    /// let background = JoystickElement::new(
    ///     center_x,
    ///     center_y,
    ///     radius,
    ///     background_color,
    /// );
    /// ```
    pub fn new(x: f32, y: f32, radius: f32, color: Color) -> Self {
        Self {
            x,
            y,
            radius,
            color,
        }
    }

    /// render the element
    pub fn render(&self) {
        draw_circle(self.x, self.y, self.radius, self.color);
    }
}

/// different directions of the [`Joystick`]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JoystickDirection {
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
    /// calculate a JoystickDirection from degrees
    ///
    /// 0 degrees are on the positive X-Axis and then it rotates clockwise
    ///
    /// # Examples
    /// ```
    /// use macroquad_virtual_joystick::JoystickDirection;
    ///
    /// let degrees = 153.5;
    /// let direction = JoystickDirection::from_degrees(degrees);
    ///
    /// assert_eq!(direction, JoystickDirection::DownLeft);
    /// ```
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

    /// convert the direction to a Vec2 with x and y
    ///
    /// x and y are both one of these: [-1.0, 0.0, 1.0]
    ///
    /// # Examples
    /// ```
    /// use macroquad::prelude::Vec2;
    /// use macroquad_virtual_joystick::JoystickDirection;
    ///
    /// let direction = JoystickDirection::Up;
    /// assert_eq!(direction.to_local(), Vec2::new(0.0, -1.0))
    /// ```
    pub fn to_local(&self) -> Vec2 {
        let (x, y) = match self {
            Self::Right => (1., 0.),
            Self::DownRight => (1., 1.),
            Self::Down => (0., 1.),
            Self::DownLeft => (-1., 1.),
            Self::Left => (-1., 0.),
            Self::UpLeft => (-1., -1.),
            Self::Up => (0., -1.),
            Self::UpRight => (1., -1.),
            Self::Idle => (0., 0.),
        };
        Vec2::new(x, y)
    }
}

/// the event of the [`Joystick`]
///
/// call [`Joystick::update`] to get the current event
#[derive(Clone, Copy, Debug)]
pub struct JoystickDirectionalEvent {
    /// the direction to which the knob was moved
    pub direction: JoystickDirection,

    /// the intensity of the knob move, from 0 (center) to 1 (edge)
    pub intensity: f32,

    /// the angle of the knob (in radians)
    ///
    /// starting on the positive x-axis and rotating counter-clockwise
    pub angle: f32,
}

impl JoystickDirectionalEvent {
    fn new(direction: JoystickDirection, intensity: f32, angle: f32) -> Self {
        Self {
            direction,
            intensity,
            angle,
        }
    }
}

impl Default for JoystickDirectionalEvent {
    fn default() -> Self {
        Self {
            direction: JoystickDirection::Idle,
            intensity: 0.,
            angle: 0.,
        }
    }
}
