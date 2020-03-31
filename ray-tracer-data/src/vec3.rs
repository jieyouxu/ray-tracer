//! 3-component vector.

use std::convert::From;

/// A 3-component vector. Components are zero-indexed.
pub struct Vec3<N>(N, N, N);

impl<N> Vec3<N> {
    /// Constructs a new 3-component vector.
    #[inline]
    pub const fn new(_0: N, _1: N, _2: N) -> Self {
        Self(_0, _1, _2)
    }

    /// Gets the first component of the vector.
    #[inline]
    pub const fn _0(&self) -> &N {
        &self.0
    }

    /// Gets the second component of the vector.
    #[inline]
    pub const fn _1(&self) -> &N {
        &self.1
    }

    /// Gets the third component of the vector.
    #[inline]
    pub const fn _2(&self) -> &N {
        &self.2
    }
}

impl<N> From<(N, N, N)> for Vec3<N> {
    fn from(t: (N, N, N)) -> Self {
        Self(t.0, t.1, t.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_getters() {
        let vec3 = Vec3::new(1, 2, 3);
        assert_eq!(1, *vec3._0());
        assert_eq!(2, *vec3._1());
        assert_eq!(3, *vec3._2());
    }

    #[test]
    fn test_from_3_tuple() {
        let t = (1, 2, 3);
        let vec3 = Vec3::from(t);
        assert_eq!(1, *vec3._0());
        assert_eq!(2, *vec3._1());
        assert_eq!(3, *vec3._2());
    }
}
