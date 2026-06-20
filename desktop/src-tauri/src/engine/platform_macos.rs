//! macOS screen color application via CoreGraphics display transfer tables.

use core_graphics::display::CGDisplay;

#[allow(non_snake_case)]
extern "C" {
    fn CGSetDisplayTransferByTable(
        display: u32,
        table_size: u32,
        red: *const f32,
        green: *const f32,
        blue: *const f32,
    ) -> i32;
}

pub fn apply_ramp(ramp: &[[f32; 256]; 3]) -> Result<(), String> {
    let displays = CGDisplay::active_displays()
        .map_err(|e| format!("Could not list displays: {e:?}"))?;

    if displays.is_empty() {
        return Err("No active displays found.".into());
    }

    for display in displays {
        let result = unsafe {
            CGSetDisplayTransferByTable(
                display,
                256,
                ramp[0].as_ptr(),
                ramp[1].as_ptr(),
                ramp[2].as_ptr(),
            )
        };
        if result != 0 {
            return Err(format!(
                "Failed to apply color to display {display} (error {result})."
            ));
        }
    }

    Ok(())
}
