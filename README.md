# Rust OpenGL Engine

A high-performance, Modern OpenGL graphics engine written in pure Rust. this project demonstrates a ground-up implementation of a 3D renderer, avoiding heavy game engine dependencies in favor of learning and low-level control.

## 🚀 Features

- **Custom Math Library**: A purpose-built linear algebra implementation (`src/math.rs`) handling Vectors and Matrices without relying on crates like `glam` or `cgmath`.
- **Modern OpenGL 4.1**: Built on `glow` for safe, zero-cost cross-platform OpenGL bindings.
- **Asset Management**: Integrated Wavefront OBJ loader (`src/obj_loader.rs`) for parsing standard 3D models.
- **Core Abstractions**:
  - `RenderContext`: Manages the complex lifecycle of Windowing (`winit`) and OpenGL Contexts (`glutin`).
  - `Program`: Clean, type-safe wrapper for Shader compilation, linking, and Uniform management.
  - `Mesh` & `buffers`: RAII compliant wrappers for VBOs, VAOs, and EBOs.
- **Camera System**: Perspective camera implementation with LookAt matrix generation.

## 🛠️ Architecture

The engine is structured to be modular and readable:

```
src/
├── main.rs          # Entry point. Initializes the EventLoop and App.
├── app.rs           # Core application logic. Holds state (meshes, camera, shader) and handles the main event loop (rendering, input).
├── render_context.rs # Low-level context management. Handles window creation (winit) and OpenGL context initialization (glutin).
├── math.rs          # Custom linear algebra library. Implements `Vec3` (cross/dot products, normalization) and `Mat4` (perspective, look_at, multiplication).
├── shader.rs        # `Program` struct. Compiles vertex/fragment shaders, links programs, and manages uniform uploading (matrices, vectors).
├── camera.rs        # `Camera` struct. Manages view matrices and camera positioning.
├── obj_loader.rs    # OBJ file parser. Reads vertex positions, normals, and indices from standard .obj files.
├── mesh.rs          # High-level `Mesh` abstraction. Combines VAO, VBO, and EBO into a drawable object.
├── mesh_data.rs     # Intermediate data structures (`MeshData`, `Vertex`) for loading assets before GPU upload.
├── vertex_array.rs  # `VertexArray` (VAO). formatting and binding vertex attributes.
├── vertex_buffer.rs # `VertexBuffer` (VBO). Manages raw vertex data on the GPU.
└── index_buffer.rs  # `IndexBuffer` (EBO). Manages face indices for indexed drawing.
```

## 🤖 Agentic Development

This project was built using **Antigravity**, a next-generation AI coding agent from Google DeepMind. It serves as a showcase of:
- **Zero-to-One Compilation**: Building a complex graphics stack from an empty directory.
- **Self-Correction**: The agent iteratively fixed compilation errors and logic bugs.
- **Tool Use**: Leveraging command-line tools and file manipulations to construct the codebase.

## 📦 Getting Started

Ensure you have Rust (2024 edition) installed.

### Run the Engine
```bash
cargo run --release
```

*Note: The engine expects `stanford_bunny.obj` and `triangle.obj` in the root directory.*
