//! Image generation module for K230 platform.
//!
//! This module provides functionality to generate properly formatted and padded
//! images for the K230 platform. It handles initial offset and alignment requirements.

use crate::error::XtaskResult;

/// Generate an image with a specific format.
pub fn gen_image(data: &[u8]) -> XtaskResult<Vec<u8>> {
    // Create a vector with 1MB of zeros as the initial offset
    let mut image = vec![0; 0x100000];

    // Append the input data to the image
    image.extend_from_slice(data);

    // Pad the image to ensure its size is a multiple of 512 bytes
    if image.len() % 512 != 0 {
        let padding_size = 512 - image.len() % 512;
        image.extend(vec![0; padding_size]);
    }

    Ok(image)
}
