pub mod manifest_generator;
pub mod manifest_controller;
pub mod dash;
pub mod hls;

pub enum ManifestType {
  HLS,
  MPEG_DASH
  // SMOOTH
}