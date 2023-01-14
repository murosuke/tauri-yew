# tauri-yew
sample for yew app based on tauri that load html from server or local file.
## about
tauri will first try to access to server written in test.toml. If connection fails, then shows local ui/index.html.

## try this
- cd yew
- trunk serve
  - Console log will show url to access. Check browser can access.
- cd src-tauri
- edit test.toml file
  - Change url "trunk serve" shown.
- cargo tauri dev

