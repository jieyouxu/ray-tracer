//! Header for a PPM image format file.

use std::num::NonZeroU16;

/// A two-byte magic constant (in ASCII) which identifies the file type. For our purposes, we
/// use the plain-text PPM format which is the two ASCII characters "P3".
pub const PLAIN_TEXT_PPM_MAGIC_CONSTANT: &'static [u8] = b"P3";

/// Dimensions of a 2D image.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ImageDimensions {
    /// Width of the image in number of pixels.
    pub width: usize,
    /// Height of the image in number of pixels.
    pub height: usize,
}

/// Header metadata for a PPM file.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PpmHeader {
    /// Dimensions of the PPM image.
    dimensions: ImageDimensions,
    /// Maximum sample value, required to be `(0..u16::MAX)` by the [NetPbm][ppm] standard.
    maxval: NonZeroU16,
}

impl PpmHeader {
    /// Construct a new header for a PPM format.
    ///
    /// If `width` and `height` are `0`, then only the header will be written out. When using
    /// some PPM encoder to try to format a pixel buffer into the PPM file, the encoder may
    /// determine that the dimensions of the header does not conform with that of the pixel
    /// buffer and thus may error.
    pub fn new(dimensions: ImageDimensions, maxval: NonZeroU16) -> Self {
        assert!(
            maxval.get() < u16::MAX,
            "`maxval` must be within the range (0..u16::MAX), exclusive on both ends"
        );
        Self { dimensions, maxval }
    }

    /// Get the dimensions specified in the header.
    pub fn dimensions(&self) -> ImageDimensions {
        self.dimensions
    }

    /// Get the maximum sample value specified by the PPM header. The value is guaranteed to be
    /// within `(0..u16::MAX)` (excluding `0` and `u16::MAX`).
    pub fn maxval(&self) -> NonZeroU16 {
        self.maxval
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_constant() {
        assert_eq!(PLAIN_TEXT_PPM_MAGIC_CONSTANT, b"P3");
    }

    #[test]
    fn test_valid_header_construction() {
        let dimensions = ImageDimensions {
            width: 1,
            height: 2,
        };
        let maxval = NonZeroU16::new(u16::MAX - 1).unwrap();
        let header = PpmHeader::new(dimensions, maxval);
        assert_eq!(
            header.dimensions(),
            ImageDimensions {
                width: 1,
                height: 2
            }
        );
        assert_eq!(header.maxval().get(), u16::MAX - 1);
    }

    #[test]
    #[should_panic]
    fn test_maxval_too_large() {
        let dimensions = ImageDimensions {
            width: 1,
            height: 2,
        };
        let invalid_maxval = NonZeroU16::new(u16::MAX).unwrap();
        PpmHeader::new(dimensions, invalid_maxval);
    }
}
