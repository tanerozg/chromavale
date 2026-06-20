//! Color-blind correction (daltonization).
//!
//! Unlike the gamma ramp, daltonization mixes color channels, so it needs a
//! full color matrix applied to the composed screen. On Windows that is the
//! Magnification API's fullscreen color effect (the same mechanism Windows'
//! built-in Color Filters use).

use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CorrectionType {
    None,
    Protan,
    Deutan,
    Tritan,
}

/// Daltonization matrices in `output = M * input` form (column vector).
/// They redistribute the contrast a viewer cannot see into channels they can.
fn matrix(kind: CorrectionType) -> [[f64; 3]; 3] {
    match kind {
        CorrectionType::None => [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        CorrectionType::Protan => [
            [1.0, 0.0, 0.0],
            [-0.2549, 1.2549, 0.0],
            [0.3031, -0.5451, 1.242],
        ],
        CorrectionType::Deutan => [
            [1.0, 0.0, 0.0],
            [-0.4375, 1.4375, 0.0],
            [0.2625, -0.5625, 1.3],
        ],
        CorrectionType::Tritan => [
            [1.05, -0.3825, 0.3325],
            [0.0, 1.2345, -0.2345],
            [0.0, 0.0, 1.0],
        ],
    }
}

/// Build the 5x5 row-major color matrix the Magnification API expects.
///
/// Windows applies `result_row = input_row * T`, so the matrix is the
/// transpose of the `output = M * input` form, with identity rows for alpha
/// and the translation term. `strength` blends between identity (0) and the
/// full correction (1).
pub fn transform(kind: CorrectionType, strength: f64) -> [f32; 25] {
    let m = matrix(kind);
    let s = strength.clamp(0.0, 1.0);

    let mut blended = [[0.0f64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let id = if i == j { 1.0 } else { 0.0 };
            blended[i][j] = id * (1.0 - s) + m[i][j] * s;
        }
    }

    let mut t = [0.0f32; 25];
    t[18] = 1.0; // alpha passthrough  (row 3, col 3)
    t[24] = 1.0; // translation term   (row 4, col 4)
    for i in 0..3 {
        for j in 0..3 {
            // T[i][j] = M[j][i]
            t[i * 5 + j] = blended[j][i] as f32;
        }
    }
    t
}

#[cfg(windows)]
pub fn apply(kind: CorrectionType, strength: f64) -> Result<(), String> {
    use windows::Win32::UI::Magnification::{
        MagInitialize, MagSetFullscreenColorEffect, MAGCOLOREFFECT,
    };

    let effect = MAGCOLOREFFECT {
        transform: transform(kind, strength),
    };

    unsafe {
        if !MagInitialize().as_bool() {
            return Err("Could not initialize the Windows magnification engine.".into());
        }
        if !MagSetFullscreenColorEffect(&effect).as_bool() {
            return Err(
                "Could not apply the color-blind correction. Another app may be \
                 using the screen color filter."
                    .into(),
            );
        }
    }
    Ok(())
}

#[cfg(windows)]
pub fn clear() -> Result<(), String> {
    use windows::Win32::UI::Magnification::{
        MagInitialize, MagSetFullscreenColorEffect, MAGCOLOREFFECT,
    };

    let identity = MAGCOLOREFFECT {
        transform: transform(CorrectionType::None, 0.0),
    };

    unsafe {
        if !MagInitialize().as_bool() {
            return Ok(());
        }
        let _ = MagSetFullscreenColorEffect(&identity);
    }
    Ok(())
}

#[cfg(not(windows))]
pub fn apply(_kind: CorrectionType, _strength: f64) -> Result<(), String> {
    Err("Color-blind correction is currently available on Windows only.".into())
}

#[cfg(not(windows))]
pub fn clear() -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_is_identity() {
        let t = transform(CorrectionType::None, 1.0);
        // Diagonal of the 5x5 is 1, everything else 0.
        for i in 0..5 {
            for j in 0..5 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((t[i * 5 + j] - expected).abs() < 1e-6);
            }
        }
    }

    #[test]
    fn zero_strength_is_identity() {
        let t = transform(CorrectionType::Deutan, 0.0);
        assert!((t[0] - 1.0).abs() < 1e-6);
        assert!((t[6] - 1.0).abs() < 1e-6);
        assert!((t[12] - 1.0).abs() < 1e-6);
        // Off-diagonal stays zero at zero strength.
        assert!(t[5].abs() < 1e-6);
    }
}
