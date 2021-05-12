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
//!         joystick.render();
//!         next_frame().await
//!     }
//! }
//! ```
#![warn(missing_docs)]

use macroquad::prelude::{
    color_u8, draw_circle, is_mouse_button_down, mouse_position, touches, Color, MouseButton,
    TouchPhase, Vec2,
};

static BACKGROUND_COLOR: Color = color_u8!(96, 128, 144, 128);
static KNOB_COLOR: Color = color_u8!(96, 128, 144, 168);

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
    event: JoystickEvent,
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
        let background_fn = Box::new(|center_x: f32, center_y: f32, radius: f32| {
            draw_circle(center_x, center_y, radius, BACKGROUND_COLOR);
        });
        let background = JoystickElement::new(x, y, size / 2., background_fn);
        let knob_fn = Box::new(|center_x: f32, center_y: f32, radius: f32| {
            draw_circle(center_x, center_y, radius, KNOB_COLOR);
        });
        let knob = JoystickElement::new(x, y, size / 4., knob_fn);

        Self {
            center: Vec2::new(x, y),
            size,
            background,
            knob,
            dragging: false,
            touch_id: 0,
            event: JoystickEvent::default(),
        }
    }

    /// create a new [`Joystick`] with custom elements for background and knob
    ///
    /// # Arguments
    /// * `x`, `y`: center of the joystick
    /// * `size`: diameter of the joystick, should have the same size as the background element
    /// * `knob_size`: diameter of the knob, should have the same size as the background element
    /// * `render_background`, `render_knob`: custom drawing functions with the following
    ///  arguments:
    ///   * `x` the x coordinate of the center of the component
    ///   * `y` the y coordinate of the center of the component
    ///   * `radius` the radius used for mouse/ touch collision
    ///     for good UX this should also be the size of the drawing
    ///
    /// # Examples
    /// ```
    /// use macroquad::prelude::*;
    /// use macroquad_virtual_joystick::Joystick;
    ///
    /// fn render_background(x: f32, y: f32, radius: f32) {
    ///     draw_circle(x, y, radius, RED);
    /// }
    ///
    /// fn render_knob(x: f32, y: f32, radius: f32) {
    ///     draw_circle(x, y, radius, GREEN);
    /// }
    ///
    /// #[macroquad::main("Custom Joystick")]
    /// async fn main() {
    ///     const SPEED: f32 = 2.5;
    ///     let mut position = Vec2::new(screen_width() / 2.0, screen_height() / 4.0);
    ///
    ///     let background_size = 50.0;
    ///     let knob_size = 32.0;
    ///
    ///     let mut joystick = Joystick::from_custom_elements(
    ///         100.0,
    ///         200.0,
    ///         background_size,
    ///         knob_size,
    ///         Box::new(render_background),
    ///         Box::new(render_knob),
    ///     );
    ///     loop {
    ///         clear_background(WHITE);
    ///
    ///         let joystick_event = joystick.update();
    ///         position += joystick_event.direction.to_local() * joystick_event.intensity * SPEED;
    ///
    ///         draw_circle(position.x, position.y, 50.0, YELLOW);
    ///
    ///         joystick.render();
    ///         next_frame().await
    ///     }
    /// }
    /// ```
    pub fn from_custom_elements(
        x: f32,
        y: f32,
        size: f32,
        knob_size: f32,
        render_background: Box<fn(f32, f32, f32)>,
        render_knob: Box<fn(f32, f32, f32)>,
    ) -> Self {
        let center = Vec2::new(x, y);
        let background = JoystickElement::new(x, y, size / 2., render_background);
        let knob = JoystickElement::new(x, y, knob_size / 2., render_knob);

        Self {
            center,
            size,
            background,
            knob,
            dragging: false,
            touch_id: 0,
            event: JoystickEvent::default(),
        }
    }

    /// render the joystick
    ///
    /// renders the background and knob
    ///
    /// call [`macroquad::prelude::set_default_camera()`] before!
    pub fn render(&self) {
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
        self.event = JoystickEvent::default();
    }

    /// update the joystick
    ///
    /// this updates the joystick and returns the current [`JoystickEvent`]
    ///
    /// # Examples
    /// see [`Joystick`]
    pub fn update(&mut self) -> JoystickEvent {
        if touches().is_empty() {
            self.update_mouse();
        } else {
            self.update_touch();
        }
        self.event
    }

    /// move the knob according to the drag position and update the [`self.event`]
    fn moving(&mut self, position: Vec2) {
        let radius = self.size / 2.;
        let delta = position - self.center;
        let angle = delta.y.atan2(delta.x);
        let angle_degrees = angle.to_degrees();

        // maximum distance for the knob is the radius of the background
        let dist = f32::min(delta.length(), radius);

        self.knob.x = self.center.x + dist * angle.cos();
        self.knob.y = self.center.y + dist * angle.sin();

        let intensity = dist / radius;
        let direction = if intensity == 0. {
            JoystickDirection::Idle
        } else {
            JoystickDirection::from_degrees(angle_degrees as f64)
        };
        self.event = JoystickEvent::new(direction, intensity, angle);
    }
}

/// element of the [`Joystick`]
///
/// can be used for the background or the knob
struct JoystickElement {
    x: f32,
    y: f32,
    radius: f32,
    drawable: Box<dyn Fn(f32, f32, f32)>,
}

impl JoystickElement {
    fn new(x: f32, y: f32, radius: f32, drawable: Box<dyn Fn(f32, f32, f32)>) -> Self {
        Self {
            x,
            y,
            radius,
            drawable,
        }
    }

    /// render the element
    pub fn render(&self) {
        (self.drawable)(self.x, self.y, self.radius);
    }
}

#[allow(missing_docs)]
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
pub struct JoystickEvent {
    /// the direction to which the knob was moved
    pub direction: JoystickDirection,

    /// the intensity of the knob move, from 0 (center) to 1 (edge)
    pub intensity: f32,

    /// the angle of the knob (in radians)
    ///
    /// starting on the positive x-axis and rotating counter-clockwise
    pub angle: f32,
}

impl JoystickEvent {
    fn new(direction: JoystickDirection, intensity: f32, angle: f32) -> Self {
        Self {
            direction,
            intensity,
            angle,
        }
    }
}

impl Default for JoystickEvent {
    fn default() -> Self {
        Self {
            direction: JoystickDirection::Idle,
            intensity: 0.,
            angle: 0.,
        }
    }
}
