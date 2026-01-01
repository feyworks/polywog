use crate::core::Context;
use crate::input::virtual_source::VirtualSource;
use crate::input::{
    Gamepad, GamepadAxis, GamepadButton, Key, VirtualAxis, VirtualButton, VirtualStick,
};
use std::fmt::{Debug, Formatter};

/// A virtual gamepad controller.
///
/// This can be used to synthesize a game controller, allowing you to map keyboard keys into the
/// virtual inputs to listen to both/either the keyboard and gamepad at the same time.
#[derive(Clone)]
pub struct VirtualController {
    pub source: VirtualSource,
    pub direction: VirtualStick,
    pub left_stick: VirtualStick,
    pub right_stick: VirtualStick,
    pub left_bumper: VirtualButton,
    pub right_bumper: VirtualButton,
    pub left_trigger: VirtualButton,
    pub right_trigger: VirtualButton,
    pub dpad_left: VirtualButton,
    pub dpad_right: VirtualButton,
    pub dpad_up: VirtualButton,
    pub dpad_down: VirtualButton,
    pub east: VirtualButton,
    pub south: VirtualButton,
    pub west: VirtualButton,
    pub north: VirtualButton,
    pub start: VirtualButton,
    pub select: VirtualButton,
    pub menu: VirtualButton,
}

impl Debug for VirtualController {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("VirtualController").finish_non_exhaustive()
    }
}

impl VirtualController {
    /// Create a basic controller with some default mappings:
    ///
    /// - the arrow keys are mapped to the left stick and d-pad
    /// - the face buttons are mapped to <kbd>Z</kbd> (south), <kbd>X</kbd> (east), <kbd>A</kbd>
    /// (west), and <kbd>S</kbd> (north)
    /// - the start/select buttons are mapped to <kbd>Enter</kbd> and <kbd>Space</kbd>
    /// - the bumpers are mapped to <kbd>Q</kbd> and <kbd>W</kbd>
    /// - the triggers are mapped to left and right <kbd>Shift</kbd>
    pub fn basic(ctx: &Context) -> Self {
        let this = Self::last_active(ctx);
        this.set_left_stick_arrows();
        this.set_dpad_arrows();
        this.set_face_buttons_zxas();
        this.set_bumpers_qw();
        this.set_triggers_shift();
        this.start.set_key(Key::Enter);
        this.select.set_key(Key::Space);
        this
    }

    /// Create a controller with no mappings that always listens to the last active gamepad.
    pub fn last_active(ctx: &Context) -> Self {
        Self::from_source(VirtualSource::last_active(ctx))
    }

    /// Create a controller with no mappings that listens to a specific gamepad.
    pub fn specific(ctx: &Context, gamepad: Option<Gamepad>) -> Self {
        Self::from_source(VirtualSource::specific(ctx, gamepad))
    }

    /// Create a controller with no mappings that listens to the provided input source.
    pub fn from_source(source: VirtualSource) -> Self {
        let dpad_left = VirtualButton::new(&source, None, GamepadButton::DPadLeft);
        let dpad_right = VirtualButton::new(&source, None, GamepadButton::DPadRight);
        let dpad_up = VirtualButton::new(&source, None, GamepadButton::DPadUp);
        let dpad_down = VirtualButton::new(&source, None, GamepadButton::DPadDown);
        Self {
            direction: VirtualStick::new(
                &source,
                VirtualAxis::new(
                    &source,
                    GamepadAxis::LeftX,
                    dpad_left.clone(),
                    dpad_right.clone(),
                ),
                VirtualAxis::new(
                    &source,
                    GamepadAxis::LeftY,
                    dpad_up.clone(),
                    dpad_down.clone(),
                ),
            ),
            left_stick: VirtualStick::new(
                &source,
                VirtualAxis::new(&source, GamepadAxis::LeftX, None, None),
                VirtualAxis::new(&source, GamepadAxis::LeftY, None, None),
            ),
            right_stick: VirtualStick::new(
                &source,
                VirtualAxis::new(&source, GamepadAxis::RightX, None, None),
                VirtualAxis::new(&source, GamepadAxis::RightY, None, None),
            ),
            left_bumper: VirtualButton::new(&source, None, GamepadButton::LeftBumper),
            right_bumper: VirtualButton::new(&source, None, GamepadButton::RightBumper),
            left_trigger: VirtualButton::new(&source, None, GamepadButton::LeftTrigger),
            right_trigger: VirtualButton::new(&source, None, GamepadButton::RightTrigger),
            dpad_left,
            dpad_right,
            dpad_up,
            dpad_down,
            east: VirtualButton::new(&source, None, GamepadButton::East),
            south: VirtualButton::new(&source, None, GamepadButton::South),
            west: VirtualButton::new(&source, None, GamepadButton::West),
            north: VirtualButton::new(&source, None, GamepadButton::North),
            start: VirtualButton::new(&source, None, GamepadButton::Start),
            select: VirtualButton::new(&source, None, GamepadButton::Select),
            menu: VirtualButton::new(&source, None, GamepadButton::Menu),
            source,
        }
    }

    /// Map the bumpers to the left and right <kbd>Shift</kbd> buttons.
    pub fn set_bumpers_shift(&self) {
        self.left_bumper.set_key(Key::ShiftLeft);
        self.right_bumper.set_key(Key::ShiftRight);
    }

    /// Map the bumpers to <kbd>Q</kbd> and <kbd>W</kbd>.
    pub fn set_bumpers_qw(&self) {
        self.left_bumper.set_key(Key::Q);
        self.right_bumper.set_key(Key::W);
    }

    /// Map the triggers to the left and right <kbd>Shift</kbd> buttons.
    pub fn set_triggers_shift(&self) {
        self.left_trigger.set_key(Key::ShiftLeft);
        self.right_trigger.set_key(Key::ShiftRight);
    }

    /// Map the triggers to <kbd>Q</kbd> and <kbd>W</kbd>.
    pub fn set_triggers_qw(&self) {
        self.left_trigger.set_key(Key::Q);
        self.right_trigger.set_key(Key::W);
    }

    /// Map the face buttons to <kbd>Z</kbd> (south), <kbd>X</kbd> (east), <kbd>A</kbd> (west),
    /// and <kbd>S</kbd> (north).
    pub fn set_face_buttons_zxas(&self) {
        self.east.set_key(Key::X);
        self.south.set_key(Key::Z);
        self.west.set_key(Key::A);
        self.north.set_key(Key::S);
    }

    /// Map the left stick to the arrow keys.
    pub fn set_left_stick_arrows(&self) {
        self.left_stick.x_axis().neg().set_key(Key::ArrowLeft);
        self.left_stick.x_axis().pos().set_key(Key::ArrowRight);
        self.left_stick.y_axis().neg().set_key(Key::ArrowUp);
        self.left_stick.y_axis().pos().set_key(Key::ArrowDown);
    }

    /// Map the dpad to the arrow keys.
    pub fn set_dpad_arrows(&self) {
        self.dpad_left.set_key(Key::ArrowLeft);
        self.dpad_right.set_key(Key::ArrowRight);
        self.dpad_up.set_key(Key::ArrowUp);
        self.dpad_down.set_key(Key::ArrowDown);
    }
}
