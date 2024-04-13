use std::{collections::HashMap};

use crate::roll::DiceRoll;

pub mod stat_names {
    pub const STR:&str = "strength";
    pub const DEX:&str = "dexterity";
    pub const CON:&str = "constitution";

    pub const INT:&str = "intelligence";
    pub const CHR:&str = "charisma";
    pub const WIS:&str = "wisdom";

    pub const MAX_HEALTH:&str = "max health";
    pub const HEALTH:&str = "health";
    pub const TEMP_HEALTH:&str = "temporary hp";
    pub const ARMOR_CLASS:&str = "armor class";
    //pub const SPEED:&str = "movement speed";

    pub const ANIMAL_HANDLING:&str = "animal handling";
    pub const ACROBATICS:&str = "acrobatics";
    pub const ARCANA:&str = "arcana";
    pub const ATHLETICS:&str = "athletics";
    pub const DECEPTION:&str = "deception";
    pub const HISTORY:&str = "history";
    pub const INSIGHT:&str = "insight";
    pub const INTIMIDATION:&str = "intimidation";
    pub const INVESTIGATION:&str = "investigation";
    pub const MEDICINE:&str = "medicine";
    pub const NATURE:&str = "nature";
    pub const PERCEPTION:&str = "perception";
    pub const PERFORMANCE:&str = "performance";
    pub const PERSUASION:&str = "persuasion";
    pub const RELIGION:&str = "religion";
    pub const SLEIGHT_OF_HAND:&str = "sleight of hand";
    pub const STEALTH:&str = "stealth";
    pub const SURVIVAL:&str = "survival";
}

trait Stat {
    fn get_bonus(&self, stat_block:&StatBlock) -> i32 {
        return score_to_bonus(self.get_score(stat_block));
    }
    fn get_score(&self, stat_block:&StatBlock) -> i32;
}

fn score_to_bonus(score:i32) -> i32 {
    return  score - 10 / 2;
}

#[derive(Hash,PartialEq, Eq, PartialOrd, Ord)]
pub enum ScoreEnum {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl Into<StatEnum> for ScoreEnum {
    fn into(self) -> StatEnum {
        return StatEnum::ScoreType(self);
    }
}

impl Stat for ScoreEnum {
    fn get_score(&self, stat_block:&StatBlock) -> i32 {
        return match self {
            Self::Strength => stat_block.str,
            Self::Dexterity => stat_block.dex,
            Self::Constitution => stat_block.con,
            Self::Intelligence => stat_block.int,
            Self::Wisdom => stat_block.wis,
            Self::Charisma => stat_block.chr,
        }
    }
}

#[derive(Clone, Copy)]
#[derive(Hash,PartialEq, Eq, PartialOrd, Ord)]
pub enum SkillEnum {
    AnimalHandling,
    Acrobatics,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
}


impl SkillEnum {
    fn get_base_score(&self) -> ScoreEnum {
        use SkillEnum::*;
        match self {
            AnimalHandling | Insight | Medicine | Perception | Survival => ScoreEnum::Wisdom,
            Acrobatics | SleightOfHand | Stealth => ScoreEnum::Dexterity,
            Arcana | History | Investigation | Nature | Religion => ScoreEnum::Intelligence,
            Athletics  => ScoreEnum::Strength,
            Deception | Intimidation |Performance | Persuasion => ScoreEnum::Charisma,
        }
    }
}

impl Stat for SkillEnum {
    fn get_bonus(&self, stat_block:&StatBlock) -> i32 {
        let base = self.get_base_score();
        let proficiency_bonus = if stat_block.is_proficient(*self) {stat_block.proficiency_bonus} else {0};
        return base.get_bonus(stat_block) + proficiency_bonus;
    }
    fn get_score(&self, stat_block:&StatBlock) -> i32 {
        let base = self.get_base_score();
        return base.get_score(stat_block);
    }
}

impl Into<StatEnum> for SkillEnum {
    fn into(self) -> StatEnum {
        return StatEnum::SkillType(self);
    }
}

pub enum StatEnum {
    ScoreType(ScoreEnum),
    SkillType(SkillEnum),
}

impl Stat for StatEnum {
    fn get_score(&self, stat_block:&StatBlock) -> i32 {
        return match self {
            Self::ScoreType(a) => a.get_score(stat_block),
            Self::SkillType(a) => a.get_score(stat_block),
        }
    }
    fn get_bonus(&self, stat_block:&StatBlock) -> i32 {
        return match self {
            Self::ScoreType(a) => a.get_bonus(stat_block),
            Self::SkillType(a) => a.get_bonus(stat_block),
        }
    }
}

pub struct StatBlock {
    level:i32,

    pub str:i32,
    pub dex:i32,
    pub con:i32,
    pub int:i32,
    pub chr:i32,
    pub wis:i32,

    pub proficiency_bonus:i32,
    proficient:HashMap<SkillEnum, ()>,

    pub hp:i32,
    pub max_hp:i32,
    pub temp_hp:i32,

    pub armor_class:i32,

    pub speed:i32,

}

impl StatBlock {

    pub fn new_default() -> Self {
        return  Self {
            level:4,
            str: 10,
            dex:10,
            con:10,
            int:10,
            chr:10,
            wis:10,
            proficiency_bonus:2,
            proficient:HashMap::new(),
            hp:25,
            max_hp:30,
            temp_hp:0,
            speed:30,
            armor_class:10,
        };
    }

    fn is_proficient(&self, skill:SkillEnum) -> bool  {
        return self.proficient.contains_key(&skill);
    }

    pub fn get_score(&self, stat:&dyn Stat) -> i32 {
        return stat.get_score(self);
    }

    pub fn get_bonus(&self, stat:&dyn Stat) -> i32 {
        return stat.get_bonus(self);
    }

    pub fn skill_check(&self, stat:&dyn Stat) -> i32 {
        let bonus = self.get_bonus(stat);
        return DiceRoll::d20().roll() + bonus;
    }

}