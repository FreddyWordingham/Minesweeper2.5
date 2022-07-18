use bevy::prelude::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    pub mins: Vec2,
    pub size: Vec2,
}

impl Bounds {
    pub fn in_bounds(&self, coords: Vec2) -> bool {
        coords.x >= self.mins.x
            && coords.y >= self.mins.y
            && coords.x <= self.mins.x + self.size.x
            && coords.y <= self.mins.y + self.size.y
    }
}
