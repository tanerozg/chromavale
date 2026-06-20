//! ChromaVale screen color engine.
//!
//! `Settings` describe the desired look; `apply` builds a gamma ramp and
//! pushes it to the display, `reset` restores a neutral ramp.

pub mod correction;
mod ramp;

use serde::Deserialize;

#[cfg(windows)]
#[path = "platform_windows.rs"]
mod platform;

#[cfg(target_os = "macos")]
#[path = "platform_macos.rs"]
mod platform;

#[cfg(not(any(windows, target_os = "macos")))]
#[path = "platform_other.rs"]
mod platform;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Settings {
    /// Color temperature in Kelvin (2500..=9300). 6500 is neutral.
    pub temperature: f64,
    /// Overall brightness multiplier (0.5..=1.2). 1.0 is neutral.
    pub brightness: f64,
    /// Red channel gain (0.5..=1.2). 1.0 is neutral.
    pub red: f64,
    /// Green channel gain (0.5..=1.2).
    pub green: f64,
    /// Blue channel gain (0.5..=1.2).
    pub blue: f64,
    /// Gamma exponent (0.6..=1.6). 1.0 is neutral.
    pub gamma: f64,
}

impl Settings {
    pub fn neutral() -> Self {
        Self {
            temperature: 6500.0,
            brightness: 1.0,
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            gamma: 1.0,
        }
    }

    /// Clamp every field to a safe range so a bad input can never produce a
    /// fully black or unusable screen.
    pub fn clamped(&self) -> Self {
        Self {
            temperature: self.temperature.clamp(2500.0, 9300.0),
            brightness: self.brightness.clamp(0.4, 1.2),
            red: self.red.clamp(0.4, 1.2),
            green: self.green.clamp(0.4, 1.2),
            blue: self.blue.clamp(0.4, 1.2),
            gamma: self.gamma.clamp(0.5, 1.8),
        }
    }
}

/// Apply the given settings to the display(s).
pub fn apply(settings: &Settings) -> Result<(), String> {
    let table = ramp::build(settings);
    platform::apply_ramp(&table)
}

/// Restore a neutral (identity) ramp.
pub fn reset() -> Result<(), String> {
    let table = ramp::build(&Settings::neutral());
    platform::apply_ramp(&table)
}
