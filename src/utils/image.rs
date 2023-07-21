use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ImageType {
    PNG,
    JPEG,
    JPG,
    GIF,
    NONE,
}

impl ImageType {
    pub fn from_str(string: &str) -> Self {
        match string {
            "image/png" => ImageType::PNG,
            "image/jpg" => ImageType::JPG,
            "image/jpeg" => ImageType::JPEG,
            "image/gif" => ImageType::GIF,
            _ => ImageType::NONE,
        }
    }
}

impl std::fmt::Display for ImageType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ImageType::PNG => write!(formatter, "png"),
            ImageType::JPEG => write!(formatter, "jpeg"),
            ImageType::JPG => write!(formatter, "jpg"),
            ImageType::GIF => write!(formatter, "gif"),
            ImageType::NONE => write!(formatter, "wtf"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub file_type: ImageType,
    pub file_name: String,
    pub data: Vec<u8>,
}

impl Image {
    pub fn new() -> Self {
        Image {
            file_type: ImageType::NONE,
            file_name: String::new(),
            data: Default::default(),
        }
    }
}
