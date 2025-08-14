# Reptil en Peligro

**Reptil en Peligro** is a terminal-based Snake game built in Rust using [ratatui](https://github.com/tui-rs-revival/ratatui) for the TUI (Terminal User Interface) and [crossterm](https://github.com/crossterm-rs/crossterm) for terminal input/output handling.

## Features

- Responsive TUI rendering with `ratatui`
- Smooth snake movement
- Fruits displayed as square blocks for consistent visuals
- Stylish top banner displaying:
  - Game title
  - Current score
  - Elapsed time
- Dynamic board resizing based on terminal size
- Collision detection (walls, snake body)
- Arrow key controls + `q` / `Esc` to quit

## Requirements

- Rust and Cargo installed ([Install Rust](https://rustup.rs/))
- ANSI-compatible terminal

## Installation

### From source
```bash
git clone https://github.com/<your-user>/reptil-en-peligro.git
cd reptil-en-peligro
cargo run
```

### Global installation via `cargo install`
```bash
cargo install --git https://github.com/<your-user>/reptil-en-peligro
```
Then run:
```bash
reptil-en-peligro
```

## Controls

| Key   | Action         |
|-------|----------------|
| ↑     | Move up        |
| ↓     | Move down      |
| ←     | Move left      |
| →     | Move right     |
| `q`   | Quit           |
| `Esc` | Quit           |

## Project Structure

```
src/
├── main.rs      # Entry point
├── ui.rs        # Board and interface rendering
├── game.rs      # Game logic
├── event.rs     # Keyboard input and tick handling
```

## Example Screenshot
<img width="1126" height="621" alt="image" src="https://github.com/user-attachments/assets/94efb86a-d098-4e90-bd3e-4ad25fb5340b" />
---

**License:** MIT
