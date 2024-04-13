use super::table_commands::DndCommand;
use super::Error;
use super::CharId;
use super::DndGame;
use crate::console::ConsoleStatement;
use std::collections::HashMap;
pub trait CharAction {
    fn perform(&self, game:&mut DndGame, user:CharId, targets:Vec<CharId>, options:HashMap<String,String>) -> Result<String,Error>;
    fn about_text(&self) -> &'static str {"No description is defined for this action."}
    fn get_options(&self) -> HashMap<String, String> {HashMap::new()}
}

/*
    Idea:
    use this for all commands
    use hashmaps for flags and options
*/

pub fn get_action(name:&str) -> Option<Box<dyn CharAction>> {

    const IMPROV_ATTACK:&str = "improvattack";

    match name {
        IMPROV_ATTACK => {
            struct ImprovisedAttack;
            impl CharAction for ImprovisedAttack {
                fn about_text(&self) -> &'static str {
                    "Attack with your fists or an improvised weapon"
                }
                fn perform(&self, game:&mut DndGame, user:CharId, targets:Vec<CharId>, options:HashMap<String,String>) -> Result<String,Error> {
                    let Some(user) = game.get_character(&user) else {return Err(Error::NoSuchCharacter)};
                    let str = user.get_stat_block().str;

                    

                    return Ok("".to_string());
                }
            }
            return Some(Box::new(ImprovisedAttack{}));
        }
        _ => None,
    }

}