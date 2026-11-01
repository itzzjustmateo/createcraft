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

The root manifest centralizes package metadata, shared dependencies, and lint settings. Check the native workspace from the repository root with:

```sh
cargo check --workspace
cargo test --workspace
cargo run -p createcraft-server
```

## Run the browser client

In Git Bash, install the WebAssembly target and Trunk once:

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
```

From the repository root, start the local browser build server:

```sh
cd apps/client
export RUSTC="$(rustup which rustc)"
trunk serve
```

Open the URL printed by Trunk (by default, <http://127.0.0.1:8080>). The client uses wgpu to render a colored triangle on the HTML canvas. `RUSTC` selects rustup's compiler so Cargo can find the installed WebAssembly target.

If `trunk` is not recognized after installing it, add Cargo's bin directory to the current Git Bash `PATH`:

```sh
export PATH="$HOME/.cargo/bin:$PATH"
```

Windows Application Control may block Cargo from running compiler build scripts while installing Trunk. If `cargo install` is blocked, ask your administrator to approve the install or provide an approved Trunk installation; switching shells does not remove that restriction.

A standalone WebAssembly build can be checked from the workspace root with:

```sh
export RUSTC="$(rustup which rustc)"
cargo check -p createcraft-client --target wasm32-unknown-unknown
```

## Core Systems

- **Camera:** First-person camera with mouse look and perspective projection.
- **Player:** Movement, gravity, jumping, and block collision.
- **World:** Procedurally generated voxel terrain, chunk management, and block interaction.
- **Rendering:** GPU-accelerated rendering using wgpu.
- **Networking:** WebSocket communication between the WASM client and native server.