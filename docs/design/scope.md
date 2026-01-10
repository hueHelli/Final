# Engine Scope & Goals

## Vision

- 2D-first, but 3D-ready game engine.
- General-purpose: usable for many genres (platformers, action, top-down, strategy, etc.).
- Cross-platform desktop focus.

## Platforms

- Primary targets:
  - Windows (x86_64)
  - Linux (x86_64), with SteamOS/Steam Deck as a key environment.
- Non-goals for now:
  - Consoles, mobile, WebAssembly.

## Rendering & Architecture

- 2D-first rendering (sprites, tilemaps, UI) on top of a 3D-capable transform and camera system.
- Use cross-platform APIs (e.g., winit + wgpu) to reach Windows and Linux with high performance.
- Plan for later 3D additions: perspective cameras, 3D meshes, lighting, and 3D physics.

## Genre Coverage

- Support:
  - Platformers, action, twin-stick shooters.
  - Top-down / ARPG.
  - Strategy / simulation with many entities.
  - Puzzle / narrative games (via UI and scene management).
- ECS and systems must be flexible and not tied to a single genre.

## Performance Goals

- Target 60–144 FPS on mid-range desktop hardware in typical 2D scenes.
- Design with:
  - Data-oriented ECS.
  - Batching in rendering.
  - Minimal per-frame allocations and dynamic dispatch.
- Build profiling and debugging tools early.

## Engine Personality

- General-purpose, not heavily opinionated.
- Modular design:
  - Clear engine core.
  - Pluggable subsystems (physics, audio, scripting, etc.).
- Rust is used both for the engine and for game logic (scripting via Rust crates/plugins).
