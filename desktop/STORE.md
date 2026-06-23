# Publishing ChromaVale to the Microsoft Store

The Microsoft Store is the route that removes the "Windows protected your PC"
(SmartScreen) warning **for free**: you submit an MSIX package and Microsoft
re-signs it. No paid code-signing certificate is required.

Tauri does not output MSIX directly, so this folder packages the built
`chromavale.exe` into an MSIX using the Windows SDK (`makeappx` + `signtool`):

- [`AppxManifest.xml`](AppxManifest.xml) - the package manifest (identity,
  logos, full-trust capability).
- [`pack.ps1`](pack.ps1) - stages the exe + assets, packs the MSIX, and signs
  it with a local test certificate.

This pipeline is verified working: running `pack.ps1` produces a valid, signed
`ChromaVale.msix`. The only steps left require a Partner Center account.

## Build a test MSIX locally (already works)

```powershell
# From desktop/, after: npm run tauri build
powershell -ExecutionPolicy Bypass -File msix\pack.ps1
```

This creates `msix\ChromaVale.msix`, signed with a self-signed test cert
(`devcert.pfx`, auto-created, `CN=ChromaVale`). To install it for testing,
trust that cert once (run as admin), then install:

```powershell
Import-PfxCertificate -FilePath msix\devcert.pfx -CertStoreLocation Cert:\LocalMachine\Root -Password (ConvertTo-SecureString "chromavale" -AsPlainText -Force)
Add-AppxPackage msix\ChromaVale.msix
```

## What only you can do (Partner Center)

1. **Create a developer account** at
   [partner.microsoft.com](https://partner.microsoft.com/dashboard) and let
   Microsoft verify your identity (a small one-time registration fee may
   apply).
2. **Reserve the app name.** Partner Center then gives you the package identity
   values under *Product management -> Product identity*:
   - **Package/Identity Name** (e.g. `1234Publisher.ChromaVale`)
   - **Publisher** (e.g. `CN=ABCD1234-1234-...`)
   - **Publisher display name**
3. **Put those values into `AppxManifest.xml`** (replace `Name`, `Publisher`,
   `PublisherDisplayName`).
4. **Pack without signing** (Microsoft signs it):

   ```powershell
   powershell -ExecutionPolicy Bypass -File msix\pack.ps1 -SkipSign
   ```

5. **Submit** `msix\ChromaVale.msix` in Partner Center. Microsoft re-signs it,
   so end users install with no SmartScreen warning.

## Notes

- Bump the `Version` in `AppxManifest.xml` (4 parts, e.g. `0.2.0.0`) for each
  new submission.
- Build one package per architecture you support (x64 shown here; add Arm64 if
  needed).
- ChromaVale needs the WebView2 runtime: present on Windows 11, available as the
  Evergreen runtime on Windows 10. If WACK (Windows App Certification Kit)
  flags it, declare it as a package dependency.
- The gamma-ramp, Magnification and foreground-app features work the same inside
  a packaged (MSIX) app as in the plain `.exe`.
- For direct `.exe` downloads from chromavale.com (outside the Store), the
  SmartScreen warning can only be removed with a paid signing option; see the
  project root `README.md`.
