# ascii-clock

A minimal terminal clock with ASCII art digits and mirror reflection effect.

```
   ██████  ████████          ██    ██  ██████            ██████  ██    ██
  ██    ██       ██    ██    ██    ██ ██    ██    ██    ██    ██ ██    ██
  ██    ██     ██            ████████  ██████             ████   ████████
  ██    ██    ██       ██          ██ ██    ██    ██    ██    ██       ██
   ██████     ██                   ██  ██████            ██████        ██
  ─────────────────────────────────────────────
    ██████   ██████             ████    ██████
      ██  ██    ██    ██    ██       ██
       ████████              ████████
```

## Features

- **24-hour Beijing time (UTC+8)** - Always shows correct local time
- **True color gradient** - Smooth color transitions using 24-bit ANSI escape codes
- **Mirror reflection** - 3-row reflection with progressive dimming below a separator line
- **Alternate screen buffer** - Clean entry/exit, original terminal content is preserved
- **Zero dependencies** - Pure Rust standard library, no external crates

## Install

### From source

```bash
git clone https://github.com/missercatos/ascii-clock.git
cd ascii-clock
cargo build --release
cp target/release/ascii_clock ~/.local/bin/clock
```

### Binary

Download from [Releases](https://github.com/missercatos/ascii-clock/releases) and place in your `$PATH`.

## Usage

```bash
clock
```

Press `Ctrl+C` to exit.

## Color Palette

| Stop | Color | Hex |
|------|-------|-----|
| 0%   | Grey  | `#78909C` |
| 25%  | Light Blue | `#90CAF9` |
| 50%  | Sky Blue | `#87CEEB` |
| 75%  | Pale Blue | `#BBDEFB` |

## Customization

Edit the `stops` array in `src/main.rs` to change the gradient colors:

```rust
let stops = [
    (0.00, Color { r: 120, g: 144, b: 156 }),  // Grey
    (0.25, Color { r: 144, g: 202, b: 249 }),  // Light Blue
    (0.50, Color { r: 135, g: 206, b: 235 }),  // Sky Blue
    (0.75, Color { r: 187, g: 222, b: 251 }),  // Pale Blue
    (1.00, Color { r: 120, g: 144, b: 156 }),  // Grey
];
```

Change the timezone by adjusting the offset in `get_time()`:

```rust
let beijing = secs + 8 * 3600; // UTC+8
```

## Requirements

- A terminal with true color (24-bit) support (most modern terminals)
- Rust toolchain (for building from source)

## License

MIT
