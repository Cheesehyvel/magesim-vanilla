use crate::aura;
use crate::common::School;
use crate::config::Config;
use crate::cooldown;
use crate::event::Event;
use crate::event::EventType;
use crate::spell::Spell;
use crate::sim::Sim;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

pub trait Unit {
    fn id(&self) -> i32;
    fn name(&self) -> String;
    fn reset(&mut self);
    fn set_config(&mut self, config: Config);
    fn new_rng(&mut self, rng_seed: u64);
    fn max_mana(&self) -> f64;
    fn current_mana(&self) -> f64;
    fn mod_mana(&mut self, mana: f64, t: f64);
    fn mana_per_second(&self, t: f64) -> f64;
    fn spirit_regen(&self) -> f64;
    fn mp5(&self) -> f64;
    fn intellect(&self) -> f64;
    fn spirit(&self) -> f64;
    fn spell_power(&self, school: School) -> f64;
    fn spell_penetration(&self, school: School) -> f64;
    fn set_gcd(&mut self, gcd: f64);
    fn auras(&mut self) -> &mut aura::Auras;
    fn cooldowns(&mut self) -> &mut cooldown::Cooldowns;
    fn next_event(&mut self, t: f64, num_targets: i32) -> Event;
    fn on_event(&mut self, event: &Event) -> Vec<Event>;

    fn owner_id(&self) -> i32 {
        self.id()
    }

    fn level(&self) -> i32 {
        60
    }

    fn mana_percent(&self) -> f64 {
        self.current_mana() / self.max_mana() * 100.0
    }

    fn base_cast_time(&self, spell: &Spell) -> f64 {
        spell.cast_time
    }

    fn cast_time(&self, spell: &Spell) -> f64 {
        self.base_cast_time(spell) * self.spell_haste()
    }

    fn spell_haste(&self) -> f64 {
        1.0
    }

    fn spell_mana_cost(&self, spell: &Spell) -> f64 {
        spell.mana_cost
    }
    
    fn spell_coeff_mod(&self, spell: &Spell) -> f64 {
        0.0
    }
    
    fn spell_cooldown_mod(&self, spell: &Spell) -> f64 {
        0.0
    }

    fn spell_hit_chance(&self, spell: &Spell) -> f64 {
        0.0
    }

    fn spell_crit_chance(&self, spell: &Spell) -> f64 {
        0.0
    }

    fn spell_crit_dmg_base_multiplier(&self, spell: &Spell) -> f64 {
        1.0
    }

    fn spell_crit_dmg_multiplier(&self, spell: &Spell) -> f64 {
        1.0
    }
    
    fn buff_spell_dmg_multiplier(&self, spell: &Spell) -> f64 {
        1.0
    }

    fn this_spell(&self, mut spell: Spell) -> Spell {
        spell.this_cast_time = self.cast_time(&spell);
        spell.this_mana_cost = self.spell_mana_cost(&spell);
        spell.coeff+= self.spell_coeff_mod(&spell);
        spell.cooldown+= self.spell_cooldown_mod(&spell);
        spell
    }

    fn mana_event(&self, mana: f64, text: String) -> Event {
        Event {
            event_type: EventType::ManaGain,
            unit_id: self.id(),
            text,
            is_main_event: false,
            ..Default::default()
        }
    }

    fn spell_event(&self, spell: Spell, target_id: i32) -> Event {
        Event {
            event_type: EventType::CastStart,
            unit_id: self.id(),
            target_id,
            spell: Some(spell),
            is_main_event: false,
            ..Default::default()
        }
    }

    fn aura_event(&self, mut aura: aura::Aura, target_id: i32) -> Event {
        aura.owner_id = self.id();

        Event {
            event_type: EventType::AuraGain,
            unit_id: self.id(),
            target_id,
            aura: Some(aura),
            is_main_event: false,
            ..Default::default()
        }
    }
}