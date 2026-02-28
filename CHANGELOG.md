# Changelog

All notable changes to this project will be documented in this file.

## [0.1.2] - 2026-02-28

### Added

- Tool ribbon: Vertical panel on left with Move (V), Transform (T), and
  Selection (S) tool buttons
- Layer ribbon: Horizontal panel at bottom showing layer thumbnails with
  selection highlighting
- Added `active_tool` state to track currently selected tool

### Changed

- Canvas automatically resizes to fill remaining space when side and bottom
  panels are active

## [0.1.1] - 2026-02-28

### Fixed

- Wayland clipboard: Added wl-paste fallback for image pasting (arboard
  workaround)
- Input handling: Fixed Ctrl+V not being detected when egui intercepts it at
  framework level

## [0.1.0] - 2026-02-28

### Added

- Initial release
- Cross-platform clipboard support
- Image clipboard support
- Wayland and X11 support on Linux
