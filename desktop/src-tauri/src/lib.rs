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

fn show_main_window(app: &tauri::AppHandle) {
    use tauri::Manager;
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

/// Returns the executable file name (e.g. "game.exe") of the foreground window.
#[cfg(windows)]
fn foreground_exe() -> Option<String> {
    use windows::core::PWSTR;
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::Threading::{
        OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
        PROCESS_QUERY_LIMITED_INFORMATION,
    };
    use windows::Win32::UI::WindowsAndMessaging::{
        GetForegroundWindow, GetWindowThreadProcessId,
    };

    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0.is_null() {
            return None;
        }
        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if pid == 0 {
            return None;
        }
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;

        let mut buf = [0u16; 260];
        let mut size = buf.len() as u32;
        let ok = QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_WIN32,
            PWSTR(buf.as_mut_ptr()),
            &mut size,
        );
        let _ = CloseHandle(handle);
        if ok.is_err() {
            return None;
        }

        let path = String::from_utf16_lossy(&buf[..size as usize]);
        let name = path
            .rsplit(['\\', '/'])
            .next()
            .unwrap_or(&path)
            .to_string();
        Some(name)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
    use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
    use tauri::{Emitter, Manager};
    use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

    // Global hotkey to toggle ChromaVale on/off from anywhere: Ctrl+Alt+C.
    let toggle = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyC);
    let toggle_for_handler =
        Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyC);

    let global_shortcut = tauri_plugin_global_shortcut::Builder::new()
        .with_handler(move |app, shortcut, event| {
            if *shortcut == toggle_for_handler && event.state() == ShortcutState::Pressed {
                let _ = app.emit("toggle-power", ());
            }
        })
        .build();

    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(global_shortcut);

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ));
    }

    builder
        .setup(move |app| {
            app.global_shortcut().register(toggle)?;

            // System tray with a menu so ChromaVale can live in the background.
            let show_item =
                MenuItem::with_id(app, "show", "Show ChromaVale", true, None::<&str>)?;
            let toggle_item = MenuItem::with_id(
                app,
                "toggle",
                "Toggle on/off (Ctrl+Alt+C)",
                true,
                None::<&str>,
            )?;
            let separator = PredefinedMenuItem::separator(app)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu =
                Menu::with_items(app, &[&show_item, &toggle_item, &separator, &quit_item])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("ChromaVale")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => show_main_window(app),
                    "toggle" => {
                        let _ = app.emit("toggle-power", ());
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        show_main_window(tray.app_handle());
                    }
                })
                .build(app)?;

            // Closing the window hides it to the tray instead of quitting, so
            // the global hotkey and filters keep working in the background.
            if let Some(window) = app.get_webview_window("main") {
                let win = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = win.hide();
                    }
                });
            }

            // When launched at login (autostart passes --minimized), start
            // hidden in the tray instead of popping up a window.
            if std::env::args().any(|arg| arg == "--minimized") {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }

            // Watch the foreground app so the UI can auto-switch profiles.
            #[cfg(windows)]
            {
                let handle = app.handle().clone();
                std::thread::spawn(move || {
                    let mut last = String::new();
                    loop {
                        if let Some(name) = foreground_exe() {
                            if name != last {
                                last = name.clone();
                                let _ = handle.emit("foreground-app", name);
                            }
                        }
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                    }
                });
            }

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
