# ChromaVale Desktop

The real ChromaVale desktop app for macOS and Windows, built with
[Tauri 2](https://tauri.app). The UI is Vue + Vite; the screen color engine is
Rust.

## What works today

- A real-time **color engine** that adjusts your entire screen via the OS gamma
  ramp: color temperature, brightness, gamma, and per-channel red/green/blue.
- **Presets** (Comfort, Reading, Night, Vivid) and a power toggle.
- On Windows it uses `SetDeviceGammaRamp`; on macOS `CGSetDisplayTransferByTable`.
- **Color-blind correction (Windows):** real daltonization for protan, deutan
  and tritan with an adjustable strength, applied to the whole screen via the
  Windows Magnification API (`MagSetFullscreenColorEffect`, the same mechanism
  Windows' own Color Filters use). This is the paid killer feature.
- The screen is always restored to neutral when the app quits.

Color-blind correction on macOS and per-app filtering are the next milestones.

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
