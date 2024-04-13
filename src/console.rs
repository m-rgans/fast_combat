use std::{collections::HashMap, default, env::Args, hash::Hash, io};

use crate::roll::{self, DiceRoll};

#[derive(Debug)]
pub struct ConsoleStatement {
    pub command:String,
    pub arguments:Vec<String>,
    pub options:HashMap<String, String>,
}

impl ConsoleStatement {
    pub fn new() -> Self {
        return Self {
            command: "".to_owned(),
            arguments:Vec::new(),
            options:HashMap::new(),
        }
    }
    pub fn get_cin() -> Self {
        let mut buffer = String::new();
        if io::stdin().read_line(&mut buffer).is_ok() {
            return Self::parse(&buffer);
        }
        else {
            return Self {
                command:"FAILED_TO_GET".to_string(),
                ..Self::default()
            };
        }
    }
    pub fn parse(string: &str) -> Self {
        return parse_con_statement(string);
    }

    pub fn has_flag(&self, flag: &str) -> bool {
        return self.options.contains_key(flag);
    }

    pub fn get_option(&self, opt:&str) -> Option<&String> {
        return self.options.get(opt);
    }

    pub fn flags_valid(&self, acceptable_flags:&Vec<String>) -> bool {

        let mut map:HashMap<String,()> = HashMap::with_capacity(acceptable_flags.len());

        for valid in acceptable_flags.iter() {
            map.insert(valid.clone(), ());
        }

        for flag in self.options.iter() {
            let (k, _) = flag;
            if ! map.contains_key(k) {
                return false;
            }
        }

        return true;

    }

}

impl Default for ConsoleStatement {
    fn default() -> Self {
        return Self {
            command: "None.".to_string(),
            arguments: Vec::new(),
            options:HashMap::new(),
        };
    }
}

fn parse_con_statement(statement: &str) -> ConsoleStatement {

    const FLAG_MARKER:char = '-';

    let mut stmt = ConsoleStatement::new();

    for token in statement.split_whitespace() {
        if stmt.command == "" {
            stmt.command = token.to_string();
        }
        else if token.chars().nth(0).unwrap() == FLAG_MARKER {
            let key = token[1..].to_string();
            let value = token[1..token.find("=").unwrap_or(1)].to_string();

            stmt.options.insert(key, value);

        }
        else {
            stmt.arguments.push(token.to_string());
        }
    }

    return stmt;

}

mod con_seperator_test {
    use super::ConsoleStatement;

    #[test]
    fn con_test_01() {
        let res = ConsoleStatement::parse("lsmod -flag1a i4d6+3");
        assert_eq!(res.command, "lsmod");
        assert_eq!(res.arguments, vec!["i4d6+3"]);
        assert!(res.get_option("flag1a").is_some());
    }

}

pub fn parse_dice_phrase(str:&str) -> Option<crate::roll::DiceRoll> {
    let mut accumulator:String = String::new();

    let mut dice_count:Option<i32> = None;
    let mut face_count:Option<i32> = None;
    let mut bonus:Option<i32> = None;

    let mut bonus_multiplier = 1;

    for char in str.chars() {
        if char.is_whitespace() {
            continue;
        }
        else if char == 'd' {

            if let Ok(v) = accumulator.parse::<i32>() {
                dice_count = Some(v);
            }
            // If no count is specified, assume 1.
            else if accumulator.is_empty(){
                dice_count = Some(1);
            }
            else {
                return None;
            }

            accumulator.clear();
        }
        else if char == '+' || char == '-' {
            let mul = if char == '-' { -1 } else { 1 };
            bonus_multiplier = mul;
            
            if let Ok(v) = accumulator.parse::<i32>() {
                
                face_count = Some(v);
            }
            else if accumulator.is_empty() {
                // No stated dice.
            }
            else {
                return None;
            }

            accumulator.clear();
        }
        else if char.is_numeric() {
            accumulator.push(char);
        }
        else {
            return None;
        }
    }

    // at end, accumulator into bonus or face count
    if let Ok(v) = accumulator.parse::<i32>() {

        // If we are still expecting a face count, put it in
        if dice_count.is_some() && face_count.is_none() {
            face_count = Some(v);
        }
        else {
            bonus = Some(v);
        }

    }

    return Some(crate::roll::DiceRoll {
        bonus:bonus.unwrap_or(0) * bonus_multiplier,
        faces:face_count.unwrap_or(0),
        dice:dice_count.unwrap_or(0),
        ..DiceRoll::default()
    });

}

mod dice_parse_test {
    use crate::roll::DiceRoll;

    use super::parse_dice_phrase;

    fn dice_assert(str: &str) -> DiceRoll{
        match parse_dice_phrase(str) {
            Some(v) => return v,
            None => panic!("Did not produce valid dice roll.")
        }
    }

    fn dice_value_assert(other:&DiceRoll, count:i32, faces:i32, bonus:i32) {
        assert_eq!(&crate::roll::DiceRoll::all(count, faces, bonus), other);
    }

    #[test]
    fn dice_parse_1() {
        let test_str = " 1d3 + 6";
        let dice = dice_assert(test_str);
        dice_value_assert(&dice, 1,3,6);
    }

    #[test]
    fn dice_parse_no_count() {
        let test_str = " d5 +2";
        let dice = dice_assert(test_str);
        dice_value_assert(&dice, 1,5,2);
    }

    #[test]
    fn dice_parse_negative_bonus() {
        let test_str = "2d8- 5";
        dice_value_assert(&dice_assert(test_str), 2,8,-5);
    }

    #[test]
    fn dice_parse_bonus_only() {
        let test_str = "- 5";
        dice_value_assert(&dice_assert(test_str), 0,0,-5);
    }

    #[test]
    fn dice_only() {
        let test_str = "1d5";
        dice_value_assert(&dice_assert(test_str), 1,5,0);
    }

}