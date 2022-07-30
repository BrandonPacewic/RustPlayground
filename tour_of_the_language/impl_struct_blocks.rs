// impl_struct_blocks.rs

// Copyright (c) Brandon Pacewic
// SPDX-License-Identifier: MIT

struct Player {
    name: String,
    iq: u8,
    friends: u8,
}

impl Player {
    fn with_name(name: &str) -> Player {
        Player {
            name: name.to_string(),
            iq: 100,
            friends: 0
        }
    }

    fn get_friends(&self) -> u8 {
        self.friends
    }

    fn set_friends(&mut self, friends: u8) {
        self.friends = friends;
    }
}

fn main() {
    let mut player = Player::with_name("Brandon");
    player.set_friends(10);
    println!("{} has {} friends!", player.name, player.get_friends());
}