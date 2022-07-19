use bevy::prelude::Vec3;
use serde::{Deserialize, Serialize};

/// Tile size options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileSize {
    /// Fixed tile size.
    Fixed(f32),
    /// Window adaptative tile size.
    Adaptive { min: f32, max: f32 },
}

impl Default for TileSize {
    #[inline]
    fn default() -> Self {
        Self::Adaptive {
            min: 10.0,
            max: 50.0,
        }
    }
}

/// Board position customization options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoardPosition {
    /// Centered board
    Centered(Vec3),
    /// Custom position
    Offset(Vec3),
}

impl Default for BoardPosition {
    #[inline]
    fn default() -> Self {
        Self::Centered(Vec3::default())
    }
}

/// Board generation options. Must be used as a resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardOptions {
    /// Tile map size.
    pub map_size: (usize, usize),
    /// bomb count.
    pub bomb_count: u16,
    /// Board world position.
    pub position: BoardPosition,
    /// Tile world size.
    pub tile_size: TileSize,
    /// Padding between tiles.
    pub tile_padding: f32,
    /// Does the board generate a safe place to start.
    pub safe_start: bool,
}

impl Default for BoardOptions {
    #[inline]
    fn default() -> Self {
        Self {
            map_size: (15, 15),
            bomb_count: 30,
            position: BoardPosition::default(),
            tile_size: TileSize::default(),
            tile_padding: 0.,
            safe_start: false,
        }
    }
}
