# Livi Pen app icon

**Design:** four white curved blades turning around a four-pointed star: a pen-like spark at the
centre of flowing Bézier-style strokes, on a full-bleed blue tile.

**Palette:**

| Colour | Hex (approx.) | Used for |
|---|---|---|
| Deep blue | `#011a57` | the field, lower left |
| Cyan | `#15d8d9` | the field's gradient, upper right |
| Paper white | `#fdfaf3` | the symbol |

**Tile:** a rounded square, `rx=112` on a 512 grid, transparent outside the tile. The macOS files add
Apple's transparent margin (tile = 824/1024 of the canvas); Windows and Linux use the tile edge to edge.

**Provenance:** made for Livi Pen by Livi (sarayunova) with ChatGPT image generation, 2026-10-25.
`livi-icon-source.png` (1254 px, no transparency) is the original. `livipen.svg` and
`livipen-small.svg` embed it as a raster, clipped to the rounded tile. Licence: see `LICENSE.txt`.

## Files

| File | What | Used by |
|---|---|---|
| `livi-icon-source.png` | the original artwork, 1254 px, square corners | source for everything below |
| `livipen.svg` | 512 master: the artwork at 1024 px, clipped to the tile | `packaging/icons.sh` |
| `livipen-small.svg` | lighter master (the artwork at 256 px) | `hicolor/scalable`, the DMG background |
| `livipen-1024.png` | full tile, 1024 px | docs, store listings |
| `livipen-macos-512.png` | tile with Apple margin, 512 px | runtime Dock icon (`apps/livipen/src/main.rs`) |
| `livipen.icns` | macOS icon set, 16 to 1024 px | `cargo xtask bundle` (`CFBundleIconFile`) |
| `livipen.ico` | Windows icon, 16 to 256 px | `apps/livipen/build.rs` (embedded in the `.exe`) |
| `hicolor/<size>/apps/com.livi_pen.liviverse.png` | Linux theme icons, 16 to 512 px; the 256 one is also the runtime icon on Windows and Linux, the 128 one the brand mark in the app bar and About box (`crates/ui-egui/src/brand.rs`) | `packaging/linux/com.livi_pen.liviverse.desktop` |
| `hicolor/scalable/apps/com.livi_pen.liviverse.svg` | Linux scalable icon | as above |

## How it reaches each OS

- **macOS:** the app sets the Dock / app-switcher icon at runtime (`ViewportBuilder::with_icon`); the
  `.app` from `cargo xtask bundle` carries `livipen.icns`.
- **Windows:** `build.rs` embeds `livipen.ico` and VERSIONINFO with `winresource` (taskbar, Start
  menu, Explorer, Alt-Tab); the runtime icon covers the title bar.
- **Linux:** install `packaging/linux/com.livi_pen.liviverse.desktop` to `share/applications/` and
  `hicolor/` to `share/icons/`. The app sets its Wayland app ID / X11 class to
  `com.livi_pen.liviverse`, so the dock matches the window to the launcher.

## Regenerate

Replace `livi-icon-source.png`, rebuild `livipen.svg` and `livipen-small.svg` from it (the
artwork embedded at 1024 and 256 px under a `rx=112` clip on a 512 viewBox), then run
`packaging/icons.sh` (needs `resvg`; `iconutil` on macOS for the `.icns`). It writes every PNG, the
`.ico` (via `cargo xtask ico`) and the `.icns`.
