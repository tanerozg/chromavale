//! Tests for the global hotkey our app relies on (Ctrl+Alt+C).
//!
//! These register the same combination through the `global-hotkey` crate (the
//! same library `tauri-plugin-global-shortcut` uses underneath).
//!
//! - `ctrl_alt_c_can_be_registered` always runs: it proves the OS accepts our
//!   hotkey (a valid, non-conflicting combination).
//! - `ctrl_alt_c_press_is_delivered` is `#[ignore]`d because it synthesizes a
//!   real keystroke with `SendInput`, which only works from an interactive
//!   desktop session (not headless CI/automation). Run it manually with:
//!   `cargo test --test hotkey_registration -- --ignored`

#![cfg(windows)]

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use std::time::{Duration, Instant};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
    VIRTUAL_KEY, VK_CONTROL, VK_MENU,
};

fn chromavale_hotkey() -> HotKey {
    HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyC)
}

#[test]
fn ctrl_alt_c_can_be_registered() {
    let manager = GlobalHotKeyManager::new().expect("create hotkey manager");
    let hotkey = chromavale_hotkey();

    // Succeeds only if the OS accepts the combination and it is not already
    // taken by another app: this is exactly what our app does at startup.
    manager
        .register(hotkey)
        .expect("Ctrl+Alt+C should register with the OS");

    manager
        .unregister(hotkey)
        .expect("Ctrl+Alt+C should unregister cleanly");
}

fn key(vk: VIRTUAL_KEY, up: bool) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: 0,
                dwFlags: if up {
                    KEYEVENTF_KEYUP
                } else {
                    KEYBD_EVENT_FLAGS(0)
                },
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

#[test]
#[ignore = "synthesizes a keystroke; requires an interactive desktop session"]
fn ctrl_alt_c_press_is_delivered() {
    let manager = GlobalHotKeyManager::new().expect("create hotkey manager");
    let hotkey = chromavale_hotkey();
    manager.register(hotkey).expect("register Ctrl+Alt+C");

    // The manager registers on its own message-loop thread; give it a moment.
    std::thread::sleep(Duration::from_millis(400));

    let rx = GlobalHotKeyEvent::receiver();
    while rx.try_recv().is_ok() {} // drain anything pending

    let vk_c = VIRTUAL_KEY(0x43); // 'C'
    let inputs = [
        key(VK_CONTROL, false),
        key(VK_MENU, false),
        key(vk_c, false),
        key(vk_c, true),
        key(VK_MENU, true),
        key(VK_CONTROL, true),
    ];
    let sent = unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) };
    assert_eq!(sent as usize, inputs.len(), "SendInput injected the keystroke");

    let deadline = Instant::now() + Duration::from_secs(3);
    let mut delivered = false;
    while Instant::now() < deadline {
        if let Ok(event) = rx.try_recv() {
            if event.id == hotkey.id() && event.state == HotKeyState::Pressed {
                delivered = true;
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(20));
    }

    let _ = manager.unregister(hotkey);
    assert!(delivered, "Ctrl+Alt+C was registered but no event was delivered");
}
