use crate::bounds::Bounds;
use crate::{Coordinates, TileMap};
use bevy::{
    prelude::{Entity, Vec2, Window},
    utils::HashMap,
};

#[derive(Debug)]
pub struct Board {
    pub entity: Entity,
    pub tile_map: TileMap,
    pub covered_tiles: HashMap<Coordinates, Entity>,
    pub bounds: Bounds,
    pub tile_size: f32,
}

impl Board {
    #[inline]
    #[must_use]
    pub const fn new(
        entity: Entity,
        tile_map: TileMap,
        covered_tiles: HashMap<Coordinates, Entity>,
        bounds: Bounds,
        tile_size: f32,
    ) -> Self {
        Self {
            entity,
            tile_map,
            covered_tiles,
            bounds,
            tile_size,
        }
    }

    /// Translates a mouse position to board coordinates
    #[inline]
    #[must_use]
    pub fn mouse_position(&self, window: &Window, position: Vec2) -> Option<Coordinates> {
        // Window to world space
        let window_size = Vec2::new(window.width(), window.height());
        let position = position - window_size * 0.5;

        // Bounds check
        if !self.bounds.in_bounds(position) {
            return None;
        }
        // World space to board space
        let coordinates = position - self.bounds.mins;
        Some(Coordinates {
            x: (coordinates.x / self.tile_size) as u16,
            y: (coordinates.y / self.tile_size) as u16,
        })
    }

    /// Retrieves a covered tile entity
    #[must_use]
    #[inline]
    pub fn tile_to_uncover(&self, coords: &Coordinates) -> Option<&Entity> {
        self.covered_tiles.get(coords)
    }

    /// We try to uncover a tile, returning the entity
    #[must_use]
    #[inline]
    pub fn try_uncover_tile(&mut self, coords: &Coordinates) -> Option<Entity> {
        self.covered_tiles.remove(coords)
    }

    /// We retrieve the adjacent covered tile entities of `coord`
    #[must_use]
    #[inline]
    pub fn adjacent_covered_tiles(&self, coord: Coordinates) -> Vec<Entity> {
        TileMap::safe_square_at(coord)
            .filter_map(|c| self.covered_tiles.get(&c))
            .copied()
            .collect()
    }
}
