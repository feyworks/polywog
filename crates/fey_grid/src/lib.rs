//! 2D grid data structure and related traits.
//!
//! This library provides two core traits, [`Grid`] and [`GridMut`], which represent immutable
//! and mutable grids respectively. Rather than just supplying a concrete 2D array type, this
//! approach allows all grid-based algorithms to be written generically, which lets the user
//! choose the actual implementation and storage method for their grids.

mod col;
mod col_iter;
mod cols_iter;
mod coord;
mod grid;
mod grid_buf;
mod grid_iter;
mod grid_mut;
mod row;
mod row_iter;
mod rows_iter;
mod view;

pub use col::*;
pub use col_iter::*;
pub use cols_iter::*;
pub use coord::*;
pub use grid::*;
pub use grid_buf::*;
pub use grid_iter::*;
pub use grid_mut::*;
pub use row::*;
pub use row_iter::*;
pub use rows_iter::*;
pub use view::*;
