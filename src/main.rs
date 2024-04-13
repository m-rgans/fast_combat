#[allow(dead_code)]

use std::io::{self, BufRead};

use game::DndGame;

mod console;

mod game;
mod roll;



fn main() {

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    
    let mut game: DndGame = DndGame::new();

    loop {
        let mut buffer = String::new();
        let _ = handle.read_line(&mut buffer);
        let cmd = console::ConsoleStatement::parse(&buffer);

        println!("Command:{cmd:?}");

        if cmd.command == "exit" {
            break;
        }
        else if cmd.command == "newgame" {
            game = DndGame::new_game_test();
        }
        else {
            game.do_command(&cmd);
        }
    }
    
}
