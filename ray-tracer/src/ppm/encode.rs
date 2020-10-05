//! Encodes a supplied pixel buffer and encodes it in the proper PPM format and writes it out
//! using the provided writer.

use super::PpmHeader;

use std::io::Write;

/// Encoder responsible for encoding a provided pixel buffer into a plain-text PPM image format
/// and write it out using the supplied writer.
#[derive(Debug)]
pub struct PpmEncoder<W: Write> {
    writer: W,
    header: PpmHeader,
}
