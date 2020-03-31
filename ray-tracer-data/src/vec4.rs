/// A 4-component vector.
pub struct Vec4<N>(N, N, N, N);

impl<N> Vec4<N> {
    /// Constructs a new 4-component vector.
    pub const fn new(_0: N, _1: N, _2: N, _3: N) -> Self {
        Self(_0, _1, _2, _3)
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
    /// Gets the fourth component of the vector.
    pub const fn _3(&self) -> &N {
        &self.3
    }
}

impl<N> From<(N, N, N, N)> for Vec4<N> {
    fn from(t: (N, N, N, N)) -> Self {
        Self(t.0, t.1, t.2, t.3)
    }
}

impl<N: Copy> From<&(N, N, N, N)> for Vec4<N> {
    fn from(t: &(N, N, N, N)) -> Self {
        Self(t.0, t.1, t.2, t.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_getters() {
        let vec4 = Vec4::new(1, 2, 3, 4);
        assert_eq!(1, *vec4._0());
        assert_eq!(2, *vec4._1());
        assert_eq!(3, *vec4._2());
        assert_eq!(4, *vec4._3());
    }

    #[test]
    fn test_from_4_tuple_by_value() {
        let t = (1, 2, 3, 4);
        let vec4 = Vec4::from(t);
        assert_eq!(1, *vec4._0());
        assert_eq!(2, *vec4._1());
        assert_eq!(3, *vec4._2());
        assert_eq!(4, *vec4._3());
    }

    #[test]
    fn test_from_4_tuple_by_ref() {
        let t = (1, 2, 3, 4);
        let vec4 = Vec4::from(&t);
        assert_eq!(1, *vec4._0());
        assert_eq!(2, *vec4._1());
        assert_eq!(3, *vec4._2());
        assert_eq!(4, *vec4._3());
    }

    #[test]
    fn test_map() {
        let vec4 = Vec4::new(1, 2, 3, 4);
        let inc_vec4 = vec4.map(|x| x + 1);
        assert_eq!(2, *inc_vec4._0());
        assert_eq!(3, *inc_vec4._1());
        assert_eq!(4, *inc_vec4._2());
        assert_eq!(5, *inc_vec4._3());
    }
}
