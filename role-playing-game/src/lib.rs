// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self {
            Player {
                health: 0,
                mana: None,
                level,
            } => Some(Player {
                health: 100,
                mana: None,
                level: *level,
            }),
            Player {
                health: 0,
                mana: Some(i32),
                level,
            } => Some(Player {
                health: 100,
                mana: Some(100),
                level: *level,
            }),
            Player {
                health: 1..,
                mana: _,
                level: _,
            } => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self {
            Player {
                health: _,
                mana: None,
                level: _,
            } => {
                self.health = self.health.saturating_sub(mana_cost);
                0
            }
            Player {
                health: _,
                mana: Some(i32),
                level: _,
            } => {
                if self.mana.unwrap() >= mana_cost {
                    *self.mana.as_mut().unwrap() -= mana_cost;
                    mana_cost * 2
                } else {
                    0
                }
            }
        }
    }
}
