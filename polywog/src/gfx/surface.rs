use crate::gfx::Texture;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

/// Handle to a surface that can be drawn to.
///
/// This handle can be cloned and passed around freely to give objects access to the surface.
///
/// Surfaces are created from [`Graphics`](super::Graphics).
#[derive(Clone)]
pub struct Surface(pub(crate) Texture);

impl Debug for Surface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Surface").finish_non_exhaustive()
    }
}

impl PartialEq for Surface {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Surface {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Surface {
    /// The surface's texture.
    #[inline]
    pub fn texture(&self) -> &Texture {
        &self.0
    }
}

impl Deref for Surface {
    type Target = Texture;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Texture> for Surface {
    #[inline]
    fn as_ref(&self) -> &Texture {
        &self.0
    }
}

impl Borrow<Texture> for Surface {
    #[inline]
    fn borrow(&self) -> &Texture {
        &self.0
    }
}
