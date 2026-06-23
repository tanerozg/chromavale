//! Full-screen color filters via the Windows Magnification API.
//!
//! This covers personal color-blind correction (daltonization) as well as the
//! grayscale / inverted modes Windows offers, plus an independent Color Boost
//! (saturation). Everything is composed into a single 5x5 color matrix that is
//! applied to the whole screen with `MagSetFullscreenColorEffect` (the same
//! mechanism Windows' own Color filters use).

use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum FilterKind {
    None,
    Protan,
    Deutan,
    Tritan,
    Grayscale,
    GrayscaleInverted,
    Inverted,
}

/// Luminance weights (Rec. 601), used for grayscale and saturation.
const LUM: [f64; 3] = [0.299, 0.587, 0.114];

/// A color transform: 3x3 matrix in `output = M * input` form plus a constant
/// translation added afterwards (needed for inversion).
struct Transform {
    m: [[f64; 3]; 3],
    t: [f64; 3],
}

const IDENTITY: [[f64; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

fn filter_transform(kind: FilterKind) -> Transform {
    let gray = [LUM, LUM, LUM];
    match kind {
        FilterKind::None => Transform { m: IDENTITY, t: [0.0; 3] },
        FilterKind::Protan => Transform {
            m: [[1.0, 0.0, 0.0], [-0.2549, 1.2549, 0.0], [0.3031, -0.5451, 1.242]],
            t: [0.0; 3],
        },
        FilterKind::Deutan => Transform {
            m: [[1.0, 0.0, 0.0], [-0.4375, 1.4375, 0.0], [0.2625, -0.5625, 1.3]],
            t: [0.0; 3],
        },
        FilterKind::Tritan => Transform {
            m: [[1.05, -0.3825, 0.3325], [0.0, 1.2345, -0.2345], [0.0, 0.0, 1.0]],
            t: [0.0; 3],
        },
        FilterKind::Grayscale => Transform { m: gray, t: [0.0; 3] },
        FilterKind::GrayscaleInverted => Transform {
            m: [
                [-LUM[0], -LUM[1], -LUM[2]],
                [-LUM[0], -LUM[1], -LUM[2]],
                [-LUM[0], -LUM[1], -LUM[2]],
            ],
            t: [1.0, 1.0, 1.0],
        },
        FilterKind::Inverted => Transform {
            m: [[-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, -1.0]],
            t: [1.0, 1.0, 1.0],
        },
    }
}

/// Saturation matrix for a boost factor (1.0 = unchanged, >1 = more vivid).
fn saturation_matrix(boost: f64) -> [[f64; 3]; 3] {
    let b = boost.clamp(0.0, 2.0);
    let mut s = [[0.0f64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let id = if i == j { 1.0 } else { 0.0 };
            s[i][j] = LUM[j] * (1.0 - b) + id * b;
        }
    }
    s
}

fn mat_mul(a: &[[f64; 3]; 3], b: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let mut out = [[0.0f64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            out[i][j] = (0..3).map(|k| a[i][k] * b[k][j]).sum();
        }
    }
    out
}

/// Build the 5x5 row-major matrix the Magnification API expects.
///
/// Pipeline: `input -> saturation -> filter (blended by strength) -> output`.
/// `strength` blends the filter toward identity (0..1); Color Boost is applied
/// independently so it works even with no color-blind filter selected.
pub fn transform(kind: FilterKind, strength: f64, color_boost: f64) -> [f32; 25] {
    let f = filter_transform(kind);
    let s = strength.clamp(0.0, 1.0);

    // Blend the filter toward identity by strength.
    let mut mb = [[0.0f64; 3]; 3];
    let mut tb = [0.0f64; 3];
    for i in 0..3 {
        for j in 0..3 {
            let id = if i == j { 1.0 } else { 0.0 };
            mb[i][j] = id * (1.0 - s) + f.m[i][j] * s;
        }
        tb[i] = f.t[i] * s;
    }

    // Apply Color Boost (saturation) before the filter.
    let sat = saturation_matrix(color_boost);
    let m_final = mat_mul(&mb, &sat);

    // Transpose into the 5x5 row-major Magnification matrix (result = in * T).
    let mut out = [0.0f32; 25];
    for i in 0..3 {
        for j in 0..3 {
            out[i * 5 + j] = m_final[j][i] as f32;
        }
    }
    out[3 * 5 + 3] = 1.0; // alpha passthrough
    for j in 0..3 {
        out[4 * 5 + j] = tb[j] as f32; // translation
    }
    out[4 * 5 + 4] = 1.0;
    out
}

#[cfg(windows)]
pub fn apply(kind: FilterKind, strength: f64, color_boost: f64) -> Result<(), String> {
    use windows::Win32::UI::Magnification::{
        MagInitialize, MagSetFullscreenColorEffect, MAGCOLOREFFECT,
    };

    let effect = MAGCOLOREFFECT {
        transform: transform(kind, strength, color_boost),
    };

    unsafe {
        if !MagInitialize().as_bool() {
            return Err("Could not initialize the Windows magnification engine.".into());
        }
        if !MagSetFullscreenColorEffect(&effect).as_bool() {
            return Err(
                "Could not apply the screen filter. Another app may be using the \
                 screen color filter."
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
        transform: transform(FilterKind::None, 0.0, 1.0),
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
pub fn apply(_kind: FilterKind, _strength: f64, _color_boost: f64) -> Result<(), String> {
    Err("Screen filters are currently available on Windows only.".into())
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
        let t = transform(FilterKind::None, 1.0, 1.0);
        for i in 0..5 {
            for j in 0..5 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((t[i * 5 + j] - expected).abs() < 1e-6, "[{i}][{j}]");
            }
        }
    }

    #[test]
    fn inverted_has_negative_diagonal_and_offset() {
        let t = transform(FilterKind::Inverted, 1.0, 1.0);
        assert!((t[0] + 1.0).abs() < 1e-6); // R->R = -1
        assert!((t[6] + 1.0).abs() < 1e-6); // G->G = -1
        assert!((t[12] + 1.0).abs() < 1e-6); // B->B = -1
        assert!((t[20] - 1.0).abs() < 1e-6); // translation R = +1
    }

    #[test]
    fn zero_strength_ignores_filter() {
        let t = transform(FilterKind::Deutan, 0.0, 1.0);
        // With no boost and zero strength, result is identity.
        assert!((t[0] - 1.0).abs() < 1e-6);
        assert!(t[5].abs() < 1e-6);
    }

    #[test]
    fn color_boost_increases_diagonal() {
        let t = transform(FilterKind::None, 0.0, 1.5);
        // Saturation > 1 pushes the diagonal above 1.
        assert!(t[0] > 1.0, "expected boosted red gain, got {}", t[0]);
    }
}
