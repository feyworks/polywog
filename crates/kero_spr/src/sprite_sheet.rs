use crate::Sprite;
use kero::prelude::*;
use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};

/// A sheet of sprite tiles.
#[derive(Debug, Clone)]
pub struct SpriteSheet {
    pub tiles: VecGrid<Option<Sprite>>,
    pub tile_size: Vec2F,
}

impl SpriteSheet {
    /// Create a new sheet.
    #[inline]
    pub fn new(grid_size: impl Into<Vec2U>, tile_size: impl Into<Vec2F>) -> Self {
        Self {
            tiles: VecGrid::new_with(grid_size, || None),
            tile_size: tile_size.into(),
        }
    }

    /// Get a reference to the tile at the provided coord.
    #[inline]
    pub fn tile<P: Coord>(&self, pos: P) -> Option<&Sprite> {
        self.tiles.get_at(pos).map(Option::as_ref).flatten()
    }

    /// Get a mutable reference to the tile at the provided coord.
    #[inline]
    pub fn tile_mut<P: Coord>(&mut self, pos: P) -> Option<&mut Sprite> {
        self.tiles.get_mut_at(pos).map(Option::as_mut).flatten()
    }

    /// Set the tile at the provided coord.
    #[inline]
    pub fn set_tile<P: Coord>(&mut self, pos: P, tile: impl Into<Option<Sprite>>) {
        self.tiles.set_at(pos, tile.into());
    }

    /// Remove all tiles from the sheet, setting them to `None`.
    #[inline]
    pub fn clear_tiles(&mut self) {
        self.tiles.fill_with(|| None);
    }
}
