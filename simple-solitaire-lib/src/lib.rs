use crate::cards::Suit::Clubs;
use crate::games::{FreeCell, Game};

mod cards;
mod board;
mod games;

pub struct GameEntry {
    pub name: &'static str,
    pub creator: fn() -> Game,
}

macro_rules! game_entry {
    ($game:ident) => {
        GameEntry {
            name: stringify!($game),
            creator: || { Game::create_with_logic(Box::new($game::default()))}
        }
    };
}

const GAME_ENTRIES: [GameEntry; 1]  = [
    game_entry!(FreeCell),
];

pub fn get_game_entries() -> &'static [GameEntry] {
    &GAME_ENTRIES
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod prelude {
    pub mod cards {
        pub use crate::cards::Card;
        pub use crate::cards::Suit;
        pub use crate::cards::Rank;
        pub use crate::board::CardLocation;
        pub use crate::board::BoardLocation;
        pub use crate::board::Pile;
        pub use crate::board::PileFlow;
        pub use crate::board::BoardItemStyle;
    }
    pub mod games {
        pub use crate::GameEntry;
        pub use crate::get_game_entries;
        pub use crate::games::{Game, GameEvent, FreeCell};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
