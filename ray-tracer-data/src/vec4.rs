/// A 4-component vector.
pub struct Vec4<N>(N, N, N, N);

impl<N> Vec4<N> {
    /// Constructs a new 4-component vector.
    pub fn new(_0: N, _1: N, _2: N, _3: N) -> Self {
        Self(_0, _1, _2, _3)
    }

    /// Gets the first component of the vector.
    pub fn _0(&self) -> &N {
        &self.0
    }
    /// Gets the second component of the vector.
    pub fn _1(&self) -> &N {
        &self.1
    }
    /// Gets the third component of the vector.
    pub fn _2(&self) -> &N {
        &self.2
    }
    /// Gets the fourth component of the vector.
    pub fn _3(&self) -> &N {
        &self.3
    }
}

impl<N> From<(N, N, N, N)> for Vec4<N> {
    fn from(t: (N, N, N, N)) -> Self {
        Self(t.0, t.1, t.2, t.3)
    }
}
