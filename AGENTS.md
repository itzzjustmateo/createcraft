# CreateCraft - AI Development Guidelines

## Project
CreateCraft is a Rust remake of **BreakMine: Revived**, an open-source 3D voxel game inspired by Minecraft. The repository is a Cargo workspace using Rust 2024.

## Architecture
- `apps/client` is the browser client and is intended to compile to WebAssembly.
- `apps/server` is a separate native Rust application.
- The client is responsible for rendering (using wgpu), input, UI, and client-side game interaction.
- The server owns authoritative world simulation, player state, multiplayer, and persistence.
- Client and server communicate through `crates/protocol`; transport belongs in `crates/networking`.
- Keep shared crates platform-neutral. In particular, do not introduce rendering or browser-only dependencies into server/shared logic.
- Crate package names use the `createcraft-` prefix and workspace package settings/dependencies should be inherited from the root `Cargo.toml`.
- Keep crate responsibilities aligned with their workspace location: `world` and `physics` provide simulation foundations; `game-core` composes game logic; `assets`, `config`, `input`, and `rendering` provide client-facing systems.

## Development Rules
- Keep implementations modular and maintainable.
- Prefer small, focused changes.
- Avoid unnecessary dependencies; add third-party crates only when they are needed and configure shared dependencies in the root manifest when appropriate.
- Do not introduce abstractions without a clear purpose.
- Use proper Rust error handling.
- Document non-obvious implementations.
- Keep client and server responsibilities separate.
- Preserve the workspace's lint policy, including `unsafe_code = "forbid"`.
- Update relevant documentation when changing architecture, crate responsibilities, or developer workflows.

## Validation
Run commands from the workspace root:

```sh
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

For browser-client changes, also verify the WebAssembly target/build path affected by the change; do not assume a native workspace build validates browser compatibility.

## AI Workflow
- Inspect existing code before modifying it.
- Explain architectural decisions.
- Run relevant formatting, compilation, and tests.
- Never silently remove existing functionality.
- Do not implement unrelated systems.