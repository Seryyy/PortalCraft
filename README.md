# PortalCraft

**Minecraft mechanics in Portal 2** — Block placement, inventory, and physics sync between Portal 2 (Source Engine) and Minecraft (Fabric).

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Portal 2 (Source Engine)                         │
│  ┌─────────────────┐    ┌─────────────────────────────────────┐   │
│  │  SourceMod      │◄──►│  PortalCraft Extension (C++)        │   │
│  │  Plugin         │    │  - Block placement (prop_physics)   │   │
│  │  (SourcePawn)   │    │  - Inventory management (SQLite)    │   │
│  └────────┬────────┘    │  - Named pipe to bridge             │   │
│           │             └─────────────────┬───────────────────┘   │
│           │ IPC (named pipe)              │                       │
└───────────┼───────────────────────────────┼───────────────────────┘
            │                               │
            ▼                               ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    Bridge Daemon (Rust)                             │
│  • Named pipe (Windows) ↔ TCP (localhost:25565)                    │
│  • Protocol translation (bincode)                                   │
│  • Session management, reconnection, heartbeats                    │
└─────────────────────────────┬───────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    Minecraft (Fabric Mod)                           │
│  • Receives block updates → renders in world                       │
│  • Sends player actions (break/place) → Portal 2                   │
│  • Custom inventory screen mirroring Portal 2                      │
│  • Player entity sync (position, rotation, held item)              │
└─────────────────────────────────────────────────────────────────────┘
```

## Components

| Component | Language | Purpose |
|-----------|----------|---------|
| `portalcraft-protocol` | Rust | Shared types, messages, coordinate transforms |
| `portalcraft-bridge` | Rust | Daemon connecting named pipe ↔ TCP |
| `portalcraft-sm` | C++ (SourceMod) | Portal 2 extension: blocks, inventory, physics |
| `portalcraft-fabric` | Java (Fabric) | Minecraft mod: rendering, networking, UI |

## Quick Start

### Prerequisites

**Portal 2:**
- Portal 2 installed via Steam
- SourceMod 1.12+ + Metamod:Source installed in `portal2/addons/`

**Development:**
- Rust 1.80+ (`rustup`)
- Java 21 (Eclipse Temurin)
- Visual Studio 2022 with C++ workload (v142 toolset)
- Source SDK 2013

### Build

```bash
# Build Rust workspace (protocol + bridge)
cd PortalCraft
cargo build --release

# Build Fabric mod
cd portalcraft-fabric
./gradlew build

# Build SourceMod extension (open .sln in Visual Studio)
# TODO: Create .vcxproj
```

### Run

```bash
# Terminal 1: Portal 2 dedicated server
srcds.exe -game portal2 -console +map sp_a1_intro1 +maxplayers 1

# Terminal 2: Bridge daemon
cargo run --release --bin portalcraft-bridge

# Terminal 3: Minecraft (Fabric dev environment)
cd portalcraft-fabric
./gradlew runClient
```

## Protocol

All communication uses `bincode`-serialized messages defined in `portalcraft-protocol`:

- `PlaceBlock` / `BreakBlock` — Block operations
- `InventorySync` / `InventoryAction` — Inventory management
- `PlayerState` — Position, rotation, health, gamemode
- `ChunkRequest` / `ChunkData` — World sync

Coordinate system: **1 MC block = 52 Hammer units** (≈ 1 meter)

## Development Status

- [x] Protocol crate (types, messages, coords, blocks)
- [x] Bridge daemon (TCP server, routing, heartbeats)
- [ ] SourceMod C++ extension (block placement, named pipe)
- [ ] Fabric mod (block rendering, inventory UI, networking)
- [ ] SourcePawn plugin (commands, config)
- [ ] Installer / packaging

## License

MIT