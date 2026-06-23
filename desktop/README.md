# ChromaVale Desktop

The real ChromaVale desktop app for macOS and Windows, built with
[Tauri 2](https://tauri.app). The UI is Vue + Vite; the screen color engine is
Rust.

## What works today

- A real-time **color engine** that adjusts your entire screen via the OS gamma
  ramp: color temperature, brightness, gamma, and per-channel red/green/blue.
- **Comfort presets** (Comfort, Reading, Night) and a power toggle.
- **Gaming & vibe** looks (Vibrant, Neon, Cinematic, Cozy, Frost) plus a
  Vibrance control, to make games look bold and saturated. Kept separate from
  the accessibility color-blind filters.
- On Windows it uses `SetDeviceGammaRamp`; on macOS `CGSetDisplayTransferByTable`.
- **Screen filters (Windows):** real daltonization for protan, deutan and
  tritan with adjustable intensity, plus Grayscale, Grayscale inverted,
  Inverted and an independent Color Boost, applied to the whole screen via the
  Windows Magnification API (`MagSetFullscreenColorEffect`, the same mechanism
  Windows' own Color Filters use). Matches and exceeds the built-in panel.
- **Calibration test:** a short interactive test ("Find my filter") that finds
  the color-blindness type and severity using iso-luminant dot discrimination,
  refines protan vs deutan with a preview, and applies the matching correction
  automatically.
- **Global hotkey:** toggle ChromaVale on/off from anywhere with `Ctrl+Alt+C`.
- **System tray:** the app runs in the background; closing the window hides it
  to the tray (right-click for Show / Toggle / Quit) so the hotkey and filters
  keep working.
- **Remembers your setup:** all settings, the active filter and the on/off
  state are saved and reapplied on the next launch.
- **Start with Windows:** optional launch at login (starts hidden in the tray)
  so your screen is tuned the moment you log in.
- **Night schedule:** automatically warms the screen during a chosen time
  window (e.g. 21:00-07:00), running in the background.
- **Per-app profiles:** detects the foreground app and auto-applies a profile
  you assign to it (e.g. Neon for a game, Comfort for your editor), restoring
  your manual look when you switch away. Works in fullscreen games because it
  drives the global engine rather than an overlay.
- The screen is always restored to neutral when the app quits.

- **Pro licensing:** free features (color engine, comfort presets, gaming
  looks, grayscale/inverted) are always available; Pro features (color-blind
  calibration + correction, per-app profiles, night schedule, autostart) unlock
  with a license key, validated against chromavale.com and device-bound.

Color-blind correction on macOS is the next milestone.

## Tests

```bash
cd desktop/src-tauri
cargo test --lib                       # color engine + filter matrix math
cargo test --test hotkey_registration  # confirms Ctrl+Alt+C registers with the OS
```

The end-to-end hotkey test that synthesizes a real keystroke is `#[ignore]`d
because it needs an interactive desktop session. Run it yourself with:

```bash
cargo test --test hotkey_registration -- --ignored
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- Node.js 20+
- Platform tooling:
  - **Windows:** Microsoft C++ Build Tools (or Visual Studio) and the WebView2
    runtime (preinstalled on Windows 11).
  - **macOS:** Xcode command line tools.

## Develop

```bash
cd desktop
npm install
npm run tauri dev
```

This launches the app with hot reload. Move the sliders and your real screen
changes.

## Build a distributable

```bash
cd desktop
npm install
npm run tauri build
```

Output (installer and binary) lands in
`src-tauri/target/release/bundle/`:

- Windows: `.msi` and `.exe` (NSIS)
- macOS: `.dmg` and `.app`

### Icons

Icons are generated from a single source image:

```bash
npm run tauri icon ../public/icon-source.png
```

### A note on signing

To ship without security warnings you need code signing: an Apple Developer ID
plus notarization on macOS, and an Authenticode certificate on Windows.
Unsigned builds still run but show a Gatekeeper / SmartScreen prompt.

The **free** way to avoid the Windows SmartScreen warning is to ship through
the Microsoft Store as an MSIX (Microsoft re-signs it). See [STORE.md](STORE.md)
for the full packaging and submission workflow.

## A note on the gamma ramp

Global gamma-ramp control is how f.lux and Night Light work. Some laptops,
external displays, and HDR configurations restrict or ignore gamma changes; the
app surfaces an error when the driver blocks it.
