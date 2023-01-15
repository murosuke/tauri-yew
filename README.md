# tauri-yew
sample for yew app based on tauri that load html from server or local file.
## About
tauri will first try to access to server written in test.toml. If connection fails, then shows local ui/index.html.

## How to try this
- `cd yew`
- `trunk serve`
  - Console log will show url to access. Check that browser can access to the url.
- `cd src-tauri`
  - with another console, not with the one started yew server.
- `cargo tauri dev`
  - Start application once to let create test.toml beside tauri.exe.
- edit test.toml file created.
  - Change url to what "trunk serve" shown.
- `cargo tauri dev`
  - Start app again.

