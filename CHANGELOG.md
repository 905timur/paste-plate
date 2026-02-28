# Changelog

All notable changes to this project will be documented in this file.

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
