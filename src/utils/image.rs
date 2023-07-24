use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ImageType {
    Png,
    Jpeg,
    Jpg,
    Gif,
}

impl ImageType {
    pub fn from_str(string: &str) -> Self {
        match string {
            "image/png" => ImageType::Png,
            "image/jpg" => ImageType::Jpg,
            "image/jpeg" => ImageType::Jpeg,
            "image/gif" => ImageType::Gif,
            _ => ImageType::Png,
        }
    }
}

impl std::fmt::Display for ImageType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ImageType::Png => write!(formatter, "png"),
            ImageType::Jpeg => write!(formatter, "jpeg"),
            ImageType::Jpg => write!(formatter, "jpg"),
            ImageType::Gif => write!(formatter, "gif"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Image {
    pub file_type: ImageType,
    pub file_name: String,
    pub data: Vec<u8>,
}

impl Image {
    pub fn new() -> Self {
        Image {
            file_type: ImageType::Png,
            file_name: String::new(),
            data: Default::default(),
        }
    }
}
