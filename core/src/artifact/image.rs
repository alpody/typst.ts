use serde::Deserialize;
use serde::Serialize;
use serde_with::base64::Base64;
use serde_with::serde_as;
pub use typst::image::Image as TypstImage;
pub use typst::image::ImageFormat;
pub use typst::image::RasterFormat;
pub use typst::image::VectorFormat;

/// A raster or vector image.
///
/// Values of this type are cheap to clone and hash.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Image {
    /// The raw, undecoded image data, follow btoa (standard) encoding
    #[serde_as(as = "Base64")]
    pub data: Vec<u8>,
    /// The format of the encoded `buffer`.
    pub format: String,
    /// The width in pixels.
    pub width: u32,
    /// The height in pixels.
    pub height: u32,
    /// A text describing the image.
    pub alt: Option<String>,
}

impl Image {
    pub fn decode_data(b: &String) -> Result<Vec<u8>, String> {
        base64::decode(b).map_err(|e| e.to_string())
    }
}
