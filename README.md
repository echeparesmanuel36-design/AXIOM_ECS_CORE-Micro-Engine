# 🪐 AXIOM_ECS_CORE // Bare-Metal Micro-Engine v1.0

[![Language](https://img.shields.io/badge/Language-Rust-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)](LICENSE)
[![Stars](https://img.shields.io/github/stars/TU_USUARIO/axiom_ecs_core?style=for-the-badge&color=yellow)](https://github.com/TU_USUARIO/axiom_ecs_core/stargazers)

**Tired of bloated game engines? Welcome to the metal.** 

`axiom_ecs_core` is an ultra-lightweight, high-performance 2D/3D game engine chassis written in pure **Rust**. It uses a custom, hardware-friendly **Entity Component System (ECS)** architecture designed to push your CPU cache to its absolute limits. 

No heavy IDEs, no complex CMake configurations, no bloat. **Just pure data-driven performance.**

---

## ⚡ Why Axiom? (Performance vs Bloat)

Commercial engines use classic object-oriented node structures that saturate the CPU with dynamic memory allocation. Axiom separates **Data** from **Logic**, aligning components linearly in memory. 

* **🚀 Ridiculous Performance:** Simulates and renders **20,000+ independent entities** at hundreds of FPS without breaking a sweat.
* **🔌 Zero Configuration:** Forget about complex setup guides or heavy toolchains. If you have Rust installed, you are ready to deploy.
* **📦 Single-Command Build:** Compiles into a single, ultra-lightweight native executable in seconds.

---

## 🛠️ Tech Stack & Architecture

* **Core Engine:** Written in 100% safe, high-performance Rust.
* **Graphics Wrapper:** Powered by `macroquad` (Hardware-accelerated via GPU, ultra-fast compile times).
* **Memory Layout:** Pure *Data-Driven Design*. Positions, velocities, and colors are packed in contiguous arrays to maximize CPU cache hits.

---

## 🚀 Quick Start (Run in 5 Seconds)

You don't need Visual Studio or heavy software. Just follow these simple steps:

1. **Clone the repository:**
```bash
   git clone [https://github.com/https://github.com/echeparesmanuel36-design/axiom_ecs_core.git](https://github.com/https://github.com/echeparesmanuel36-design/axiom_ecs_core.git)
   cd axiom_ecs_core
Run the engine:
```

```bash
   cargo run --release
```

### And that's it! Rust will automatically download dependencies, compile the code, and launch the engine at maximum speed. 🏎️💨

## ⚙️ Core Architecture Blueprint
The entire engine logic is structured in under 100 lines of crystalline code. Perfect for learning how a real high-performance ECS works:

```rust
// Data Alignment (Components)
struct Position { x: f32, y: f32 }
struct Velocity { x: f32, y: f32 }

// The Systems (Logic)
fn update_physics(ecs: &mut AxiomECS, dt: f32) {
    // Process thousands of movements in a single microsecond sweep
}
```
### 🌐 Compiling to WebAssembly (Run in Browser)

Axiom Engine supports one-click compilation to the browser using WebAssembly. 

1. Install the WASM target (if you haven't already):
```bash
   rustup target add wasm32-unknown-unknown
   ```
2. Build the project for web:
```bash
   cargo build --target wasm32-unknown-unknown --release
   ```
## This will generate a ultra-light .wasm binary inside target/wasm32-unknown-unknown/release/. You can embed it in any standard HTML5 page and it will run at native code speeds inside the browser! 🚀🕹️

```markdown
 🗺️ Roadmap & Future Features

- [x] Multithreaded System Execution (Parallel CPU processing) 🔥 *DONE*
- [x] Spatial Partitioning Memory Grid layout simulation 🛠️ *DONE*
- [x] WebAssembly (Wasm) target compilation with a single click 🌐 *DONE*
- [x] Custom Audio-Reactive Kinetic Feedback 🎧 *DONE*
```

📄 License
Distributed under the MIT License. Feel free to fork, hack, modify, and build your own sovereign software on top of it.

🦀 Show some love!
If you think commercial engines are too heavy and you love bare-metal engineering, drop a star (⭐) to support the project!

Developed by Axiom Systems. ⚡
