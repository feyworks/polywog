use crate::input::Key;
use compact_str::CompactString;
use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::time::SystemTime;
use strum::{EnumCount, VariantArray};
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::PhysicalKey;

/// Handle to the keyboard state.
///
/// This handle can be cloned and passed around freely to give objects access to the keyboard.
#[derive(Clone)]
pub struct Keyboard(Rc<State>);

impl Debug for Keyboard {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Keyboard").finish_non_exhaustive()
    }
}

#[derive(Clone)]
struct State {
    down: Cell<[bool; Key::COUNT]>,
    phases: [Phase; 2],
    phase: Cell<usize>,
    last_active: Cell<SystemTime>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            down: Cell::new([false; _]),
            phases: std::array::repeat(Phase {
                pressed: Cell::new([false; _]),
                released: Cell::new([false; _]),
                repeated: Cell::new([false; _]),
                text_input: Cell::new(CompactString::default()),
            }),
            phase: Cell::new(0),
            last_active: Cell::new(SystemTime::now()),
        }
    }
}

struct Phase {
    pub pressed: Cell<[bool; Key::COUNT]>,
    pub released: Cell<[bool; Key::COUNT]>,
    pub repeated: Cell<[bool; Key::COUNT]>,
    pub text_input: Cell<CompactString>,
}

impl Clone for Phase {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            pressed: self.pressed.clone(),
            released: self.released.clone(),
            repeated: self.repeated.clone(),
            text_input: {
                let text = self.text_input.take();
                self.text_input.set(text.clone());
                Cell::new(text)
            },
        }
    }
}

impl Keyboard {
    pub(crate) fn new() -> Self {
        Self(Rc::new(State::default()))
    }

    /// Time the state was last updated.
    #[inline]
    pub fn last_active(&self) -> SystemTime {
        self.0.last_active.get()
    }

    #[inline]
    fn phase(&self) -> &Phase {
        &self.0.phases[self.0.phase.get()]
    }

    /// If the key is down.
    #[inline]
    pub fn down(&self, key: Key) -> bool {
        Cell::as_array_of_cells(&self.0.down)[key as usize].get()
    }

    /// Value of a key (`0.0` when up, `1.0` when down).
    #[inline]
    pub fn value(&self, key: Key) -> f32 {
        self.down(key).then_some(1.0).unwrap_or(0.0)
    }

    /// If the key was pressed this frame.
    #[inline]
    pub fn pressed(&self, key: Key) -> bool {
        Cell::as_array_of_cells(&self.phase().pressed)[key as usize].get()
    }

    /// If the key was released this frame.
    #[inline]
    pub fn released(&self, key: Key) -> bool {
        Cell::as_array_of_cells(&self.phase().released)[key as usize].get()
    }

    /// If the key was repeated this frame.
    #[inline]
    pub fn repeated(&self, key: Key) -> bool {
        Cell::as_array_of_cells(&self.phase().repeated)[key as usize].get()
    }

    /// If the key was pressed or repeated this frame.
    #[inline]
    pub fn pressed_or_repeated(&self, key: Key) -> bool {
        self.pressed(key) || self.repeated(key)
    }

    /// Text input that occurred this frame.
    #[inline]
    pub fn text_input(&self) -> CompactString {
        let phase = self.phase();
        let text = phase.text_input.take();
        phase.text_input.set(text.clone());
        text
    }

    /// All keys that are currently down.
    #[inline]
    pub fn currently_down(&self) -> impl Iterator<Item = Key> {
        Key::VARIANTS.iter().copied().filter(|key| self.down(*key))
    }

    /// If the left or right control key is down.
    #[inline]
    pub fn ctrl(&self) -> bool {
        self.down(Key::ControlLeft) || self.down(Key::ControlRight)
    }

    /// If the left or right shift key is down.
    #[inline]
    pub fn shift(&self) -> bool {
        self.down(Key::ShiftLeft) || self.down(Key::ShiftRight)
    }

    /// If the left or right alt key is down.
    #[inline]
    pub fn alt(&self) -> bool {
        self.down(Key::AltLeft) || self.down(Key::AltRight)
    }

    /// If the left or right command key is down.
    #[inline]
    pub fn cmd(&self) -> bool {
        self.down(Key::SuperLeft) || self.down(Key::SuperRight)
    }

    /// If a control key is down (Windows/Linux) or the command key is down (macOS).
    #[inline]
    pub fn ctrl_or_cmd(&self) -> bool {
        if cfg!(target_os = "macos") {
            self.cmd()
        } else {
            self.ctrl()
        }
    }

    #[inline]
    pub(crate) fn set_update_phase(&self) {
        self.0.phase.set(0);
    }

    #[inline]
    pub(crate) fn set_render_phase(&self) {
        self.0.phase.set(1);
    }

    #[inline]
    pub(crate) fn handle_event(&self, event: KeyEvent) {
        self.0.last_active.set(SystemTime::now());

        if let Some(txt) = event.text {
            for phase in &self.0.phases {
                let mut dst = phase.text_input.take();
                dst.push_str(&txt);
                phase.text_input.set(dst);
            }
        }
        match event.state {
            ElementState::Pressed => {
                if let PhysicalKey::Code(key) = event.physical_key {
                    if let Ok(key) = Key::try_from(key) {
                        let key = key as usize;
                        Cell::as_array_of_cells(&self.0.down)[key].set(true);
                        for phase in &self.0.phases {
                            if event.repeat {
                                Cell::as_array_of_cells(&phase.repeated)[key].set(true);
                            } else {
                                Cell::as_array_of_cells(&phase.pressed)[key].set(true);
                            }
                        }
                    }
                }
            }
            ElementState::Released => {
                if let PhysicalKey::Code(key) = event.physical_key {
                    if let Ok(key) = Key::try_from(key) {
                        let key = key as usize;
                        Cell::as_array_of_cells(&self.0.down)[key].set(false);
                        for phase in &self.0.phases {
                            Cell::as_array_of_cells(&phase.released)[key].set(true);
                        }
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn clear_phase(&self) {
        let phase = self.phase();
        phase.pressed.set([false; _]);
        phase.released.set([false; _]);
        phase.repeated.set([false; _]);
        phase.text_input.set(CompactString::default());
    }
}
