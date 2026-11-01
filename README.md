# CreateCraft

A Rust remake of **BreakMine: Revived**, the open-source 3D voxel engine game inspired by Minecraft. The workspace is structured for a WebAssembly client and a dedicated native server.

## Architecture

CreateCraft uses two separate applications:

- **Client:** Rust compiled to WebAssembly, responsible for rendering, camera, player input, UI, and networking.
- **Server:** A native Rust application responsible for world simulation, player state, multiplayer, and persistence.
- **Shared crates:** Common types, world data structures, and client-server protocol.

## Workspace

- `apps/client` — browser client entry point.
- `apps/server` — dedicated server entry point.
- `crates/game-core` — game orchestration shared by the applications.
- `crates/world` and `crates/physics` — voxel-world and simulation foundations.
- `crates/protocol` and `crates/networking` — shared messages and transport.
- `crates/rendering`, `crates/input`, `crates/assets`, and `crates/config` — client-facing systems.

The root manifest centralizes package metadata, shared dependencies, and lint settings. From the repository root, use:

```sh
cargo check --workspace
cargo test --workspace
cargo run -p createcraft-client
cargo run -p createcraft-server
```

## Core Systems

- **Camera:** First-person camera with mouse look and perspective projection.
- **Player:** Movement, gravity, jumping, and block collision.
- **World:** Procedurally generated voxel terrain, chunk management, and block interaction.
- **Rendering:** GPU-accelerated rendering using wgpu.
- **Networking:** WebSocket communication between the WASM client and native server.