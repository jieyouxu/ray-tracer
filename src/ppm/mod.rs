//! The [**plain-text PPM image format**][ppm] (lowest common denominator for a color image format)
//! encoding and decoding module.
//!
//! The plain-text PPM image format should adhere to [the standard defined by NetPbm][ppm].
//!
//! [ppm]: http://netpbm.sourceforge.net/doc/ppm.html

pub mod decode;
pub mod encode;
pub mod header;

pub use decode::PpmDecoder;
pub use encode::PpmEncoder;
pub use header::PpmHeader;
