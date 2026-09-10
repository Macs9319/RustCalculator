# Embed Inter, not SF Pro, for the macOS-style restyle

The GUI's font is being changed to approximate macOS's system look. The obvious choice would be SF Pro (Apple's own system font), but Apple's font license (`developer.apple.com/fonts/`) explicitly forbids embedding it in software running on non-Apple operating systems — and this app targets Linux/Windows/macOS uniformly via `eframe`. We're embedding Inter (SIL OFL 1.1, redistribution-permissive) instead, via `include_bytes!` into `FontFamily::Proportional` only. The numeric display deliberately keeps egui's separate built-in monospace font untouched, to avoid digit-width jitter that a proportional font would introduce.

See `docs/research/macos-style-ui.md` and `docs/research/inter-font-embedding.md` for the full licensing and API details.
