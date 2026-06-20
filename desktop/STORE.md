# Publishing ChromaVale to the Microsoft Store

The Microsoft Store is the route that removes the "Windows protected your PC"
(SmartScreen) warning **for free**: you submit an MSIX package and Microsoft
re-signs it. No paid code-signing certificate is required.

Tauri does not output MSIX directly, so we use Microsoft's official
[`winapp` CLI](https://learn.microsoft.com/en-us/windows/apps/dev-tools/winapp-cli/guides/tauri)
to wrap the built `chromavale.exe` into an MSIX. The npm scripts for this are
already in `package.json` (`msix:dev`, `msix:pack`).

## One-time setup

1. **Install the winapp CLI** (and ensure Rust + Node are present):

   ```powershell
   winget install microsoft.winappcli --source winget
   ```

2. **Reserve the app name** in
   [Partner Center](https://partner.microsoft.com/dashboard) (Windows & Xbox →
   create a new app). This gives you the values you must use for package
   identity:
   - **Package/Identity Name** (e.g. `1234Publisher.ChromaVale`)
   - **Publisher** (e.g. `CN=ABCD1234-...`)
   - **Publisher display name**

   A Partner Center developer account is required (a small one-time
   registration fee may apply).

3. **Generate the manifest and assets** from the `desktop/` folder:

   ```powershell
   winapp init
   ```

   This creates `Package.appxmanifest` and an `Assets` folder. Open the
   manifest and set the **Identity Name**, **Publisher**, and **Publisher
   display name** to exactly the values from Partner Center. Set the entry
   point to `chromavale.exe`.

   You can replace the generated logos with the branded ones already produced
   in `src-tauri/icons/` (`Square150x150Logo.png`, `Square44x44Logo.png`,
   `StoreLogo.png`, etc.).

## Build and test locally

```powershell
# From desktop/
winapp cert generate --if-exists skip   # self-signed dev cert (matches manifest Publisher)
npm run msix:pack                        # builds release exe and packs the MSIX
winapp cert install .\devcert.pfx        # trust the dev cert (run as admin, once)
Add-AppxPackage .\ChromaVale_*.msix      # install and launch from Start menu
```

> If you previously ran `npm run msix:dev`, the package may already be
> registered. Run `winapp unregister` first, then install.

> When repackaging after changes, bump the `Version` in
> `Package.appxmanifest` (Windows requires a higher version to update).

## Submit to the Store

1. In Partner Center, create a submission for the reserved app.
2. Upload the `.msix` (build one per architecture you support, e.g. x64 and
   Arm64).
3. Submit. **Microsoft signs the package for you**, so end users install it
   with no SmartScreen warning.

## Notes

- ChromaVale needs the WebView2 runtime; it is present on Windows 11 and
  available as the Evergreen runtime on Windows 10. Declare it as a dependency
  if WACK (Windows App Certification Kit) flags it.
- The gamma-ramp and Magnification color features used by the engine work the
  same inside a packaged (MSIX) app as in the plain `.exe`.
- For direct downloads from chromavale.com (outside the Store), the
  SmartScreen warning can only be removed with a paid signing option; see the
  notes in the project root `README.md`.
