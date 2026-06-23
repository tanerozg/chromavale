mod engine;

use engine::filters::FilterKind;
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
fn apply_filter(kind: FilterKind, strength: f64, color_boost: f64) -> Result<(), String> {
    engine::filters::apply(kind, strength, color_boost)
}

#[tauri::command]
fn clear_filter() -> Result<(), String> {
    engine::filters::clear()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use tauri::Emitter;
    use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

    // Global hotkey to toggle ChromaVale on/off from anywhere: Ctrl+Alt+C.
    let toggle = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyC);
    let toggle_for_handler =
        Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyC);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if *shortcut == toggle_for_handler
                        && event.state() == ShortcutState::Pressed
                    {
                        let _ = app.emit("toggle-power", ());
                    }
                })
                .build(),
        )
        .setup(move |app| {
            app.global_shortcut().register(toggle)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            apply_color,
            reset_color,
            apply_filter,
            clear_filter
        ])
        .build(tauri::generate_context!())
        .expect("error while building ChromaVale")
        .run(|_app_handle, event| {
            // Always restore the screen when the app exits so we never leave
            // the display tinted or filtered.
            if let tauri::RunEvent::Exit = event {
                let _ = engine::reset();
                let _ = engine::filters::clear();
            }
        });
}
