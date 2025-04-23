# Road Intersection Traffic Simulation

A traffic control strategy simulation for a capital city intersection, implemented in Rust using the SDL2 library.

## Table of Contents

- [Objectives](#objectives)
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Building and Running](#building-and-running)
- [Project Structure](#project-structure)
- [Environment and Rules](#environment-and-rules)
  - [Roads](#roads)
  - [Traffic Lights](#traffic-lights)
  - [Vehicles](#vehicles)
- [Controls](#controls)
- [Implementation Details](#implementation-details)
- [Color Coding](#color-coding)
- [Troubleshooting](#troubleshooting)
- [Future Improvements](#future-improvements)
- [Contributing](#contributing)
- [License](#license)

## Objectives

The goal of this project is to:

- Model traffic flow through a city intersection
- Implement an efficient traffic control strategy in Rust
- Visualize the simulation with proper vehicle behavior
- Demonstrate collision avoidance and traffic management
- Provide an interactive simulation with keyboard controls

## Features

- **Rust-native implementation** leveraging:
  - `sdl2` crate for graphics and input handling
  - Rust's ownership model for safe memory management
  - Iterator patterns for efficient vehicle processing
- **Interactive controls**:
  - Keyboard-based vehicle spawning
  - Visual feedback

## Prerequisites

- Rust toolchain (stable)
- SDL2 development libraries
- Cargo package manager

## Installation

### 1. Install Rust

If you don't have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Install SDL2 Dependencies

#### Linux (Debian/Ubuntu)

```bash
sudo apt update
sudo apt install libsdl2-dev
```

#### MacOS

```bash
brew install sdl2
```

#### Windows

Download SDL2 development libraries from [libsdl.org](https://www.libsdl.org/) and add to your PATH.

### 3. Clone the Repository

```bash
git clone https://github.com/johneliud/road_intersection_raid.git
cd road_intersection_raid
```

## Building and Running

### Debug Build

```bash
cargo run
```

### Release Build (Optimized)

```bash
cargo run --release
```

### Running Tests

```bash
cargo test
```

## Environment and Rules

### Roads

The simulation features two perpendicular roads forming a four-way intersection:

```
                        North
                    |  ↓  |  ↑  |
                    |  ↓  |  ↑  |
                    |     |     |
                    |     |     |
West ---------------+     +--------------- East
                    |     |     |
                    |     |     |
                    |  ↓  |  ↑  |
                    |  ↓  |  ↑  |
                        South
```

### Traffic Lights

- Implemented using finite state machines
- Red/Green cycle with configurable timing
- Positioned at each entry point to the intersection
- Smart scheduling to minimize congestion

### Vehicles

Key characteristics:

- Color-coded by intended route
- Fixed velocity (configurable per direction)
- Maintains safe following distance
- Obeys traffic signals
- Cannot change route after spawning

## Controls

| Key   | Action                              |
| ----- | ----------------------------------- |
| ↑     | Spawn southbound vehicle            |
| ↓     | Spawn northbound vehicle            |
| →     | Spawn westbound vehicle             |
| ←     | Spawn eastbound vehicle             |
| r     | Spawn vehicle from random direction |
| Space | Pause simulation                    |
| Esc   | Quit simulation                     |

## Implementation Details

### Color Coding

| Color | Hex Code | Route      |
| ----- | -------- | ---------- |
| Red   | #FF0000  | Left turn  |
| Green | #00FF00  | Right turn |
| Blue  | #0000FF  | Straight   |

## Troubleshooting

### Common Issues

1. **SDL2 Not Found**

   ```
   error: linking with `cc` failed: exit status: 1
   = note: /usr/bin/ld: cannot find -lSDL2
   ```

   **Solution**: Install SDL2 development libraries as shown in [Prerequisites](#prerequisites)

2. **Permission Denied**
   **Solution**: Run with `sudo` if required by your system

3. **Cargo Build Errors**
   **Solution**: Ensure you're using Rust stable:
   ```bash
   rustup update stable
   ```

## Future Improvements

- [ ] Add pedestrian crossings
- [ ] Implement emergency vehicle priority
- [ ] Add configurable traffic patterns
- [ ] Introduce variable vehicle speeds
- [ ] Add day/night cycle
- [ ] Implement traffic statistics collection

## License

MIT License - see [LICENSE](LICENSE) for details
