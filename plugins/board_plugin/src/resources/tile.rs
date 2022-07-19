#[cfg(feature = "debug")]
use colored::Colorize;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Bomb,
    BombNeighbor(u8),
    Empty,
}

impl Tile {
    #[must_use]
    pub const fn is_bomb(&self) -> bool {
        matches!(self, Self::Bomb)
    }

    #[must_use]
    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!(
            " {}",
            match self {
                Tile::Bomb => "*".bright_red(),
                Tile::BombNeighbor(v) => match v {
                    1 => "1".cyan(),
                    2 => "2".green(),
                    3 => "3".yellow(),
                    _ => v.to_string().red(),
                },
                Tile::Empty => " ".normal(),
            }
        )
    }
}
