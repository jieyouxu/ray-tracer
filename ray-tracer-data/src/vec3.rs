//! 3-component vector.

use std::convert::From;

/// A 3-component vector. Components are zero-indexed.
pub struct Vec3<N>(N, N, N);

impl<N> Vec3<N> {
    /// Constructs a new 3-component vector.
    pub const fn new(_0: N, _1: N, _2: N) -> Self {
        Self(_0, _1, _2)
    }

    /// Gets the first component of the vector.
    pub const fn _0(&self) -> &N {
        &self.0
    }
    /// Gets the second component of the vector.
    pub const fn _1(&self) -> &N {
        &self.1
    }
    /// Gets the third component of the vector.
    pub const fn _2(&self) -> &N {
        &self.2
    }
}

impl<N> From<(N, N, N)> for Vec3<N> {
    fn from(t: (N, N, N)) -> Self {
        Self(t.0, t.1, t.2)
    }
}
