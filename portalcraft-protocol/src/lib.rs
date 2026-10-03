//! PortalCraft shared protocol types
//! 
//! This crate defines all messages, coordinates, and block types
//! shared between Portal 2 (SourceMod extension), the bridge daemon,
//! and the Minecraft Fabric mod.

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Scale factor: 1 Minecraft block = 52 Hammer units (≈ 1 meter)
pub const BLOCK_SCALE: f32 = 52.0;

/// Protocol version for compatibility checking
pub const PROTOCOL_VERSION: u32 = 1;

/// Unique session identifier for this sync session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 3D position in Minecraft coordinates (meters, Y-up)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct McPos {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl McPos {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Convert to block coordinates (integers)
    pub fn to_block_pos(&self) -> BlockPos {
        BlockPos::new(
            self.x.floor() as i32,
            self.y.floor() as i32,
            self.z.floor() as i32,
        )
    }

    /// Convert to Portal 2 Hammer coordinates (Z-up, units)
    pub fn to_hammer(&self) -> HammerPos {
        HammerPos::new(
            self.x as f32 * BLOCK_SCALE,
            self.z as f32 * BLOCK_SCALE,  // MC Y → Hammer Z
            -self.y as f32 * BLOCK_SCALE, // MC Z → Hammer -Y
        )
    }
}

/// 3D position in Portal 2 Hammer coordinates (units, Z-up)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HammerPos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl HammerPos {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Snap to 52-unit grid (1 MC block)
    pub fn snap_to_grid(&self) -> Self {
        Self {
            x: (self.x / BLOCK_SCALE).round() * BLOCK_SCALE,
            y: (self.y / BLOCK_SCALE).round() * BLOCK_SCALE,
            z: (self.z / BLOCK_SCALE).round() * BLOCK_SCALE,
        }
    }

    /// Convert to Minecraft coordinates
    pub fn to_mc(&self) -> McPos {
        McPos::new(
            self.x as f64 / BLOCK_SCALE as f64,
            -self.z as f64 / BLOCK_SCALE as f64, // Hammer -Z → MC Y
            self.y as f64 / BLOCK_SCALE as f64,  // Hammer Y → MC Z
        )
    }
}

/// Integer block position (Minecraft style)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn to_mc_pos(&self) -> McPos {
        McPos::new(self.x as f64, self.y as f64, self.z as f64)
    }

    pub fn to_hammer(&self) -> HammerPos {
        HammerPos::new(
            self.x as f32 * BLOCK_SCALE,
            self.y as f32 * BLOCK_SCALE,
            self.z as f32 * BLOCK_SCALE,
        )
    }
}

/// Block type registry - maps between MC and Portal 2 representations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u16)]
pub enum BlockType {
    Air = 0,
    Stone = 1,
    Dirt = 2,
    GrassBlock = 3,
    Cobblestone = 4,
    OakPlanks = 5,
    Glass = 6,
    Sand = 7,
    Gravel = 8,
    OakLog = 9,
    OakLeaves = 10,
    Bedrock = 11,
    Water = 12,
    Lava = 13,
    // Portal 2 specific
    PortalableSurface = 100,
    NonPortalableSurface = 101,
    GelBlue = 102,   // Repulsion gel
    GelOrange = 103, // Propulsion gel
    GelWhite = 104,  // Conversion gel
}

impl BlockType {
    /// Get the Portal 2 model path for this block type
    pub fn portal2_model(&self) -> &'static str {
        match self {
            BlockType::Air => "",
            BlockType::Stone | BlockType::Cobblestone | BlockType::Bedrock => {
                "models/hunter/blocks/cube025x025x025.mdl"
            }
            BlockType::Dirt | BlockType::GrassBlock | BlockType::Sand | BlockType::Gravel => {
                "models/hunter/blocks/cube025x025x025.mdl"
            }
            BlockType::OakPlanks | BlockType::OakLog => {
                "models/hunter/blocks/cube025x025x025.mdl"
            }
            BlockType::OakLeaves => "models/hunter/blocks/cube025x025x025.mdl",
            BlockType::Glass => "models/glass/glass_block.mdl",
            BlockType::Water | BlockType::Lava => "models/effects/vol_light001.mdl", // placeholder
            BlockType::PortalableSurface => "models/hunter/blocks/cube025x025x025.mdl",
            BlockType::NonPortalableSurface => "models/hunter/blocks/cube025x025x025.mdl",
            BlockType::GelBlue => "models/hunter/blocks/cube025x025x025.mdl",
            BlockType::GelOrange => "models/hunter/blocks/cube025x025x025.mdl",
            BlockType::GelWhite => "models/hunter/blocks/cube025x025x025.mdl",
        }
    }

    /// Get the Minecraft block ID string
    pub fn mc_id(&self) -> &'static str {
        match self {
            BlockType::Air => "minecraft:air",
            BlockType::Stone => "minecraft:stone",
            BlockType::Dirt => "minecraft:dirt",
            BlockType::GrassBlock => "minecraft:grass_block",
            BlockType::Cobblestone => "minecraft:cobblestone",
            BlockType::OakPlanks => "minecraft:oak_planks",
            BlockType::Glass => "minecraft:glass",
            BlockType::Sand => "minecraft:sand",
            BlockType::Gravel => "minecraft:gravel",
            BlockType::OakLog => "minecraft:oak_log",
            BlockType::OakLeaves => "minecraft:oak_leaves",
            BlockType::Bedrock => "minecraft:bedrock",
            BlockType::Water => "minecraft:water",
            BlockType::Lava => "minecraft:lava",
            BlockType::PortalableSurface => "minecraft:stone",
            BlockType::NonPortalableSurface => "minecraft:bedrock",
            BlockType::GelBlue => "minecraft:blue_concrete",
            BlockType::GelOrange => "minecraft:orange_concrete",
            BlockType::GelWhite => "minecraft:white_concrete",
        }
    }

    /// Is this block solid (collidable)?
    pub fn is_solid(&self) -> bool {
        !matches!(self, BlockType::Air | BlockType::Water | BlockType::Lava)
    }

    /// Can portals be placed on this block?
    pub fn is_portalable(&self) -> bool {
        matches!(self, BlockType::PortalableSurface | BlockType::Stone | BlockType::Cobblestone | BlockType::OakPlanks | BlockType::Glass)
    }

    /// Get default mass for physics (kg)
    pub fn mass_kg(&self) -> f32 {
        match self {
            BlockType::Air => 0.0,
            BlockType::Water | BlockType::Lava => 1000.0,
            BlockType::Glass => 50.0,
            BlockType::OakLeaves => 10.0,
            BlockType::Sand | BlockType::Gravel => 200.0,
            _ => 500.0, // stone, dirt, etc.
        }
    }
}

impl Default for BlockType {
    fn default() -> Self {
        BlockType::Stone
    }
}

/// All messages exchanged between Portal 2, Bridge, and Minecraft
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    // Handshake
    Hello {
        protocol_version: u32,
        session_id: SessionId,
        role: Role,
    },
    HelloAck {
        session_id: SessionId,
        accepted: bool,
        reason: Option<String>,
    },

    // Block operations
    PlaceBlock {
        seq: u64,
        pos: BlockPos,
        block_type: BlockType,
        placer: PlayerId,
    },
    BreakBlock {
        seq: u64,
        pos: BlockPos,
        breaker: PlayerId,
    },
    BlockUpdate {
        seq: u64,
        pos: BlockPos,
        block_type: BlockType,
    },

    // Inventory
    InventorySync {
        seq: u64,
        player: PlayerId,
        slots: Vec<InventorySlot>,
    },
    InventoryAction {
        seq: u64,
        player: PlayerId,
        action: InventoryActionType,
        slot: u8,
        item: Option<ItemStack>,
    },

    // Player state
    PlayerState {
        seq: u64,
        player: PlayerId,
        position: McPos,
        rotation: (f32, f32), // yaw, pitch
        health: f32,
        hunger: f32,
        gamemode: GameMode,
    },

    // World
    ChunkRequest {
        chunk_x: i32,
        chunk_z: i32,
    },
    ChunkData {
        chunk_x: i32,
        chunk_z: i32,
        blocks: Vec<(BlockPos, BlockType)>,
    },

    // Control
    Ping { timestamp: u64 },
    Pong { timestamp: u64 },
    Disconnect { reason: String },
}

/// Role in the protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    Portal2Server,
    Bridge,
    MinecraftClient,
}

/// Player identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub Uuid);

impl PlayerId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for PlayerId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Inventory slot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySlot {
    pub index: u8,
    pub item: Option<ItemStack>,
}

/// Item stack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemStack {
    pub item_type: BlockType, // simplified: blocks are items
    pub count: u8,
    pub damage: Option<u16>,
    pub nbt: Option<String>, // serialized NBT for enchantments, etc.
}

/// Inventory action types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InventoryActionType {
    Pickup,
    Drop,
    Move,
    Swap,
    Craft,
    Consume,
}

/// Game mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

/// Serialize message to bincode bytes
pub fn encode(msg: &Message) -> anyhow::Result<Vec<u8>> {
    Ok(bincode::serialize(msg)?)
}

/// Deserialize message from bincode bytes
pub fn decode(bytes: &[u8]) -> anyhow::Result<Message> {
    Ok(bincode::deserialize(bytes)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_conversion() {
        let mc = McPos::new(10.0, 64.0, -5.0);
        let hammer = mc.to_hammer();
        let mc_back = hammer.to_mc();
        
        assert!((mc.x - mc_back.x).abs() < 0.01);
        assert!((mc.y - mc_back.y).abs() < 0.01);
        assert!((mc.z - mc_back.z).abs() < 0.01);
    }

    #[test]
    fn test_block_snap() {
        let pos = HammerPos::new(100.0, 200.0, 300.0);
        let snapped = pos.snap_to_grid();
        assert_eq!(snapped.x % BLOCK_SCALE, 0.0);
        assert_eq!(snapped.y % BLOCK_SCALE, 0.0);
        assert_eq!(snapped.z % BLOCK_SCALE, 0.0);
    }

    #[test]
    fn test_message_codec() {
        let msg = Message::PlaceBlock {
            seq: 1,
            pos: BlockPos::new(10, 64, -5),
            block_type: BlockType::Stone,
            placer: PlayerId::new(),
        };
        let encoded = encode(&msg).unwrap();
        let decoded = decode(&encoded).unwrap();
        
        match decoded {
            Message::PlaceBlock { seq, pos, block_type, placer: _ } => {
                assert_eq!(seq, 1);
                assert_eq!(pos, BlockPos::new(10, 64, -5));
                assert_eq!(block_type, BlockType::Stone);
            }
            _ => panic!("Wrong message type"),
        }
    }
}