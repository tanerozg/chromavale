mod engine;

use engine::correction::CorrectionType;
use engine::Settings;

#[tauri::command]
fn apply_color(settings: Settings) -> Result<(), String> {
    engine::apply(&settings)
}

#[tauri::command]
fn reset_color() -> Result<(), String> {
    engine::reset()
}

#[tauri::command]
fn apply_correction(kind: CorrectionType, strength: f64) -> Result<(), String> {
    engine::correction::apply(kind, strength)
}

#[tauri::command]
fn clear_correction() -> Result<(), String> {
    engine::correction::clear()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            apply_color,
            reset_color,
            apply_correction,
            clear_correction
        ])
        .build(tauri::generate_context!())
        .expect("error while building ChromaVale")
        .run(|_app_handle, event| {
            // Always restore the screen when the app exits so we never leave
            // the display tinted or filtered.
            if let tauri::RunEvent::Exit = event {
                let _ = engine::reset();
                let _ = engine::correction::clear();
            }
        });
}
