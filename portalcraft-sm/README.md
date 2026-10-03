/**
 * PortalCraft SourceMod Extension
 * 
 * C++ extension for SourceMod that implements block placement, inventory management,
 * and communication with the bridge daemon via named pipe.
 */

// TODO: Implement SourceMod C++ extension
// This will be a Visual Studio project (.vcxproj) that compiles to portalcraft.ext.2.sdk2013.dll
// 
// Key components:
// 1. extension.cpp/h - Main extension entry point, implements ISourceModExtension
// 2. portalcraft.cpp/h - Core logic: block placement, inventory, named pipe communication
// 3. vphysics_wrapper.cpp/h - Helpers for spawning prop_physics_multiplayer
// 4. sqlite_wrapper.cpp/h - SQLite database for persistent block/inventory storage
// 5. named_pipe.cpp/h - Windows named pipe client for bridge communication

// Build requirements:
// - SourceMod 1.12+ SDK (sourcepawn, sourcemod headers)
// - Source SDK 2013 (tier0, tier1, vphysics, server dll headers)
// - Visual Studio 2022 with v142 toolset
// - Link against: sourcemod, tier0, tier1, vphysics, sqlite3

// Key hooks to implement:
// - OnPlayerUse (E key) -> raycast -> place block at grid position
// - OnPlayerRunCmd -> sync player position/rotation
// - OnEntityCreated -> track entities
// - OnGameFrame -> periodic sync

// Commands to register (via SourcePawn plugin):
// - sm_block_give <type> <count>
// - sm_block_list
// - sm_inventory
// - sm_sync

// Named pipe protocol:
// - Connect to \\.\pipe\portalcraft
// - Send/receive bincode-encoded portalcraft_protocol::Message
// - Reconnect on disconnect

// Database schema (portalcraft.db):
// CREATE TABLE blocks (id INTEGER PRIMARY KEY, x INTEGER, y INTEGER, z INTEGER, type INTEGER, owner TEXT, map TEXT, created_at INTEGER);
// CREATE TABLE inventory (player_id TEXT, slot INTEGER, item_type INTEGER, count INTEGER, damage INTEGER, nbt TEXT, PRIMARY KEY(player_id, slot));
// CREATE TABLE sync_state (key TEXT PRIMARY KEY, value TEXT);