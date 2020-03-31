//! RGB data type.

use crate::vec3::Vec3;
use std::convert::Into;

/// RGB color (red, green, blue).
pub struct Rgb {
    inner: Vec3<u8>,
}

impl Rgb {
    /// Construct a new RGB color with three components, red, green and blue, in that order.
    #[inline]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Rgb {
            inner: Vec3::new(red, green, blue),
        }
    }

    /// Get the red component of the RGB color.
    #[inline]
    pub const fn red(&self) -> u8 {
        *self.inner._0()
    }

    /// Get the green component of the RGB color.
    #[inline]
    pub const fn green(&self) -> u8 {
        *self.inner._1()
    }

    /// Get the blue component of the RGB color.
    #[inline]
    pub const fn blue(&self) -> u8 {
        *self.inner._2()
    }
}

impl<N> From<(N, N, N)> for Rgb
where
    N: Into<u8>,
{
    /// Converts a 3-tuple `(n0, n1, n2)` into `(red: n0, green: n1, blue: n2)`; order here is
    /// significant.
    fn from(tuple: (N, N, N)) -> Self {
        Self::new(tuple.0.into(), tuple.1.into(), tuple.2.into())
    }
}

impl<N> From<&(N, N, N)> for Rgb
where
    N: Into<u8> + Copy,
{
    /// Converts a 3-tuple `(n0, n1, n2)` into `(red: n0, green: n1, blue: n2)`; order here is
    /// significant.
    fn from(tuple: &(N, N, N)) -> Self {
        Self::new(tuple.0.into(), tuple.1.into(), tuple.2.into())
    }
}

impl<N> From<Vec3<N>> for Rgb
where
    N: Into<u8> + Copy,
{
    /// Converts a `Vec3<N>` into RGB as long as `N` can be converted into a `u8`; order is
    /// significant.
    fn from(vec3: Vec3<N>) -> Self {
        Rgb {
            inner: vec3.map(Into::into),
        }
    }
}

impl Into<Vec3<u8>> for Rgb {
    fn into(self) -> Vec3<u8> {
        Vec3::new(self.red(), self.green(), self.blue())
    }
}

#[cfg(test)]
mod tests {
    use super::Rgb;
    use crate::vec3::Vec3;

    #[test]
    fn test_new_and_getters() {
        let rgb = Rgb::new(1, 2, 3);
        assert_eq!(1, rgb.red());
        assert_eq!(2, rgb.green());
        assert_eq!(3, rgb.blue());
    }

    #[test]
    fn test_from_3_tuple_by_value() {
        let t = (1, 2, 3);
        let rgb = Rgb::from(&t);
        assert_eq!(1, rgb.red());
        assert_eq!(2, rgb.green());
        assert_eq!(3, rgb.blue());
    }

    #[test]
    fn test_from_3_tuple_by_ref() {
        let t = (1, 2, 3);
        let rgb = Rgb::from(&t);
        assert_eq!(1, rgb.red());
        assert_eq!(2, rgb.green());
        assert_eq!(3, rgb.blue());
    }

    #[test]
    fn test_from_vec3() {
        let vec3 = Vec3::new(1, 2, 3);
        let rgb = Rgb::from(vec3);
        assert_eq!(1, rgb.red());
        assert_eq!(2, rgb.green());
        assert_eq!(3, rgb.blue());
    }

    #[test]
    fn test_into_vec3() {
        let rgb = Rgb::new(1, 2, 3);
        let vec3: Vec3<u8> = rgb.into();
        assert_eq!(1, *vec3._0());
        assert_eq!(2, *vec3._1());
        assert_eq!(3, *vec3._2());
    }
}
