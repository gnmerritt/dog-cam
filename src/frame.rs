use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

#[derive(Debug)]
pub enum Format {
    YUYV,
}

#[derive(Debug)]
pub struct Frame {
    pub height: i32,
    pub width: i32,
    pub format: Format,
    pub data: Vec<u8>,
    pub filename: Option<String>,
}

impl Frame {
    pub fn parse_resolution(filename: &str) -> Option<(i32, i32)> {
        let path = Path::new(filename);
        if let Some(filename) = path.file_stem() {
            let filename = filename.to_string_lossy();
            if let Some(hyphen_resolution) = filename.split(".").last() {
                let mut split = hyphen_resolution.split("-");
                let width = split.nth(0).map(|w| w.parse::<i32>());
                let height = split.nth(0).map(|h| h.parse::<i32>());
                if let (Some(Ok(width)), Some(Ok(height))) = (width, height) {
                    return Some((width, height));
                }
            }
        }
        None
    }

    pub fn from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data = fs::read(filename)?;
        if let Some((width, height)) = Self::parse_resolution(filename) {
            return Ok(Frame {
                height,
                width,
                format: Format::YUYV,
                data,
                filename: Some(filename.into()),
            });
        }
        Err(Box::new(Error::new(
            ErrorKind::Other,
            format!("Failed to load frame from {}", filename),
        )))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_filename_parsing() {
        assert_eq!(Frame::parse_resolution(""), None);
        assert_eq!(Frame::parse_resolution("some-random-filename.txt"), None);
        assert_eq!(Frame::parse_resolution("f.whoops-5.yuyv"), None);
        assert_eq!(Frame::parse_resolution("f.4-5.yuyv"), Some((4, 5)));
    }
}
