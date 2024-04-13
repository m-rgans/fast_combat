use stat::StatBlock;

type CharId = i32;

mod stat;
mod char_table;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum CharState {
    Alive,
    Incapacitated, // immobile, but stable
    DeathSave,
    Dead,
    Undead,
}

pub struct Character {
    pub name:String,
    state:CharState,
    stat_block:StatBlock,
}

impl Character {

    pub fn heal(&mut self, amt:i32) {

        if amt < 0 {self.hurt(amt * -1);return}
        if amt == 0 {return}

        self.stat_block.hp = (amt as i32 + self.stat_block.hp).min(self.stat_block.max_hp);
        if self.state == CharState::DeathSave {
            self.state = CharState::Incapacitated;
        }
    }

    pub fn hurt(&mut self, amt:i32) {

        if amt < 0 {self.heal(amt * -1);return}

        self.stat_block.hp = (self.stat_block.hp - amt as i32);

        if self.stat_block.hp < 0 {
            if self.stat_block.hp < self.stat_block.max_hp * -2 {
                self.state = CharState::Dead;
            }
            else {
                self.state = CharState::DeathSave;
            }
        }
    }

    pub fn set_health(&mut self, amt:i32) {
        let dif = amt - self.stat_block.hp;
        self.heal(amt);
    }

    fn on_ressurect (&mut self) {

    }

    fn on_death(&mut self) {
        print!("Character {} died!", self.name);
    }

    pub fn get_stat_block(&self) -> &StatBlock {
        return &self.stat_block;
    }

    pub fn make_hudson() ->Self {
        return Self {
            name: "Hudson".to_string(),
            state:CharState::Alive,
            stat_block : StatBlock::new_default(),
        };
    }

}