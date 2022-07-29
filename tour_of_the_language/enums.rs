// enums.rs

// Copyright (c) Brandon Pacewic
// SPDX-License-Identifier: MIT

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum PlayerAction {
    Move {
        direction: Direction,
    },
    Wait,
    Attack(Direction)
}

fn main() {
    let simulated_player_action = PlayerAction::Move {
        direction: Direction::Up,
    };
    
    match simulated_player_action {
        PlayerAction::Wait => println!("The player is waiting..."),
        PlayerAction::Move { direction } => { 
            println!("The player is moving {:?}", direction) 
        },
        PlayerAction::Attack(direction) => println!("The player is attacking in the {:?} direction", direction),
    };
}