extern crate chrono;
extern crate uuid;

use chrono::prelude::*;
use log::warn;
use uuid::Uuid;

/**
 * Get file extension by content-type string
 */
pub fn get_ext_by_type(cont_type: String) -> String {
  let mut t = "";

  match &cont_type[..] {
    "image/gif" => t = "gif",
    "image/jpeg" => t = "jpg",
    "image/pjpeg" => t = "jpeg",
    "image/png" => t = "png",
    "image/svg+xml" => t = "svg",
    "image/tiff" => t = "tif",
    "image/vnd.microsoft.icon" => t = "ico",
    "image/vnd.wap.wbmp" => t = "bmp",
    "image/webp" => t = "webp",
    _ => {
      warn!(
        "Unknow image mime type: {}. Will be returned empty string",
        cont_type
      );

      return String::from("");
    }
  }

  String::from(t)
}

/**
 * Generate unique file name
 */
pub fn generate_filename(ext: String) -> String {
  let uuid = Uuid::new_v4();
  let datetime = Utc::now().format("%Y-%m-%d_%H:%M:%S");

  format!("image_{}_{}.{}", uuid, datetime, ext)
}

/**
 * Get file content type from four first bytes
 */
pub fn get_ext_from_bytes(bytes: &[u8]) -> String {
  let mut t = "";

  if bytes == [255, 216, 255, 224] || bytes == [255, 216, 255, 225] {
    t = "image/jpeg";
  } else if bytes == [137, 80, 78, 71] {
    t = "image/png";
  } else if bytes == [66, 77, 138, 247] {
    t = "image/vnd.wap.wbmp";
  } else if bytes == [71, 73, 70, 56] {
    t = "image/gif";
  } else if bytes == [0, 0, 1, 0] {
    t = "image/vnd.microsoft.icon";
  } else if bytes == [73, 73, 42, 0] {
    t = "image/tiff";
  } else if bytes == [60, 115, 118, 103] {
    t = "image/svg+xml";
  }

  String::from(t)
}
