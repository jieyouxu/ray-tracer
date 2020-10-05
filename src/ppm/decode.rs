//! Decodes a supplied plain-text PPM file into a `PpmImage`, including the pixel buffer and
//! metadata.

use super::PpmHeader;

use std::io::Read;

/// Decoder responsible for decoding a plain-text PPM image format via the supplied Reader.
#[derive(Debug)]
pub struct PpmDecoder<R: Read> {
    reader: R,
    header: PpmHeader,
}
