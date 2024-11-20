use crate::aura;
use crate::common::School;
use crate::config::Config;
use crate::cooldown;
use crate::event::Event;
use crate::event::EventType;
use crate::macros::console_log;
use crate::sim::Sim;
use crate::spell;
use crate::stats::Stats;
use crate::unit::Unit;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use serde::{Serialize, Deserialize};

const TALENT_ARCANE_SUBLETY: usize = 1;
const TALENT_ARCANE_FOCUS: usize = 2;
const TALENT_IMP_ARCANE_MISSILES: usize = 3;
const TALENT_WAND_SPEC: usize = 4;
const TALENT_MAGIC_ABSORPTION: usize = 5;
const TALENT_ARCANE_CONCENTRATION: usize = 6;
const TALENT_MAGIC_ATTUNEMENT: usize = 7;
const TALENT_IMP_ARCANE_EXPLOSION: usize = 8;
const TALENT_ARCANE_RESILIENCE: usize = 9;
const TALENT_IMP_MANA_SHIELD: usize = 10;
const TALENT_IMP_COUNTERSPELL: usize = 11;
const TALENT_ARCANE_MEDITATION: usize = 12;
const TALENT_PRESENCE_OF_MIND: usize = 13;
const TALENT_ARCANE_MIND: usize = 14;
const TALENT_ARCANE_INSTABILITY: usize = 15;
const TALENT_ARCANE_POWER: usize = 16;

const TALENT_IMP_FIREBALL: usize = 17;
const TALENT_IMPACT: usize = 18;
const TALENT_IGNITE: usize = 19;
const TALENT_FLAME_THROWING: usize = 20;
const TALENT_IMP_FIRE_BLAST: usize = 21;
const TALENT_INCINERATE: usize = 22;
const TALENT_IMP_FLAMESTRIKE: usize = 23;
const TALENT_PYROBLAST: usize = 24;
const TALENT_BURNING_SOUL: usize = 25;
const TALENT_IMP_SCORCH: usize = 26;
const TALENT_IMP_FIRE_WARD: usize = 27;
const TALENT_MASTER_OF_ELEMENTS: usize = 28;
const TALENT_CRITICAL_MASS: usize = 29;
const TALENT_BLAST_WAVE: usize = 30;
const TALENT_FIRE_POWER: usize = 31;
const TALENT_COMBUSTION: usize = 32;

const TALENT_FROST_WARDING: usize = 33;
const TALENT_IMP_FROSTBOLT: usize = 34;
const TALENT_ELEMENTAL_PRECISION: usize = 35;
const TALENT_ICE_SHARDS: usize = 36;
const TALENT_FROSTBITE: usize = 37;
const TALENT_IMP_FROST_NOVA: usize = 38;
const TALENT_PERMAFROST: usize = 39;
const TALENT_PIERCING_ICE: usize = 40;
const TALENT_COLD_SNAP: usize = 41;
const TALENT_IMP_BLIZZARD: usize = 42;
const TALENT_ARCTIC_REACH: usize = 43;
const TALENT_FROST_CHANNELING: usize = 44;
const TALENT_SHATTER: usize = 45;
const TALENT_ICE_BLOCK: usize = 46;
const TALENT_IMP_CONE_OF_COLD: usize = 47;
const TALENT_WINTERS_CHILL: usize = 48;
const TALENT_ICE_BARRIER: usize = 49;

pub struct Mage {
    pub id: i32,
    pub name: String,
    pub level: i32,
    pub config: Option<Config>,
    pub mana: f64,
    pub base_mana: f64,
    pub t_gcd: f64,
    pub t_mana_spent: f64,
    pub stats: Stats,
    pub talents: Vec<u8>,
    pub auras: aura::Auras,
    pub cooldowns: cooldown::Cooldowns,
    pub rng: ChaCha8Rng,
}

impl Mage {
    pub fn new() -> Self {
        Self {
            id: 1,
            mana: 0.0,
            config: None,
            level: 60,
            name: String::from("Player"),
            base_mana: 0.0,
            t_gcd: 0.0,
            t_mana_spent: 0.0,
            stats: Stats::default(),
            talents: vec![],
            auras: aura::Auras::default(),
            cooldowns: cooldown::Cooldowns::default(),
            rng: ChaCha8Rng::from_entropy(),
        }
    }
}

impl Unit for Mage {
    fn id(&self) -> i32 {
        return self.id;
    }

    fn name(&self) -> String {
        return self.name.clone();
    }

    fn reset(&mut self) {
        if self.level == 25 {
            self.base_mana = 481.0;
        } else if self.level == 40 {
            self.base_mana = 853.0;
        } else if self.level == 50 {
            self.base_mana = 1048.0;
        } else if self.level == 60 {
            self.base_mana = 1213.0;
        }

        self.mana = self.max_mana();
        self.auras.reset();
    }

    fn set_config(&mut self, config: Config) {
        self.config = Some(config);
    }

    fn new_rng(&mut self, rng_seed: u64) {
        if rng_seed == 0 {
            self.rng = ChaCha8Rng::from_entropy();
        } else {
            self.rng = ChaCha8Rng::seed_from_u64(rng_seed);
        }
    }
    
    fn level(&self) -> i32 {
        return self.level;
    }

    fn max_mana(&self) -> f64 {
        // Subtract 280 because the first 20 intellect only gives 1 mana instead of 15
        return self.base_mana + self.stats.int * 15.0 - 280.0;
    }

    fn current_mana(&self) -> f64 {
        return self.mana;
    }

    fn mod_mana(&mut self, mana: f64, t: f64) {
        self.mana+= mana;

        if self.mana > self.max_mana() {
            self.mana = self.max_mana();
        } else if self.mana < 0.0 {
            self.mana = 0.0;
        }

        if mana < 0.0 {
            self.t_mana_spent = t;
        }
    }

    fn mana_per_second(&self, t: f64) -> f64 {
        let mut mps = self.mp5() / 5.0;
        let mut spi = self.spirit_regen();
        let mut while_casting = 0.0;

        if self.config.as_ref().unwrap().mage_armor {
            while_casting+= 0.3;
        }
        if self.talents[TALENT_ARCANE_MEDITATION] > 0 {
            while_casting+= 0.05 * (self.talents[TALENT_ARCANE_MEDITATION] as f64);
        }
        if self.t_mana_spent + 5.0 < t {
            while_casting = 1.0;
        }
        if self.auras.has(aura::INNERVATE) {
            spi*= 5.0;
            while_casting = 1.0;
        }
        if self.auras.has(aura::EVOCATION) {
            spi*= 16.0;
            while_casting = 1.0;
        }
        if self.auras.has(aura::BLUE_DRAGON) {
            while_casting = 1.0;
        }

        mps+= while_casting.min(1.0) * spi;

        return mps;
    }

    fn spirit_regen(&self) -> f64 {
        let spi = self.spirit();
        return 0.25 * spi.min(50.0) + 0.125 * (spi.max(50.0) - 50.0);
    }

    fn mp5(&self) -> f64 {
        return self.stats.mp5 + self.auras.stats.mp5;
    }

    fn intellect(&self) -> f64 {
        return self.stats.int + self.auras.stats.int;
    }

    fn spirit(&self) -> f64 {
        return self.stats.spi + self.auras.stats.spi;
    }

    fn spell_power(&self, school: School) -> f64 {
        let mut sp = self.stats.sp + self.auras.stats.sp;

        match school {
            School::Arcane => {
                sp+= self.stats.sp_arcane + self.auras.stats.sp_arcane;
            },
            School::Fire => {
                sp+= self.stats.sp_fire + self.auras.stats.sp_fire;
            },
            School::Frost => {
                sp+= self.stats.sp_frost + self.auras.stats.sp_frost;
            },
            School::Nature => {
                sp+= self.stats.sp_nature + self.auras.stats.sp_nature;
            },
            School::Shadow => {
                sp+= self.stats.sp_shadow + self.auras.stats.sp_shadow;
            },
            School::None => {
                // Do nothing
            }
        }

        return sp;
    }

    fn spell_penetration(&self, school: School) -> f64 {
        return self.stats.spell_penetration + self.auras.stats.spell_penetration;
    }

    fn spell_hit_chance(&self, spell: &spell::Spell) -> f64 {
        let mut hit = self.stats.hit + self.auras.stats.hit;

        if spell.school == School::Arcane && self.talents[TALENT_ARCANE_FOCUS] > 0 {
            hit+= 2.0 * (self.talents[TALENT_ARCANE_FOCUS] as f64);
        }

        if (spell.school == School::Fire || spell.school == School::Frost) && self.talents[TALENT_ELEMENTAL_PRECISION] > 0 {
            hit+= 2.0 * (self.talents[TALENT_ELEMENTAL_PRECISION] as f64);
        }

        return hit;
    }

    fn spell_crit_chance(&self, spell: &spell::Spell) -> f64 {
        let mut crit = self.stats.crit + self.auras.stats.crit;

        if spell.is_proc {
            return crit;
        }

        if self.talents[TALENT_ARCANE_INSTABILITY] > 0 {
            crit+= 1.0 * (self.talents[TALENT_ARCANE_INSTABILITY] as f64);
        }

        // TODO: More stuff

        return crit;
    }

    fn base_cast_time(&self, spell: &spell::Spell) -> f64 {
        let mut cast_time = spell.cast_time;

        if spell.id == spell::FROSTBOLT && self.talents[TALENT_IMP_FROSTBOLT] > 0 {
            cast_time-= 0.1 * (self.talents[TALENT_IMP_FROSTBOLT] as f64);
        }
        if spell.id == spell::FIREBALL && self.talents[TALENT_IMP_FIREBALL] > 0 {
            cast_time-= 0.1 * (self.talents[TALENT_IMP_FIREBALL] as f64);
        }
        if self.auras.has(aura::PRESENCE_OF_MIND) && !spell.is_channeled {
            cast_time = 0.0;
        }

        return cast_time;
    }

    fn spell_haste(&self) -> f64 {
        let mut haste = 1.0;

        if self.auras.has(aura::MQG) {
            haste*= 1.33;
        }
        if self.auras.has(aura::BERSERKING) {
            haste*= 1.1;
        }

        return 1.0 / haste;
    }

    fn set_gcd(&mut self, gcd: f64) {
        self.t_gcd = gcd;
    }
    
    fn spell_cooldown_mod(&self, spell: &spell::Spell) -> f64 {
        if spell.id == spell::FIRE_BLAST && self.talents[TALENT_IMP_FIRE_BLAST] > 0 {
            return -0.5 * (self.talents[TALENT_IMP_FIRE_BLAST] as f64);
        }

        return 0.0;
    }

    fn auras(&mut self) -> &mut aura::Auras {
        return &mut self.auras;
    }

    fn cooldowns(&mut self) -> &mut cooldown::Cooldowns {
        return &mut self.cooldowns;
    }

    fn next_event(&mut self, t: f64) -> Event {
        let mut event = Event::new(EventType::None);

        // GCD
        if t < self.t_gcd {
            event.event_type = EventType::Wait;
            event.t = self.t_gcd - t;
            event.text = String::from("GCD");
            return event;
        } 

        // TODO: Rotation
        event.event_type = EventType::CastStart;
        if !self.cooldowns.has(spell::FIRE_BLAST) {
            event.spell = Some(self.this_spell(spell::fire_blast()));
        } else if self.rng.gen_range(0..=1) == 0 {
            event.spell = Some(self.this_spell(spell::arcane_missiles()));
        } else {
            event.spell = Some(self.this_spell(spell::fireball()));
        }

        return event;
    }

    fn on_event(&mut self, event: &Event) -> Vec<Event> {
        let mut events: Vec<Event> = Vec::new();
        let mut fval: f64;

        match event.event_type {
            EventType::CastSuccess => {
                if event.spell.is_some() {
                    let spell = event.spell.as_ref().unwrap();

                    if spell.id == spell::MANA_RUBY {
                        fval = self.rng.gen_range(1000..=1200) as f64;
                        events.push(self.mana_event(fval, String::from("Mana Ruby")));
                    }
                }
            }

            EventType::SpellImpact => {
                if event.spell_instance.is_some() {
                    let instance = event.spell_instance.as_ref().unwrap();

                    if instance.spell.id == spell::FIREBALL {
                        events.push(self.spell_event(self.this_spell(spell::fireball_dot(instance.spell.rank)), event.target_id));
                    }
                }
            }

            EventType::SpellTick => {
                if event.spell_instance.is_some() {
                    //
                }
            }

            EventType::AuraGain => {
                if event.aura.is_some() {
                    //
                }
            }

            EventType::AuraExpire => {
                if event.aura.is_some() {
                    //
                }
            }

            EventType::CooldownGain => {
                if event.cooldown.is_some() {
                    //
                }
            }

            _ => {
                //
            }
        }

        return events;
    }

}