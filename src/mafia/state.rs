// Core game logic of the Morty bot

use mafia::roles::*;

enum GameState {
    Setup,
    PreGame,
    Summit,
    Harvest,
    Winter,
    PostGame,
}

// impl GameState for State {
//     fn tick(&mut self) {
//         //
//     }
// }

struct State {
    mode: GameState,
}
impl State {
    fn new() -> Self {
        State {
            mode: GameState::Setup,
        }
    }
}
// impl Setup {
//     fn pull_names() {}
// }
