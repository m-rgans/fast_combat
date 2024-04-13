use std::default;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct DiceRoll {
    pub faces:i32,
    pub dice:i32,
    pub bonus:i32,
    pub advantage:i32,
}

pub fn roll(dice_count:i32, face_count:i32, bonus:i32) -> i32 {
    let mut sum:i32 = bonus;
    for i in 0..dice_count {
        sum += (rand::random::<i32>() % face_count) + 1;
    }
    return sum;
}

impl Default for DiceRoll {
    fn default() -> Self {
        return Self {
            faces:0,
            dice:0,
            bonus:0,
            advantage:0,
        };
    }
}

impl DiceRoll {

    pub fn with_advantage(&self, amt:i32) -> Self {
        let mut v = self.clone();
        v.advantage = amt;
        return  v;
    }

    pub fn roll (&self) -> i32 {
        return roll(self.dice, self.faces, self.bonus);
    }

    pub fn roll_crit(&self) -> i32 {
        return roll(self.dice, self.faces * 2, self.bonus);
    }

    pub fn roll_advantage(&self, advantage:i32) -> i32 {

        let mut min:i32 = i32::MAX;
        let mut max:i32 = 0;

        for i in 0..self.advantage + 1{
            let roll = self.roll();
            if roll > max {max = roll}
            else if roll < min {min = roll}
        }

        if self.advantage < 0 {
            return min;
        }
        else {
             return max;
        }
    }

    pub fn d20() -> Self {
        return  Self {
            faces:20,
            dice:1,
            ..Default::default()
        };
    }

    pub fn d20_with_bonus(bonus:i32) -> Self {
        return  Self {
            faces:20,
            dice:1,
            bonus:bonus,
            advantage:0,
        };
    }

    pub fn all(dice_count:i32, face_count:i32, bonus:i32) -> Self {
        return Self {
            faces:face_count,
            dice:dice_count,
            bonus:bonus,
            advantage:0,
        };
    }

    pub fn dice_only(dice_count:i32, face_count:i32) -> Self {
        return Self {
            faces:face_count,
            dice:dice_count,
            ..Default::default()
        };
    }

    pub fn flat_number(amt:i32) -> Self {
        return Self {
            bonus:amt,
            ..Default::default()
        };
    }
}
