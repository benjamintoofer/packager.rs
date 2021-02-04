
pub trait ManifestGenerator {
    fn generate<'a>(mp4_frag: &[u8], timescale: u32, last_init_seg_byte: usize, asset_duration_sec: f64) -> &'a str;
}
