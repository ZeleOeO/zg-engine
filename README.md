# zg-engine

A Rust-based 3D Graphics Engine built with WebGPU (wgpu). Renders a textured, rotating cube with a modern graphics pipeline. A learning project for understanding GPU-accelerated rendering, shader authoring (WGSL), and event-loop-driven graphics in Rust.

> Thinking out loud: It's crazy to me how fast this all is. GPU Programming is so insanely cool

---

>  EVERYTHING IS BLOCKED FOR NOW, IT'S IN BUILD MODE 

## Technologies

[![Rust](https://img.shields.io/badge/Rust-1.85%2B-orange?logo=rust&logoColor=white)](https://www.rust-lang.org) [![wgpu](https://img.shields.io/badge/wgpu-29.0-brightgreen?logo=webgpu&logoColor=white)](https://wgpu.rs) [![winit](https://img.shields.io/badge/Windowing-winit-blue)](https://github.com/rust-windowing/winit) [![WGSL](https://img.shields.io/badge/Shader-WGSL-purple)](https://www.w3.org/TR/WGSL/)

## Prerequisites

- Rust Toolchain (rustc & cargo) — edition 2024 (Rust 1.85+)
- A GPU with Vulkan / Metal / DirectX 12 support
- **Optional** — An IDE such as VSCode (with rust-analyzer) or IntelliJ Rust. I personally use my terminal with nvim

## Installation (Will work on both powershell and bash/zsh terminals)

1.  Clone the repository:
    
    ```
    git clone https://github.com/ZeleOeO/zg-engine.git
    ```
    
2.  Navigate to the project directory:
    
    ```
    cd zg-engine
    ```
    
3.  Ensure Rust is installed by running `cargo --version`
    
### Run Application

1.  Build and run the engine:
    
    ```
    cargo run --release
    ```
    
## Usage

Once the application starts, a window titled **"Graphics Engine"** opens with a textured 3D cube rotating continuously around the Y-axis.

- Press **Escape** to close the window and exit
- Set log verbosity via the `RUST_LOG` environment variable (e.g., `RUST_LOG=info cargo run --release`)

The embedded texture (`happy-tree.png`) is mapped onto all six faces of the cube. The rotation speed is driven by a uniform buffer passed to the vertex shader.

### TODO

In no particular order

- [X] Make the code cleaner - ongoing
- [X] Camera controls (orbit)
- [X] Create Systems
- [X] Switch querying to use Query
- [X] Add a central "system" function
- [ ] Add ordering to systems
- [ ] Make folders into crates
- [ ] Add crate level access modifiers
- [ ] Fix all the places where there's an unwrap and handle it properly
- [ ] Asynchrounous and multithreading
- [ ] Multiple objects / scene graph
- [ ] Lighting (Phong / PBR)
- [ ] OBJ / GLTF model loading
- [ ] MSAA / post-processing effects
- [ ] ImGui debug overlay


### Potential Folder Structure
``` bash
app/           -- windowing, input, game loop, ties everything together
renderer/      -- the actual wgpu engine (device, passes, pipelines)
  core/        -- Device/Queue/Surface wrapper, resize handling
  resources/   -- buffers, textures, samplers, bind groups (with pooling/caching)
  passes/      -- render passes as discrete, composable units
  pipelines/   -- pipeline layouts, shader modules, pipeline cache
  scene/       -- render graph / draw list construction
  shaders/     -- .wgsl files, often with includes/preprocessing
assets/        -- loaders (gltf, textures, etc.), converted into GPU resources
ecs/ or scene graph/  -- your actual game/scene data, independent of renderer
```

## Tests

I don't have a massive test suite yet, but if I do come back to it, you can run the unit tests with this:

```
cargo test
```

## Steps to Contribute

Contributions are more than welcome, I'm still figuring out the architecture though, so... keep that in mind or something.

1.  Open an issue first so I can like keep track, but if that's too much stress that's fine too
2.  Fork the Repository
3.  Clone your fork
4.  Create a new branch:
    
    ```
    git checkout -b your-branch-name
    ```
    
5.  Make your change
6.  Commit your change, please use [Conventional Commits](https://gist.github.com/qoomon/5dfcdf8eec66a051ecd85625518cfd13) if you can.
7.  Push your change
8.  Make a pull request and reference your issue

Please stick to idiomatic Rust patterns, don't mess up my already spaghetti code.
