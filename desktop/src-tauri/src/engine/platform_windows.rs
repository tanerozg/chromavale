//! Windows screen color application via the GDI gamma ramp.

use std::ffi::c_void;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{GetDC, ReleaseDC};
use windows::Win32::UI::ColorSystem::SetDeviceGammaRamp;

pub fn apply_ramp(ramp: &[[f32; 256]; 3]) -> Result<(), String> {
    // Windows expects three contiguous [u16; 256] tables (R, G, B).
    let mut gamma = [[0u16; 256]; 3];
    for c in 0..3 {
        for i in 0..256 {
            let v = (ramp[c][i] * 65535.0).round();
            gamma[c][i] = v.clamp(0.0, 65535.0) as u16;
        }
    }

    unsafe {
        // A null HWND yields a device context for the whole screen.
        let hdc = GetDC(HWND::default());
        if hdc.0.is_null() {
            return Err("Could not obtain the screen device context.".into());
        }

        let ok = SetDeviceGammaRamp(hdc, gamma.as_ptr() as *const c_void);
        ReleaseDC(HWND::default(), hdc);

        if !ok.as_bool() {
            return Err(
                "Your display or graphics driver blocked the gamma change. \
                 Some laptops and HDR configurations disable this."
                    .into(),
            );
        }
    }

    Ok(())
}
