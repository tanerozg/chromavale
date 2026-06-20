//! Pure color math: turns engine settings into a normalized gamma ramp.
//! Output is three channels of 256 values in the range 0.0..=1.0. Platform
//! layers convert this to the format their display API expects.

use super::Settings;

/// Approximate RGB white point for a color temperature in Kelvin.
/// Based on the Tanner Helland approximation. Each component is 0.0..=1.0.
fn kelvin_to_rgb(kelvin: f64) -> (f64, f64, f64) {
    let temp = kelvin / 100.0;

    let red = if temp <= 66.0 {
        255.0
    } else {
        329.698_727_446 * (temp - 60.0).powf(-0.133_204_759_2)
    };

    let green = if temp <= 66.0 {
        99.470_802_586_1 * temp.ln() - 161.119_568_166_1
    } else {
        288.122_169_528_3 * (temp - 60.0).powf(-0.075_514_849_2)
    };

    let blue = if temp >= 66.0 {
        255.0
    } else if temp <= 19.0 {
        0.0
    } else {
        138.517_731_223_1 * (temp - 10.0).ln() - 305.044_792_730_7
    };

    (
        red.clamp(0.0, 255.0) / 255.0,
        green.clamp(0.0, 255.0) / 255.0,
        blue.clamp(0.0, 255.0) / 255.0,
    )
}

/// White-balance multipliers normalized so that 6500 K is neutral (1, 1, 1).
fn white_balance(kelvin: f64) -> (f64, f64, f64) {
    let (r, g, b) = kelvin_to_rgb(kelvin);
    let (nr, ng, nb) = kelvin_to_rgb(6500.0);
    (
        if nr > 0.0 { r / nr } else { 1.0 },
        if ng > 0.0 { g / ng } else { 1.0 },
        if nb > 0.0 { b / nb } else { 1.0 },
    )
}

/// Build the normalized 3 x 256 gamma ramp for the given settings.
pub fn build(settings: &Settings) -> [[f32; 256]; 3] {
    let s = settings.clamped();
    let (wr, wg, wb) = white_balance(s.temperature);

    let mult = [
        wr * s.red * s.brightness,
        wg * s.green * s.brightness,
        wb * s.blue * s.brightness,
    ];

    let mut ramp = [[0.0f32; 256]; 3];
    for (c, channel) in ramp.iter_mut().enumerate() {
        for (i, slot) in channel.iter_mut().enumerate() {
            let x = i as f64 / 255.0;
            // gamma > 1 darkens midtones (more contrast), < 1 lifts them.
            let v = x.powf(s.gamma) * mult[c];
            *slot = v.clamp(0.0, 1.0) as f32;
        }
    }
    ramp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neutral_settings_are_close_to_identity() {
        let ramp = build(&Settings::neutral());
        // Endpoints map to themselves.
        assert!((ramp[0][0] - 0.0).abs() < 1e-3);
        assert!((ramp[0][255] - 1.0).abs() < 1e-3);
        // Midpoint stays near the middle for all channels.
        for c in 0..3 {
            assert!((ramp[c][128] - 0.5).abs() < 0.05, "channel {c} midpoint");
        }
    }

    #[test]
    fn warm_reduces_blue_relative_to_red() {
        let mut s = Settings::neutral();
        s.temperature = 3000.0;
        let ramp = build(&s);
        assert!(ramp[2][255] < ramp[0][255], "blue should be cut when warm");
    }
}
