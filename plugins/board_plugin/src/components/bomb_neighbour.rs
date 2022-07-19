use bevy::prelude::Component;

/// Bomb neighbor component
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct BombNeighbour(u8);

impl BombNeighbour {
    #[inline]
    #[must_use]
    pub const fn new(count: u8) -> Self {
        Self(count)
    }
}
