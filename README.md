# Text on Screen app

Windows desktop overlay built with Tauri 2 and a vanilla TypeScript frontend. The window is transparent, always on top, hidden from the taskbar, and moves to the tray when closed. `F8` toggles visibility, `F9` toggles click-through mode, `Ctrl+F8` resets position and size, and the mouse wheel changes font size while active.

## Development

```bash
npm install
npm run tauri dev
```

`npm run build` checks and bundles the frontend. `cargo +stable check` checks the native shell. Set the server URL in the settings panel; the default is `http://localhost:8787`.

## Release updates

The app checks `/api/v1/version` and `/api/v1/releases/latest` at startup and from the settings panel. A newer semver release opens its signed installer with the install button. When the server advertises a newer `minimumClientVersion`, starting a room is blocked until the app is updated. The tagged/manual GitHub Actions workflow builds NSIS/MSI installers and can upload a selected `x.y.z` release to the server over SSH.

The protocol copy used by the client is in [`protocol/v1.md`](protocol/v1.md); the server repository is the canonical owner of protocol changes.
