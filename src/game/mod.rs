use core::fmt;
use std::{collections::HashMap, default, hash::Hash, vec};

pub mod character;
use character::Character;

pub type CharId = String;

struct Player {
    game_master:bool,
    controls:Vec<CharId>,
}

impl Player {
    fn new_as_dm() -> Self {
        return Self {
            game_master:true,
            controls:Vec::new(),
        };
    }
    fn new_as_player() -> Self {
        return Self {
            game_master:false,
            controls:Vec::new(),
        };
    }
    fn new_with_char(char:&CharId) -> Self {
        return Self {
            game_master:false,
            controls:vec![char.clone()],
        };
    }
}

struct Combat {
    combatants:Vec<CharId>,
    turn_order:Vec<u32>,
    turn_number:i32,
}

impl Combat {
    
}

#[derive(std::fmt::Debug)]
pub enum Error {
    NoSuchCharacter,
    NoSuchAction,
    DuplicateName,
    NoSuchStat,
    Usage,
    DicePhraseInvalid,

    ArgCount,
    ArgIncoherent,
    NoSuchFlag,

    NoSpellSlots,
    
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct DndGame {
    characters:HashMap<String, Character>,
    combat_session:Option<Combat>,
}

use crate::{console::{self, ConsoleStatement}, roll::DiceRoll};

impl DndGame {

    pub fn new() -> Self {
        return  Self {
            characters:HashMap::new(),
            combat_session:None,
        };
    }

    pub fn new_game_test() -> Self {
        let hudson = Character::make_hudson();
        return Self {
            characters: HashMap::from([
                ("hudson".to_string(), hudson)
            ]),
            ..Self::new()
        };
    }

    pub fn do_line_user(&mut self, line:&str) {

    }

    pub fn do_line(&mut self, line:&str) {
        let cmd = console::ConsoleStatement::parse(line);
        self.do_command(&cmd);
    }

    pub fn do_command(&mut self, cmd:&ConsoleStatement) {
        if let Some(cmd_vtable) = cmd::get_command(&cmd.command) {
            let err = cmd_vtable.as_ref().perform(cmd, self);
            if let Err(e) = err {
                println!("Error:{}.", e.to_string());
            }
        }
        else {
            println!("{}: No such command.", cmd.command);
        }
    }

    fn hurt_character(&mut self, char:&CharId, amt:DiceRoll) -> Result< (), Error> {
        if let Some(char) = self.get_character_mut(char) {
            char.hurt(amt.roll());
            return Ok(());
        }
        else {
            return Err(Error::NoSuchCharacter);
        }
    }

    fn charid_is_valid(&self, char:&CharId) -> bool {
        return self.characters.contains_key(char);
    }

    fn get_character(&mut self, char:&CharId) -> Option<&Character> {
        return self.characters.get(char);
    }

    fn get_character_mut(&mut self, char:&CharId) -> Option<&mut Character> {
        return  self.characters.get_mut(char);
    }

    pub fn add_character(&mut self, char:Character) -> Result<CharId, Error> {
        let name = char.name.clone();

        if self.characters.contains_key(&name) {
            return  Err(Error::DuplicateName);
        }

        self.characters.insert(name.clone(),char);
        return Ok(name);
    }

}

impl Default for DndGame {
    fn default() -> Self {
        return Self {
            characters: HashMap::new(),
            combat_session:None,
        };
    }
}

pub mod table_commands;
use table_commands::cmd;

pub mod action;