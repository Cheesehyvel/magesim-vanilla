use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct Stats {
    pub int: f64,
    pub spi: f64,
    pub mp5: f64,
    pub crit: f64,
    pub hit: f64,
    pub sp: f64,
    pub sp_arcane: f64,
    pub sp_fire: f64,
    pub sp_frost: f64,
    pub sp_nature: f64,
    pub sp_shadow: f64,
    pub spell_penetration: f64,
}

impl Stats {
    pub fn reset(&mut self) {
        self.int = 0.0;
        self.spi = 0.0;
        self.mp5 = 0.0;
        self.crit = 0.0;
        self.hit = 0.0;
        self.sp = 0.0;
        self.sp_arcane = 0.0;
        self.sp_fire = 0.0;
        self.sp_frost = 0.0;
        self.sp_nature = 0.0;
        self.sp_shadow = 0.0;
        self.spell_penetration = 0.0;
    }
}

impl std::ops::Add for Stats {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            int: self.int + other.int,
            spi: self.spi + other.spi,
            mp5: self.mp5 + other.mp5,
            crit: self.crit + other.crit,
            hit: self.hit + other.hit,
            sp: self.sp + other.sp,
            sp_arcane: self.sp_arcane + other.sp_arcane,
            sp_fire: self.sp_fire + other.sp_fire,
            sp_frost: self.sp_frost + other.sp_frost,
            sp_nature: self.sp_nature + other.sp_nature,
            sp_shadow: self.sp_shadow + other.sp_shadow,
            spell_penetration: self.spell_penetration + other.spell_penetration,
        }
    }
}

impl std::ops::AddAssign for Stats {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            int: self.int + other.int,
            spi: self.spi + other.spi,
            mp5: self.mp5 + other.mp5,
            crit: self.crit + other.crit,
            hit: self.hit + other.hit,
            sp: self.sp + other.sp,
            sp_arcane: self.sp_arcane + other.sp_arcane,
            sp_fire: self.sp_fire + other.sp_fire,
            sp_frost: self.sp_frost + other.sp_frost,
            sp_nature: self.sp_nature + other.sp_nature,
            sp_shadow: self.sp_shadow + other.sp_shadow,
            spell_penetration: self.spell_penetration + other.spell_penetration,
        }
    }
}

impl std::ops::Sub for Stats {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            int: self.int - other.int,
            spi: self.spi - other.spi,
            mp5: self.mp5 - other.mp5,
            crit: self.crit - other.crit,
            hit: self.hit - other.hit,
            sp: self.sp - other.sp,
            sp_arcane: self.sp_arcane - other.sp_arcane,
            sp_fire: self.sp_fire - other.sp_fire,
            sp_frost: self.sp_frost - other.sp_frost,
            sp_nature: self.sp_nature - other.sp_nature,
            sp_shadow: self.sp_shadow - other.sp_shadow,
            spell_penetration: self.spell_penetration - other.spell_penetration,
        }
    }
}

impl std::ops::SubAssign for Stats {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            int: self.int - other.int,
            spi: self.spi - other.spi,
            mp5: self.mp5 - other.mp5,
            crit: self.crit - other.crit,
            hit: self.hit - other.hit,
            sp: self.sp - other.sp,
            sp_arcane: self.sp_arcane - other.sp_arcane,
            sp_fire: self.sp_fire - other.sp_fire,
            sp_frost: self.sp_frost - other.sp_frost,
            sp_nature: self.sp_nature - other.sp_nature,
            sp_shadow: self.sp_shadow - other.sp_shadow,
            spell_penetration: self.spell_penetration - other.spell_penetration,
        }
    }
}

impl std::ops::Mul<f64> for Stats {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            int: self.int * rhs,
            spi: self.spi * rhs,
            mp5: self.mp5 * rhs,
            crit: self.crit * rhs,
            hit: self.hit * rhs,
            sp: self.sp * rhs,
            sp_arcane: self.sp_arcane * rhs,
            sp_fire: self.sp_fire * rhs,
            sp_frost: self.sp_frost * rhs,
            sp_nature: self.sp_nature * rhs,
            sp_shadow: self.sp_shadow * rhs,
            spell_penetration: self.spell_penetration * rhs,
        }
    }
}