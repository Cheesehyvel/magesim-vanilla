use crate::aura;
use crate::common::School;
use crate::config::Config;
use crate::config::PlayerConfig;
use crate::cooldown;
use crate::event::Event;
use crate::event::EventType;
use crate::item;
use crate::macros::console_log;
use crate::sim::Sim;
use crate::spell;
use crate::stats::Stats;
use crate::unit::Unit;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use serde::{Serialize, Deserialize};

const TALENT_ARCANE_SUBLETY: usize = 0;
const TALENT_ARCANE_FOCUS: usize = 1;
const TALENT_IMP_ARCANE_MISSILES: usize = 2;
const TALENT_WAND_SPEC: usize = 3;
const TALENT_MAGIC_ABSORPTION: usize = 4;
const TALENT_ARCANE_CONCENTRATION: usize = 5;
const TALENT_MAGIC_ATTUNEMENT: usize = 6;
const TALENT_IMP_ARCANE_EXPLOSION: usize = 7;
const TALENT_ARCANE_RESILIENCE: usize = 8;
const TALENT_IMP_MANA_SHIELD: usize = 9;
const TALENT_IMP_COUNTERSPELL: usize = 10;
const TALENT_ARCANE_MEDITATION: usize = 11;
const TALENT_PRESENCE_OF_MIND: usize = 12;
const TALENT_ARCANE_MIND: usize = 13;
const TALENT_ARCANE_INSTABILITY: usize = 14;
const TALENT_ARCANE_POWER: usize = 15;

const TALENT_IMP_FIREBALL: usize = 16;
const TALENT_IMPACT: usize = 17;
const TALENT_IGNITE: usize = 18;
const TALENT_FLAME_THROWING: usize = 19;
const TALENT_IMP_FIRE_BLAST: usize = 20;
const TALENT_INCINERATE: usize = 21;
const TALENT_IMP_FLAMESTRIKE: usize = 22;
const TALENT_PYROBLAST: usize = 23;
const TALENT_BURNING_SOUL: usize = 24;
const TALENT_IMP_SCORCH: usize = 25;
const TALENT_IMP_FIRE_WARD: usize = 26;
const TALENT_MASTER_OF_ELEMENTS: usize = 27;
const TALENT_CRITICAL_MASS: usize = 28;
const TALENT_BLAST_WAVE: usize = 29;
const TALENT_FIRE_POWER: usize = 30;
const TALENT_COMBUSTION: usize = 31;

const TALENT_FROST_WARDING: usize = 32;
const TALENT_IMP_FROSTBOLT: usize = 33;
const TALENT_ELEMENTAL_PRECISION: usize = 34;
const TALENT_ICE_SHARDS: usize = 35;
const TALENT_FROSTBITE: usize = 36;
const TALENT_IMP_FROST_NOVA: usize = 37;
const TALENT_PERMAFROST: usize = 38;
const TALENT_PIERCING_ICE: usize = 39;
const TALENT_COLD_SNAP: usize = 40;
const TALENT_IMP_BLIZZARD: usize = 41;
const TALENT_ARCTIC_REACH: usize = 42;
const TALENT_FROST_CHANNELING: usize = 43;
const TALENT_SHATTER: usize = 44;
const TALENT_ICE_BLOCK: usize = 45;
const TALENT_IMP_CONE_OF_COLD: usize = 46;
const TALENT_WINTERS_CHILL: usize = 47;
const TALENT_ICE_BARRIER: usize = 48;

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
    pub auras: aura::Auras,
    pub cooldowns: cooldown::Cooldowns,
    pub rng: ChaCha8Rng,
    _combustion: i32,
    _mana_gems: i32,
}

impl Mage {
    pub fn new() -> Self {
        Self {
            id: 1,
            name: String::from("Player"),
            mana: 0.0,
            config: None,
            level: 60,
            base_mana: 0.0,
            t_gcd: 0.0,
            t_mana_spent: 0.0,
            stats: Stats::default(),
            auras: aura::Auras::default(),
            cooldowns: cooldown::Cooldowns::default(),
            rng: ChaCha8Rng::from_entropy(),
            _combustion: 0,
            _mana_gems: 0,
        }
    }

    fn player_config(&self) -> &PlayerConfig {
        &self.config.as_ref().unwrap().players[(self.id as usize) - 1]
    }

    fn talent(&self, index: usize) -> u8 {
        self.player_config().talents[index]
    }

    fn has_set(&self, set: i32, pc: i32) -> bool {
        self.player_config().items.iter().filter(|&x| *x == set).count() >= pc as usize
    }

    fn has_item(&self, item_id: i32) -> bool {
        self.player_config().items.contains(&item_id)
    }
}

impl Unit for Mage {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn reset(&mut self) {
        self.base_mana = 1213.0;
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
        self.level
    }

    fn max_mana(&self) -> f64 {
        // Subtract 280 because the first 20 intellect only gives 1 mana instead of 15
        self.base_mana + self.stats.int * 15.0 - 280.0 + self.stats.mana
    }

    fn current_mana(&self) -> f64 {
        self.mana
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

        if self.player_config().mage_armor {
            while_casting+= 0.3;
        }
        if self.talent(TALENT_ARCANE_MEDITATION) > 0 {
            while_casting+= 0.05 * (self.talent(TALENT_ARCANE_MEDITATION) as f64);
        }
        if self.t_mana_spent + 5.0 < t {
            while_casting = 1.0;
        }
        if self.auras.has_any(aura::INNERVATE) {
            spi*= 5.0;
            while_casting = 1.0;
        }
        if self.auras.has_any(aura::EVOCATION) {
            spi*= 16.0;
            while_casting = 1.0;
        }
        if self.auras.has_any(aura::BLUE_DRAGON) {
            while_casting = 1.0;
        }

        mps+= while_casting.min(1.0) * spi;

        mps
    }

    fn spirit_regen(&self) -> f64 {
        let spi = self.spirit();
        0.25 * spi.min(50.0) + 0.125 * (spi.max(50.0) - 50.0)
    }

    fn mp5(&self) -> f64 {
        self.stats.mp5 + self.auras.stats.mp5
    }

    fn intellect(&self) -> f64 {
        self.stats.int + self.auras.stats.int
    }

    fn spirit(&self) -> f64 {
        self.stats.spi + self.auras.stats.spi
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
            _ => {}
        }

        sp
    }

    fn spell_penetration(&self, school: School) -> f64 {
        self.stats.spell_penetration + self.auras.stats.spell_penetration
    }

    fn spell_mana_cost(&self, spell: &spell::Spell) -> f64 {
        let mut cost = spell.mana_cost;

        // Free spells
        if self.auras.has_any(aura::CLEARCAST) {
            return 0.0;
        }

        // Base cost modifiers
        if self.auras.has_any(aura::BURST_OF_KNOWLEDGE) {
            cost = (cost - 100.0).max(0.0);
        }

        // Multipliers
        if self.auras.has_any(aura::ARCANE_POWER) {
            cost*= 1.3;
        }

        cost
    }

    fn spell_hit_chance(&self, spell: &spell::Spell) -> f64 {
        let mut hit = self.stats.hit + self.auras.stats.hit;

        if spell.school == School::Arcane && self.talent(TALENT_ARCANE_FOCUS) > 0 {
            hit+= 2.0 * (self.talent(TALENT_ARCANE_FOCUS) as f64);
        }

        if (spell.school == School::Fire || spell.school == School::Frost) && self.talent(TALENT_ELEMENTAL_PRECISION) > 0 {
            hit+= 2.0 * (self.talent(TALENT_ELEMENTAL_PRECISION) as f64);
        }

        hit
    }

    fn spell_crit_chance(&self, spell: &spell::Spell) -> f64 {
        let mut crit = self.stats.crit + self.auras.stats.crit;

        if spell.is_proc {
            return crit;
        }

        if self.talent(TALENT_INCINERATE) > 0 && (spell.id == spell::FIRE_BLAST || spell.id == spell::SCORCH) {
            crit+= 2.0 * self.talent(TALENT_INCINERATE) as f64;
        }

        if self.talent(TALENT_ARCANE_INSTABILITY) > 0 {
            crit+= self.talent(TALENT_ARCANE_INSTABILITY) as f64;
        }

        if self.talent(TALENT_CRITICAL_MASS) > 0 && spell.school == School::Fire {
            crit+= 2.0 * self.talent(TALENT_CRITICAL_MASS) as f64;
        }

        if self.auras.has_any(aura::COMBUSTION) && spell.school == School::Fire {
            crit+= 10.0 * self.auras.stacks(aura::COMBUSTION, self.id) as f64;
        }

        if self.auras.has_any(aura::ARCANE_POTENCY) && spell.school == School::Arcane {
            crit+= 5.0;
        }

        crit
    }

    fn spell_crit_dmg_multiplier(&self, spell: &spell::Spell) -> f64 {
        let mut multi = 1.0;

        if spell.school == School::Frost && self.talent(TALENT_ICE_SHARDS) > 0 {
            multi+= self.talent(TALENT_ICE_SHARDS) as f64 * 0.2;
        }

        if self.auras.has_any(aura::ARCANE_POTENCY) && spell.school == School::Arcane {
            multi+= 0.5;
        }

        multi
    }
    
    fn buff_spell_dmg_multiplier(&self, spell: &spell::Spell) -> f64 {
        let mut dmg = 1.0;
        let mut additive = 1.0;

        if self.player_config().dmf_dmg {
            dmg*= 1.1;
        }
        if self.player_config().soul_revival {
            dmg*= 1.1;
        }
        if self.player_config().traces_of_silithyst {
            dmg*= 1.05;
        }
        if self.auras.has_any(aura::POWER_INFUSION) && !self.auras.has_any(aura::ARCANE_POWER) {
            dmg*= 1.2;
        }
        if self.has_set(item::SET_UDC, 3) {
            dmg*= 1.02;
        }

        // Additive category
        if spell.school == School::Frost && self.talent(TALENT_PIERCING_ICE) > 0 {
            additive+= 0.02 * (self.talent(TALENT_PIERCING_ICE) as f64);
        }
        if self.auras.has_any(aura::ARCANE_POWER) {
            additive+= 0.3;
        }

        // Ignite does not double dip talents
        if spell.id != spell::IGNITE {
            if spell.school == School::Fire && self.talent(TALENT_FIRE_POWER) > 0 {
                additive+= 0.02 * (self.talent(TALENT_FIRE_POWER) as f64);
            }
            if self.talent(TALENT_ARCANE_INSTABILITY) > 0 {
                additive+= 0.01 * (self.talent(TALENT_ARCANE_INSTABILITY) as f64);
            }
        }

        dmg * additive
    }

    fn base_cast_time(&self, spell: &spell::Spell) -> f64 {
        let mut cast_time = spell.cast_time;

        if spell.id == spell::FROSTBOLT && self.talent(TALENT_IMP_FROSTBOLT) > 0 {
            cast_time-= 0.1 * (self.talent(TALENT_IMP_FROSTBOLT) as f64);
        }
        if spell.id == spell::FIREBALL && self.talent(TALENT_IMP_FIREBALL) > 0 {
            cast_time-= 0.1 * (self.talent(TALENT_IMP_FIREBALL) as f64);
        }
        if self.auras.has_any(aura::PRESENCE_OF_MIND) && !spell.is_channeled {
            cast_time = 0.0;
        }
        if self.auras.has_any(aura::NETHERWIND_FOCUS) && !spell.is_channeled {
            cast_time = 0.0;
        }

        cast_time
    }

    fn spell_haste(&self) -> f64 {
        let mut haste = 1.0;

        if self.auras.has_any(aura::MIND_QUICKENING) {
            haste*= 1.33;
        }
        if self.auras.has_any(aura::BERSERKING) {
            haste*= 1.1;
        }

        1.0 / haste
    }

    fn set_gcd(&mut self, gcd: f64) {
        self.t_gcd = gcd;
    }
    
    fn spell_cooldown_mod(&self, spell: &spell::Spell) -> f64 {
        if spell.id == spell::FIRE_BLAST && self.talent(TALENT_IMP_FIRE_BLAST) > 0 {
            return -0.5 * (self.talent(TALENT_IMP_FIRE_BLAST) as f64);
        }
        if spell.id == spell::EVOCATION && self.has_set(item::SET_T3, 2) {
            return -60.0;
        }

        0.0
    }

    fn auras(&mut self) -> &mut aura::Auras {
        &mut self.auras
    }

    fn cooldowns(&mut self) -> &mut cooldown::Cooldowns {
        &mut self.cooldowns
    }

    fn next_event(&mut self, t: f64, num_targets: i32) -> Event {
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
        event.target_id = 1;
        if t < 5.0 {
            event.spell = Some(self.this_spell(spell::scorch()));
        } else if !self.cooldowns.has(spell::FIRE_BLAST) {
            event.spell = Some(self.this_spell(spell::fire_blast()));
        } else {
            event.spell = Some(self.this_spell(spell::fireball()));
        }

        event
    }

    fn on_event(&mut self, event: &Event) -> Vec<Event> {
        let mut events: Vec<Event> = Vec::new();

        match event.event_type {
            EventType::CastSuccess => {
                if event.spell.is_some() {
                    let spell = event.spell.as_ref().unwrap();
                    let is_harmful = spell.min_dmg > 0.0 || spell.max_dmg > 0.0;

                    // Item triggers
                    if spell.is_trigger {
                        match spell.id {
                            // Mana gems
                            spell::MANA_GEM => {
                                if self._mana_gems == 0 {
                                    let fval = self.rng.gen_range(1000..=1200) as f64;
                                    events.push(self.mana_event(fval, String::from("Mana Ruby")));
                                } else if self._mana_gems == 1 {
                                    let fval = self.rng.gen_range(775..=925) as f64;
                                    events.push(self.mana_event(fval, String::from("Mana Citrine")));
                                } else if self._mana_gems == 2 {
                                    let fval = self.rng.gen_range(550..=650) as f64;
                                    events.push(self.mana_event(fval, String::from("Mana Jade")));
                                } else if self._mana_gems == 3 {
                                    let fval = self.rng.gen_range(375..=425) as f64;
                                    events.push(self.mana_event(fval, String::from("Mana Agate")));
                                }
                                self._mana_gems+= 1;
                                return events;
                            }

                            // Mana potions
                            spell::MANA_POTION => {
                                let mut fval = self.rng.gen_range(1350..=2250) as f64;
                                if self.has_item(item::TRINKET_ALCHEMIST_STONE) {
                                    fval*= 1.33;
                                }
                                events.push(self.mana_event(fval, String::from("Major Mana Potion")));
                                return events;
                            }

                            // Trinkets
                            spell::BURST_OF_KNOWLEDGE => {
                                events.push(self.aura_event(aura::burst_of_knowledge(), 0));
                                return events;
                            }
                            spell::CHAOS_FIRE => {
                                let fval = self.rng.gen_range(1..=500) as f64;
                                events.push(self.mana_event(fval, String::from("Fire Ruby")));
                                events.push(self.aura_event(aura::chaos_fire(), 0));
                                return events;
                            }
                            spell::CHROMATIC_INFUSION => {
                                events.push(self.aura_event(aura::chromatic_infusion(), 0));
                                return events;
                            }
                            spell::EPHEMERAL_POWER => {
                                events.push(self.aura_event(aura::ephemeral_power(), 0));
                                return events;
                            }
                            spell::ESSENCE_OF_SAPPHIRON => {
                                events.push(self.aura_event(aura::essence_of_sapphiron(), 0));
                                return events;
                            }
                            spell::MANA_INFUSION => {
                                events.push(self.mana_event(500.0, String::from("Warmth of Forgiveness")));
                                return events;
                            }
                            spell::MIND_QUICKENING => {
                                events.push(self.aura_event(aura::mind_quickening(), 0));
                                return events;
                            }
                            spell::NAT_PAGLE => {
                                events.push(self.aura_event(aura::nat_pagle(), 0));
                                return events;
                            }
                            spell::OBSIDIAN_INSIGHT => {
                                events.push(self.aura_event(aura::obsidian_insight(), 0));
                                return events;
                            }
                            spell::UNSTABLE_POWER => {
                                events.push(self.aura_event(aura::unstable_power(), 0));
                                return events;
                            }

                            // Other items
                            spell::CELESTIAL_ORB => {
                                let fval = self.rng.gen_range(400..=1200) as f64;
                                events.push(self.mana_event(fval, spell.name.clone()));
                                return events;
                            }
                            spell::ROBE_ARCHMAGE => {
                                let fval = self.rng.gen_range(375..=625) as f64;
                                events.push(self.mana_event(fval, spell.name.clone()));
                                return events;
                            }

                            _ => {}
                        }
                    }

                    // Now we get to class abilities

                    // Triggers
                    if spell.is_trigger {
                        match spell.id {
                            spell::ARCANE_POWER => {
                                events.push(self.aura_event(aura::arcane_power(), 0));
                            }
                            spell::COLD_SNAP => {
                                // No spells worth resetting lmao
                            }
                            spell::COMBUSTION => {
                                events.push(self.aura_event(aura::combustion(), 0));
                            }
                            spell::EVOCATION => {
                                events.push(self.aura_event(aura::evocation(), 0));
                            }
                            spell::INNERVATE => {
                                events.push(self.aura_event(aura::innervate(), 0));
                            }
                            spell::MANA_TIDE => {
                                events.push(self.mana_event_at(290.0, String::from("Mana Tide"), 3.0));
                                events.push(self.mana_event_at(290.0, String::from("Mana Tide"), 6.0));
                                events.push(self.mana_event_at(290.0, String::from("Mana Tide"), 9.0));
                                events.push(self.mana_event_at(290.0, String::from("Mana Tide"), 12.0));
                                events.push(self.aura_event(aura::mana_tide(), 0));
                            }
                            spell::POWER_INFUSION => {
                                events.push(self.aura_event(aura::power_infusion(), 0));
                            }
                            spell::PRESENCE_OF_MIND => {
                                events.push(self.aura_event(aura::presence_of_mind(), 0));
                            }
                            _ => {}
                        }
                    }

                    if self.has_set(item::SET_T2, 8) && (spell.id == spell::ARCANE_MISSILES || spell.id == spell::FIREBALL || spell.id == spell::FROSTBOLT) && self.rng.gen_range(1..=10) == 1 {
                        events.push(self.aura_event(aura::netherwind_focus(), 0));
                    }

                    // Instant cast buffs
                    if spell.this_cast_time > 0.0 && !spell.is_channeled {
                        if self.auras.has_any(aura::PRESENCE_OF_MIND) {
                            events.push(self.aura_expire_event(aura::presence_of_mind(), 0));
                        } else if self.auras.has_any(aura::NETHERWIND_FOCUS) {
                            events.push(self.aura_expire_event(aura::netherwind_focus(), 0));
                        }
                    }

                    if self.auras.has_any(aura::CLEARCAST) && !spell.is_trigger {
                        events.push(self.aura_expire_event(aura::clearcast(), 0));
                    }

                    if spell.can_proc && self.has_item(item::TRINKET_BLUE_DRAGON) && self.rng.gen_range(1..=50) == 1 {
                        events.push(self.aura_event(aura::blue_dragon(), 0));
                    }

                    if is_harmful {
                        if self.auras.has_any(aura::UNSTABLE_POWER) {
                            events.push(self.aura_event(aura::unstable_power(), 0));
                        }
                        if self.auras.has_any(aura::CHAOS_FIRE) && spell.school == School::Fire {
                            events.push(self.aura_expire_event(aura::chaos_fire(), 0));
                        }
                    }
                }
            }

            EventType::SpellImpact => {
                if event.spell_instance.is_some() {
                    let instance = event.spell_instance.as_ref().unwrap();

                    if instance.result == spell::SpellResult::Miss && self.has_set(item::SET_AQ40, 5) {
                        events.push(self.aura_event(aura::enigmas_answer(), 0));
                    }

                    if instance.result != spell::SpellResult::Miss {
                        // Secondary dots
                        if instance.spell.id == spell::FIREBALL {
                            events.push(self.spell_event(self.this_spell(spell::fireball_dot(instance.spell.rank)), event.target_id));
                        }
                        if instance.spell.id == spell::PYROBLAST {
                            events.push(self.spell_event(self.this_spell(spell::pyroblast_dot(instance.spell.rank)), event.target_id));
                        }
                        if instance.spell.id == spell::FIRE_VULNERABILITY {
                            events.push(self.aura_event(aura::fire_vulnerability(), event.target_id));
                        }

                        if instance.spell.id == spell::SCORCH && self.talent(TALENT_IMP_SCORCH) > 0 {
                            let imp_sc = self.talent(TALENT_IMP_SCORCH) as i32;
                            if imp_sc == 3 || self.rng.gen_range(0..2) < imp_sc {
                                events.push(self.spell_event(self.this_spell(spell::fire_vulnerability()), event.target_id));
                            }
                        }

                        if !instance.spell.is_dot {
                            if self.talent(TALENT_ARCANE_CONCENTRATION) > 0 {
                                let mut fval = self.rng.gen_range(0.0..=100.0);
                                // Less chance per tick for channeled spells
                                if instance.spell.ticks > 0 {
                                    fval/= instance.spell.ticks as f64;
                                }
                                if fval < (self.talent(TALENT_ARCANE_CONCENTRATION) as f64) * 2.0 {
                                    events.push(self.aura_event(aura::clearcast(), 0));
                                }
                            }

                            if self.auras.has_any(aura::COMBUSTION) && instance.spell.school == School::Fire {
                                if instance.result == spell::SpellResult::Crit {
                                    self._combustion+= 1;
                                }
                                if self._combustion == 3 {
                                    events.push(self.aura_expire_event(aura::combustion(), 0));
                                } else {
                                    events.push(self.aura_event(aura::combustion(), 0));
                                }
                            }

                            if self.talent(TALENT_WINTERS_CHILL) > 0 && instance.spell.school == School::Frost && (self.talent(TALENT_WINTERS_CHILL) == 5 || self.rng.gen_range(1..=5) <= self.talent(TALENT_WINTERS_CHILL) as i32) {
                                events.push(self.aura_event(aura::winters_chill(), event.target_id));
                            }
                        }
                    }

                    if instance.result == spell::SpellResult::Crit {
                        if self.talent(TALENT_IGNITE) > 0 && instance.spell.school == School::Fire && !instance.spell.is_proc && instance.dmg > 0.0 {
                            events.push(self.spell_event(self.this_spell(spell::ignite(instance.dmg * 0.2)), event.target_id));
                        }

                        if self.talent(TALENT_MASTER_OF_ELEMENTS) > 0 && (instance.spell.school == School::Fire || instance.spell.school == School::Frost) && instance.spell.mana_cost > 0.0 {
                            let mana = 0.1 * instance.spell.mana_cost * self.talent(TALENT_MASTER_OF_ELEMENTS) as f64;
                            events.push(self.mana_event(mana, String::from("Master of Elements")));
                        }
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
                    let aura = event.aura.as_ref().unwrap();

                    if aura.id == aura::COMBUSTION {
                        self._combustion = 0;
                    }
                }
            }

            EventType::AuraExpire => {
                if event.aura.is_some() {
                    let aura = event.aura.as_ref().unwrap();

                    if aura.id == aura::COMBUSTION {
                        events.push(self.spell_cooldown_event(spell::combustion()));
                    }
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

        events
    }

}