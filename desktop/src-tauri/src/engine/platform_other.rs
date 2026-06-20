//! Fallback for unsupported platforms: report that color control is
//! unavailable rather than failing to compile.

pub fn apply_ramp(_ramp: &[[f32; 256]; 3]) -> Result<(), String> {
    Err("Screen color control is not supported on this platform yet.".into())
}
