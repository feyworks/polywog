pub mod core;
pub mod gfx;
pub mod input;
mod new_game;

#[doc(inline)]
pub use fey_color as color;

#[doc(inline)]
pub use fey_grid as grid;

#[doc(inline)]
pub use fey_guid as guid;

#[doc(inline)]
pub use fey_img as img;

#[doc(inline)]
pub use fey_math as math;

#[doc(inline)]
pub use fey_rand as rand;

pub use new_game::new_game;

/// Include all types and traits.
pub mod prelude {
    pub use crate::color::*;
    pub use crate::core::*;
    pub use crate::gfx::*;
    pub use crate::grid::*;
    pub use crate::guid::*;
    pub use crate::img::*;
    pub use crate::input::*;
    pub use crate::math::*;
    pub use crate::rand::*;
}
