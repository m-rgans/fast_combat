
use crate::console;
use super::DndGame;
use super::Error;

pub trait DndCommand {
    fn help_text(&self, long:bool) -> &'static str {"No help text is defined for this command."}
    fn perform(&self, statement:&console::ConsoleStatement, game:&mut DndGame) -> Result<(), Error>;
    fn get_auth_level(&self) -> cmd::PermissionLevel {return cmd::PermissionLevel::Controller}
    fn get_valid_flags(&self) -> Vec<String> {return vec![]}
}

//mod cmd;

pub mod cmd {

    #[derive(PartialEq, PartialOrd)]
    pub enum PermissionLevel {
        Spectator, // spectators
        Controller, // controls the character in question
        GameMaster, // dm only
    }

    //todo: auto arg validation
    // can have like fn rip_args -> <T> and T is a tuple or smth

    use super::{DndCommand, DndGame};
    use crate::console::{self, ConsoleStatement, parse_dice_phrase};
    
    use super::Error::*;

    pub fn get_command(name:&str) -> Option<Box<dyn DndCommand>> {

        return match name {
            HURT_CMD_NAME => Some(Box::new(HurtCmd{})),
            HELP_CMD_NAME => Some(Box::new(HelpCmd{})),
            LS_CHAR_NAME => Some(Box::new(LsCharCmd{})),
            _ => None
        }

    }

    struct HurtCmd;
    const HURT_CMD_NAME:&'static str = "hurt";
    impl DndCommand for HurtCmd {
        fn help_text(&self, long:bool) -> &'static str {
            if long {
                return "Hurt a character by an amount equivalent to the given dice roll.";
            }
            else {
                return "hurt character roll";
            }
        }
        fn perform(&self, statement:&console::ConsoleStatement, game:&mut DndGame) -> Result<(), super::Error> {

            if statement.arguments.len() != 2 {
                return Err(ArgCount);
            }

            let char = &statement.arguments[2];
            let Some(char) = game.get_character_mut(&char) else {return Err(NoSuchCharacter)};

            let roll = match parse_dice_phrase(&statement.arguments[1]) {
                Some(r) => r,
                None => return Err(ArgIncoherent),
            };

            char.hurt(roll.roll());

            return Ok(());

        }

        fn get_auth_level(&self) -> self::PermissionLevel {PermissionLevel::GameMaster}

    }

    struct HelpCmd;
    const HELP_CMD_NAME:&str = "help";
    impl DndCommand for HelpCmd {
        fn help_text(&self, long:bool) -> &'static str {
            if long {
                return "Provides a description of the given command. Pass -l to show the long version of a commands help text (as you have apparently just done).";
            }
            else {
                return "help [-l] command";
            }
        }

        fn perform(&self, statement:&console::ConsoleStatement, game:&mut DndGame) -> Result<(), super::Error> {
            if statement.arguments.len() != 1 {
                return Err(ArgCount);
            }
            else {
                if let Some(cmd) = get_command(&statement.arguments[0]) {
                    let long = statement.has_flag("l");
                    println!("{}", cmd.help_text(long));
                    return  Ok(());
                }
                else {
                    return Err(NoSuchAction);
                }
            }
        }

        fn get_auth_level(&self) -> self::PermissionLevel {PermissionLevel::Spectator}
    }

    struct LsCharCmd;
    const LS_CHAR_NAME:&str = "lschar";
    impl DndCommand for LsCharCmd {
        fn get_auth_level(&self) -> self::PermissionLevel {
            return PermissionLevel::Spectator;
        }

        fn help_text(&self, long:bool) -> &'static str {
            if long {
                return "Show stats about a character.";
            }
            else {
                return "lschar character";
            }
        }

        fn perform(&self, statement:&console::ConsoleStatement, game:&mut DndGame) -> Result<(), super::Error> {
            if let Some(char_name) = statement.arguments.get(0) {
                match game.get_character(char_name) {
                    Some(char) => {
                        println!("Name:{}\nHealth:{}", char.name, char.get_stat_block().hp);
                        return Ok(());
                    }
                    None => return Err(NoSuchCharacter),
                    
                }
            }
            else {
                return  Err(ArgIncoherent);
            }
        }
    }

    struct ActCmd;
    impl DndCommand for ActCmd {
        fn perform(&self, statement:&console::ConsoleStatement, game:&mut DndGame) -> Result<(), crate::game::Error> {
            Ok(())
        }
        fn get_auth_level(&self) -> self::PermissionLevel {PermissionLevel::Controller}
        fn help_text(&self, long:bool) -> &'static str {
            if long {
                "Perform an action as character."
            }
            else {
                "act character action_name option=value [target(s)]"
            }
        }
    }

}