use crate::aura;
use crate::common;
use crate::cooldown;
use crate::config::Config;
use crate::spell;
use crate::event::Event;
use crate::event::EventType;
use crate::log;
use crate::target::Target;
use crate::unit::Unit;
use crate::mage::Mage;
use crate::macros::console_log;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub fn new_rng(rng_seed: u64) -> ChaCha8Rng {
    if rng_seed != 0 {
        return ChaCha8Rng::seed_from_u64(rng_seed);
    }

    ChaCha8Rng::from_entropy()
}

// Result from one run
#[derive(Default, Serialize, Deserialize)]
pub struct SimulationResult {
    pub t: f64,
    pub dmg: u64,
    pub dps: f64,
    pub player_dmg: Vec<u64>,
    pub player_dps: Vec<f64>,
    pub log: Vec<log::LogEntry>,
}

// Result from multiple runs
#[derive(Default, Serialize, Deserialize)]
pub struct SimulationsResult {
    pub avg_dps: f64,
    pub min_dps: f64,
    pub max_dps: f64,
    pub player_avg_dps: Vec<f64>,
    pub player_min_dps: Vec<f64>,
    pub player_max_dps: Vec<f64>,
    pub iterations: i32,
}

// Public function to start a single simulation
pub fn run_single(config: Config) -> SimulationResult {
    let mut sim = Sim::new(config);

    sim.iteration = 1;
    sim.log_enabled = true;

    sim.run()
}

// Public function to start multiple simulations
pub fn run_multiple(config: Config, iterations: i32) -> SimulationsResult {
    let mut sim = Sim::new(config);
    sim.log_enabled = false;

    let mut result: SimulationsResult = SimulationsResult { iterations, ..Default::default() };

    for i in 1..=iterations {
        sim.iteration = i;
        let r = sim.run();

        result.avg_dps+= (r.dps - result.avg_dps) / (i as f64);
        if i == 1 || r.dps < result.min_dps {
            result.min_dps = r.dps;
        }
        if i == 1 || r.dps > result.max_dps {
            result.max_dps = r.dps;
        }

        for (j, pdps) in r.player_dps.iter().enumerate() {
            if i == 1 {
                result.player_avg_dps.push(0.0);
                result.player_min_dps.push(0.0);
                result.player_max_dps.push(0.0);
            }

            result.player_avg_dps[j]+= (*pdps - result.player_avg_dps[j]) / (i as f64);
            if i == 1 || *pdps < result.player_min_dps[j] {
                result.player_min_dps[j] = *pdps;
            }
            if i == 1 || *pdps > result.player_max_dps[j] {
                result.player_max_dps[j] = *pdps;
            }
        }
    }

    result
}

fn spawn_player(config: Config, id: i32) -> Box<dyn Unit> {
    let mut player = Box::new(Mage::new());
    let index = (id as usize) - 1;

    player.id = id;
    player.level = config.players[index].level;
    player.stats = config.players[index].stats;

    player.set_config(config);
    player.reset();

    player
}

// Main sim struct
pub struct Sim {
    pub config: Config,
    pub queue: Vec<Event>,
    pub t: f64,
    pub duration: f64,
    pub iteration: i32,
    pub rng: ChaCha8Rng,
    pub units: HashMap<i32, Box<dyn Unit>>,
    pub targets: HashMap<i32, Target>,
    pub log_enabled: bool,
    pub log: Vec<log::LogEntry>,
}

impl Sim {

    pub fn new(config: Config) -> Self {
        Self {
            config,
            queue: vec![],
            t: 0.0,
            duration: 0.0,
            iteration: 1,
            rng: new_rng(0),
            units: HashMap::new(),
            targets: HashMap::new(),
            log_enabled: false,
            log: vec![],
        }
    }

    // The internal function to start a simulation
    pub fn run(&mut self) -> SimulationResult {
        self.reset();

        for i in 1..=self.config.players.len() {
            let id = i as i32;
            self.push_mana_regen(id);
            if i > 1 && self.config.player_delay > 0.0 {
                self.push_idle(id, self.config.player_delay * i as f64, String::from("Player delay"));
            } else {
                self.next_event(id);
            }
        }

        self.work();

        let mut result: SimulationResult = Default::default();
        result.t = self.duration;
        result.dmg = self.total_dmg();
        result.dps = (result.dmg as f64) / result.t;
        result.log = self.log.clone();

        for i in 1..=self.config.players.len() {
            let id = i as i32;
            result.player_dmg.push(self.unit_total_dmg(id));
            result.player_dps.push((result.player_dmg[i - 1] as f64) / result.t);
        }

        result
    }

    fn reset(&mut self) {
        let rng_seed = self.config.rng_seed + (self.iteration as u64) - 1;
        if self.config.rng_seed != 0 {
            self.rng = new_rng(rng_seed);
        }

        self.t = 0.0;
        self.duration = self.config.duration - self.config.duration_variance + self.rng.gen_range(0.0..=self.config.duration_variance) * 2.0;

        self.queue.clear();

        self.units.clear();
        for i in 1..=self.config.players.len() {
            let id = i as i32;
            self.units.insert(id, spawn_player(self.config.clone(), id));
            if self.config.rng_seed != 0 {
                self.units.get_mut(&id).unwrap().new_rng(rng_seed);
            }
        }

        self.targets.clear();
        for i in 1..=self.config.targets {
            self.targets.insert(i, Target::new(i));
        }

        if self.log_enabled {
            self.log.clear();
        }
    }

    fn unit(&self, unit_id: i32) -> &dyn Unit {
        return self.units[&unit_id].as_ref();
    }

    fn target(&self, target_id: i32) -> &Target {
        return &self.targets[&target_id];
    }

    fn total_dmg(&self) -> u64 {
        self.targets.iter().fold(0, |acc, (id, t)| acc + t.total_dmg())
    }

    fn unit_total_dmg(&self, unit_id: i32) -> u64 {
        self.targets.iter().fold(0, |acc, (id, t)| acc + t.dmg.get(&unit_id).unwrap_or(&0))
    }

    fn work(&mut self) {
        while !self.queue.is_empty() {
            let mut event = self.queue.remove(0);

            if event.t > self.duration {
                self.t = self.duration;
                break;
            }

            self.t = event.t;
            self.handle_event(&mut event);
        }
    }

    fn handle_events(&mut self, events: Vec<Event>) {
        for mut event in events {
            event.t = self.t;
            self.handle_event(&mut event);
        }
    }

    fn handle_event(&mut self, event: &mut Event) {
        match event.event_type {
            EventType::CastStart => {
                let spell = event.spell.as_ref().unwrap();
                if self.can_cast(event.unit_id, spell) {
                    self.on_cast_start(event);
                } else if event.is_main_event {
                    self.wait(event.unit_id, 0.5, format!("Not enough mana (needed {})", spell.this_mana_cost));
                }
            }

            EventType::CastFinish => {
                let spell = event.spell.as_mut().unwrap();
                spell.this_mana_cost = self.unit(event.unit_id).spell_mana_cost(spell);

                if self.can_cast(event.unit_id, spell) {
                    self.on_cast_success(event);
                } else if event.is_main_event {
                    self.wait(event.unit_id, 0.5, format!("Not enough mana (needed {})", spell.this_mana_cost));
                }
            }

            EventType::SpellImpact => {
                self.on_spell_impact(event);
            }

            EventType::SpellTick => {
                self.on_spell_tick(event);
            }

            EventType::ManaRegen => {
                self.on_mana_regen(event);
            }

            EventType::ManaGain => {
                self.on_mana_gain(event);
            }

            EventType::AuraGain => {
                self.on_aura_gain(event);
            }

            EventType::AuraExpire => {
                self.on_aura_expire(event);
            }

            EventType::CooldownGain => {
                self.on_cooldown_gain(event);
            }

            EventType::CooldownExpire => {
                self.on_cooldown_expire(event);
            }

            EventType::Wait => {
                if !event.text.is_empty() {
                    self.log(log::LogType::Wait, event.text.clone(), event.unit_id);
                }

                let mut ev = Event::new(EventType::Idle);
                ev.event_type = EventType::Idle;
                ev.t = event.t;
                ev.text = event.text.clone();
                ev.unit_id = event.unit_id;
                self.push_event(ev);
            }

            EventType::Idle => {
                self.next_event(event.unit_id);
            }

            _ => {
                console_log!("Unhandled event type");
            }
        }
    }

    fn next_event(&mut self, unit_id: i32) {
        self.log(log::LogType::Debug, "Next event".to_string(), unit_id);

        let mut event = self.units.get_mut(&unit_id).unwrap().next_event(self.t, self.config.targets);
        event.unit_id = unit_id;
        self.handle_event(&mut event);
    }

    pub fn push_event(&mut self, mut event: Event) {
        event.t+= self.t;

        for (i, ev) in self.queue.iter().enumerate() {
            if event.t < ev.t {
                self.queue.insert(i, event);
                return;
            }
        }

        self.queue.push(event);
    }

    fn push_spell_impact(&mut self, unit_id: i32, spell: &spell::Spell, target_id: i32, t: f64) {
        let mut event = Event::new(EventType::SpellImpact);
        event.t = t;
        event.unit_id = unit_id;
        event.target_id = target_id;
        event.spell_instance = Some(self.get_spell_instance(unit_id, spell, target_id));
        self.push_event(event);
    }

    fn push_channeling_tick(&mut self, unit_id: i32, spell: &spell::Spell, target_id: i32, tick: u8, t_offset: f64) {
        let mut event = Event::new(EventType::SpellTick);
        event.t = spell.this_cast_time / (spell.ticks as f64) * (tick as f64) + t_offset;
        event.unit_id = unit_id;
        event.target_id = target_id;
        event.spell_instance = Some(spell::SpellInstance::new(spell.clone()));

        if spell.is_dynamic {
            event.spell_instance = Some(spell::SpellInstance::new(spell.clone()));
            event.spell_instance.as_mut().unwrap().result = spell::SpellResult::Pending;
        } else {
            event.spell_instance = Some(self.get_spell_instance(unit_id, spell, target_id));
        }

        let instance = event.spell_instance.as_mut().unwrap();
        instance.tick = tick;

        self.push_event(event);
    }

    fn push_dot_tick(&mut self, unit_id: i32, spell: &spell::Spell, target_id: i32, tick: u8, t_offset: f64) {
        let mut event = Event::new(EventType::SpellTick);
        event.t = spell.t_interval * (tick as f64) + t_offset;
        event.unit_id = unit_id;
        event.target_id = target_id;

        if spell.is_dynamic {
            event.spell_instance = Some(spell::SpellInstance::new(spell.clone()));
            event.spell_instance.as_mut().unwrap().result = spell::SpellResult::Pending;
        } else {
            event.spell_instance = Some(self.get_spell_instance(unit_id, spell, target_id));
        }

        let instance = event.spell_instance.as_mut().unwrap();
        instance.tick = tick;

        self.push_event(event);
    }

    fn push_mana_regen(&mut self, unit_id: i32) {
        let mut event = Event::new(EventType::ManaRegen);
        event.unit_id = unit_id;
        event.t = crate::MANA_TICK_T;
        event.is_main_event = false;
        self.push_event(event);
    }

    fn push_idle(&mut self, unit_id: i32, t: f64, text: String) {
        let mut event = Event::new(EventType::Idle);
        event.t = t;
        event.unit_id = unit_id;
        event.text = text;
        self.push_event(event);
    }

    fn wait(&mut self, unit_id: i32, t: f64, text: String) {
        let mut event = Event::new(EventType::Wait);
        event.t = t;
        event.unit_id = unit_id;
        event.text = text;
        self.handle_event(&mut event);
    }

    fn can_cast(&mut self, unit_id: i32, spell: &spell::Spell) -> bool {
        return self.unit(unit_id).current_mana() >= spell.this_mana_cost;
    }

    fn cast_spell(&mut self, unit_id: i32, spell: spell::Spell, target_id: i32) {
        let mut event = Event::new(EventType::CastStart);
        event.spell = Some(spell);
        event.unit_id = unit_id;
        event.target_id = target_id;
        self.handle_event(&mut event);
    }

    fn on_cast_start(&mut self, event: &mut Event) {
        if event.spell.is_none() {
            return;
        }

        let spell = event.spell.as_ref().unwrap();

        let mut log_text = format!("s[{}]", spell.name);
        if event.target_id != 0 {
            log_text.push_str(&format!(" on t[{}]", self.target(event.target_id).name));
        }
        self.log(log::LogType::CastStart, log_text, event.unit_id);

        // Set unit gcd
        if event.is_main_event {
            self.units.get_mut(&event.unit_id).unwrap().set_gcd(self.t + spell.gcd);
        }

        if spell.is_channeled {
            self.on_cast_success(event);
        } else {
            let mut ev = Event::new(EventType::CastFinish);
            ev.unit_id = event.unit_id;
            ev.spell = Some(spell.clone());
            ev.target_id = event.target_id;
            ev.t = spell.this_cast_time;
            ev.is_main_event = event.is_main_event;
            ev.is_quiet = event.is_quiet;
            self.push_event(ev);
        }
    }

    fn on_cast_success(&mut self, event: &mut Event) {
        if event.spell.is_none() {
            return;
        }

        let spell = event.spell.as_ref().unwrap();

        // Apply mana cost and cooldown
        let unit = self.units.get_mut(&event.unit_id).unwrap();
        unit.mod_mana(-spell.this_mana_cost, self.t);
        if spell.cooldown > 0.0 {
            self.add_spell_cooldown(event.unit_id, spell);
        }

        self.log(log::LogType::CastSuccess, format!("s[{}]", spell.name), event.unit_id);

        if spell.is_trigger {
            // Do nothing
        } else if spell.is_dot {
            self.apply_dot(event.unit_id, spell, event.target_id);
        } else if spell.is_channeled {
            for i in 1..=spell.ticks {
                self.push_channeling_tick(event.unit_id, spell, event.target_id, i, 0.0);
            }
        } else {
            self.push_spell_impact(event.unit_id, spell, event.target_id, spell.travel_time(self.config.distance as f64));
        }

        if event.is_main_event {
            event.event_type = EventType::CastSuccess;
            let events = self.units.get_mut(&event.unit_id).unwrap().on_event(event);
            self.handle_events(events);

            if spell.is_channeled && spell.this_cast_time > 0.0 {
                self.push_idle(event.unit_id, spell.this_cast_time, String::from(""));
            } else {
                self.push_idle(event.unit_id, 0.0, String::from(""));
            }
        }
    }

    fn on_spell_impact(&mut self, event: &mut Event) {
        if event.spell_instance.is_none() {
            return;
        }

        let instance = event.spell_instance.as_mut().unwrap();

        if instance.result == spell::SpellResult::Pending {
            self.roll_spell_instance(event.unit_id, instance, event.target_id);
        }

        if instance.spell.is_dot {
            instance.dmg*= self.spell_debuff_dmg_multiplier(event.unit_id, &instance.spell, event.target_id);
        }

        if instance.dmg > 0.0 && event.unit_id != 0 {
            self.targets.get_mut(&event.target_id).expect("TARGET_NOT_FOUND").add_dmg(event.unit_id, instance.dmg as u64);
        }

        let si = event.spell_instance.as_ref().unwrap();
        self.log_spell_impact(event.unit_id, event.spell_instance.as_ref().unwrap(), event.target_id);

        let events = self.units.get_mut(&event.unit_id).unwrap().on_event(event);
        self.handle_events(events);

        // TODO: spell logging
    }

    fn on_spell_tick(&mut self, event: &mut Event) {
        if event.spell_instance.is_none() {
            return;
        }

        let instance = event.spell_instance.as_ref().unwrap();

        if !instance.spell.is_trigger {
            let mut ev = Event::new(EventType::SpellImpact);
            ev.t = instance.spell.travel_time(self.config.distance as f64);
            ev.unit_id = event.unit_id;
            ev.target_id = event.target_id;
            ev.spell_instance = Some(instance.clone());
            ev.is_main_event = event.is_main_event;
            ev.is_quiet = event.is_quiet;
            self.push_event(ev);
        }

        let events = self.units.get_mut(&event.unit_id).unwrap().on_event(event);
        self.handle_events(events);
    }

    fn on_mana_gain(&mut self, event: &mut Event) {
        if event.mana == 0.0 {
            return;
        }

        self.units.get_mut(&event.unit_id).unwrap().mod_mana(event.mana, self.t);

        self.log(log::LogType::Mana, format!("Mana m[{}]", event.mana), event.unit_id);
    }

    fn on_mana_regen(&mut self, event: &mut Event) {
        let unit = self.units.get_mut(&event.unit_id).unwrap();
        let mana = (unit.mana_per_second(self.t) * crate::MANA_TICK_T).round();

        if mana > 0.0 {
            unit.mod_mana(mana, self.t);
            self.log(log::LogType::Mana, format!("Mana regen m[{}]", mana), event.unit_id);
        }

        self.push_mana_regen(event.unit_id);
    }

    fn on_aura_gain(&mut self, event: &mut Event) {
        if event.aura.is_none() {
            return;
        }

        let aura = event.aura.as_mut().unwrap();

        let auras = if event.target_id != 0 {
            &mut self.targets.get_mut(&event.target_id).unwrap().auras
        } else {
            self.units.get_mut(&event.unit_id).unwrap().auras()
        };

        let old_stacks = auras.stacks(aura.id, aura.owner_id);

        if old_stacks < 1 {
            aura.t_gained = self.t;
        }

        let stacks = auras.add(aura.clone());

        if old_stacks < 1 || aura.stack_refresh {
            aura.t_expires = self.t + aura.duration;
            self.remove_aura_expiration(event.unit_id, aura.id, event.target_id);

            let mut expire = Event::new(EventType::AuraExpire);
            expire.unit_id = event.unit_id;
            expire.t = aura.duration;
            expire.aura = Some(aura.clone());
            expire.is_main_event = false;
            self.push_event(expire);
        }

        if stacks != old_stacks || aura.show_refresh {
            let events = self.units.get_mut(&event.unit_id).unwrap().on_event(event);
            self.handle_events(events);

            let a = event.aura.as_ref().unwrap();
            if event.target_id != 0 {
                self.log(log::LogType::AuraGain, format!("a[{}] ({}) -> t[{}]", a.name, stacks, self.target(event.target_id).name), event.unit_id);
            } else {
                self.log(log::LogType::AuraGain, format!("a[{}] ({})", a.name, stacks), event.unit_id);
            }
        }
    }

    fn on_aura_expire(&mut self, event: &mut Event) {
        if event.aura.is_none() {
            return;
        }

        let auras = self.units.get_mut(&event.unit_id).unwrap().auras();
        let aura = event.aura.as_ref().unwrap();

        if auras.has(aura.id, aura.owner_id) {
            auras.remove(aura.id, aura.owner_id);
            self.remove_aura_expiration(event.unit_id, aura.id, event.target_id);

            let events = self.units.get_mut(&event.unit_id).unwrap().on_event(event);
            self.handle_events(events);

            self.log(log::LogType::AuraExpire, format!("a[{}]", aura.name), event.unit_id);
        }
    }

    fn on_cooldown_gain(&mut self, event: &mut Event) {
        if event.cooldown.is_none() {
            return;
        }

        let cooldowns = self.units.get_mut(&event.unit_id).unwrap().cooldowns();
        let cooldown = event.cooldown.as_mut().unwrap();

        if cooldowns.duration(cooldown.id) >= cooldown.duration {
            return;
        } 

        cooldown.t_gained = self.t;
        cooldown.t_expires = self.t + cooldown.duration;

        cooldowns.add(cooldown.clone());
        self.remove_cooldown_expiration(event.unit_id, cooldown.id);

        let mut expire = Event::new(EventType::CooldownExpire);
        expire.unit_id = event.unit_id;
        expire.t = cooldown.duration;
        expire.cooldown = Some(cooldown.clone());
        expire.is_main_event = false;
        self.push_event(expire);

        let events = self.units.get_mut(&event.unit_id).unwrap().on_event(event);
        self.handle_events(events);

        let c = event.cooldown.as_ref().unwrap();
        self.log(log::LogType::CooldownGain, format!("c[{}]", c.name), event.unit_id);
    }

    fn on_cooldown_expire(&mut self, event: &mut Event) {
        if event.cooldown.is_none() {
            return;
        }

        let cooldowns = self.units.get_mut(&event.unit_id).unwrap().cooldowns();
        let cooldown = event.cooldown.as_ref().unwrap();

        if cooldowns.has(cooldown.id) {
            cooldowns.remove(cooldown.id);
            self.remove_cooldown_expiration(event.unit_id, cooldown.id);

            self.log(log::LogType::CooldownExpire, format!("c[{}]", cooldown.name), event.unit_id);
        }
    }

    fn add_spell_cooldown(&mut self, unit_id: i32, spell: &spell::Spell) {
        let mut event = Event::new(EventType::CooldownGain);
        event.is_main_event = false;
        event.unit_id = unit_id;
        event.t = self.t;
        event.cooldown = Some(cooldown::Cooldown::new(spell.id, spell.name.clone(), spell.cooldown));
        self.on_cooldown_gain(&mut event);
    }

    fn remove_spell_impacts(&mut self, unit_id: i32, spell_id: i32, target_id: i32) {
        self.queue.retain(|ev| ev.event_type != EventType::SpellImpact || ev.unit_id != unit_id || ev.target_id != target_id || ev.spell_instance.as_ref().is_none() || ev.spell_instance.as_ref().unwrap().spell.id != spell_id);
    }

    fn remove_spell_ticks(&mut self, unit_id: i32, spell_id: i32, target_id: i32) {
        self.queue.retain(|ev| ev.event_type != EventType::SpellTick || ev.unit_id != unit_id || ev.target_id != target_id || ev.spell_instance.as_ref().is_none() || ev.spell_instance.as_ref().unwrap().spell.id != spell_id);
    }

    fn remove_aura_expiration(&mut self, unit_id: i32, id: i32, target_id: i32) {
        self.queue.retain(|ev| ev.event_type != EventType::AuraExpire || ev.unit_id != unit_id || ev.target_id != target_id || ev.aura.as_ref().is_none() || ev.aura.as_ref().unwrap().id != id);
    }

    fn remove_cooldown_expiration(&mut self, unit_id: i32, id: i32) {
        self.queue.retain(|ev| ev.event_type != EventType::CooldownExpire || ev.unit_id != unit_id || ev.cooldown.as_ref().is_none() || ev.cooldown.as_ref().unwrap().id != id);
    }

    fn apply_dot(&mut self, unit_id: i32, spell: &spell::Spell, target_id: i32) {
        if spell.id == spell::IGNITE {
            // TODO
        }
        else {
            // This is the most common case
            if spell.tick_refresh {
                self.remove_spell_ticks(unit_id, spell.id, target_id);

                for i in 1..=spell.ticks {
                    self.push_dot_tick(unit_id, spell, target_id, i, 0.0);
                }
            } else {
                // TODO
            }
        }
    }

    fn get_spell_instance(&mut self, unit_id: i32, spell: &spell::Spell, target_id: i32) -> spell::SpellInstance {
        let mut instance = spell::SpellInstance::new(spell.clone());

        self.roll_spell_instance(unit_id, &mut instance, target_id);

        instance
    }

    fn roll_spell_instance(&mut self, unit_id: i32, instance: &mut spell::SpellInstance, target_id: i32) {
        if instance.spell.max_dmg > 0.0 {
            instance.result = self.roll_spell_result(unit_id, instance, target_id);

            if instance.result != spell::SpellResult::Miss {
                instance.dmg = self.roll_spell_dmg(unit_id, &instance.spell, target_id);
            }

            if !instance.spell.is_dot {
                instance.dmg*= self.spell_debuff_dmg_multiplier(unit_id, &instance.spell, target_id);
            }

            if instance.result == spell::SpellResult::Crit {
                instance.dmg*= self.spell_crit_dmg_multiplier(unit_id, &instance.spell, target_id);
            }

            if !instance.spell.is_fixed_dmg {
                instance.resist = self.spell_dmg_resist(unit_id, instance);
                instance.dmg-= instance.resist;
            }

            instance.dmg = instance.dmg.round();
        }
    }

    fn roll_spell_result(&mut self, unit_id: i32, instance: &mut spell::SpellInstance, target_id: i32) -> spell::SpellResult {
        if instance.spell.can_miss && self.rng.gen_range(0.0..=100.0) > self.spell_hit_chance(unit_id, &instance.spell, target_id) {
            return spell::SpellResult::Miss;
        }

        if instance.spell.can_crit && self.rng.gen_range(0.0..=100.0) <= self.spell_crit_chance(unit_id, &instance.spell, target_id) {
            return spell::SpellResult::Crit;
        }

        spell::SpellResult::Hit
    }

    fn spell_hit_chance(&mut self, unit_id: i32, spell: &spell::Spell, target_id: i32) -> f64 {
        let dlevel = (self.config.target_level - self.unit(unit_id).level()) as f64;
        let mut hit: f64 = 96.0 - dlevel;

        if dlevel > 2.0 {
            hit-= (dlevel - 2.0) * 10.0;
        }

        // Miss chance for binary spells
        // Based on targets non-level based resistance
        // Chance of resist is 75% of the resistScore/resistCap
        // https://royalgiraffe.github.io/resist-guide
        if spell.is_binary {
            hit*= 1.0 - 0.75 * self.spell_resist_score(unit_id, spell, false) / ((self.config.target_level as f64) * 5.0);
        }

        hit+= self.unit(unit_id).spell_hit_chance(spell);

        hit.min(99.0)
    }

    fn spell_crit_chance(&mut self, unit_id: i32, spell: &spell::Spell, target_id: i32) -> f64 {
        let mut crit: f64 = self.unit(unit_id).spell_crit_chance(spell);

        // TODO: Check debuffs

        crit = crit.min(100.0);

        // Crit suppression
        if spell.max_dmg > 0.0 {
            let dlevel = self.config.target_level - self.unit(unit_id).level();
            if dlevel == 3 {
                crit-= 2.1;
            } else if dlevel == 2 {
                crit-= 0.3;
            }
        }

        crit
    }

    fn roll_spell_dmg(&mut self, unit_id: i32, spell: &spell::Spell, target_id: i32) -> f64 {
        let mut dmg: f64;

        if self.config.avg_spell_dmg {
            dmg = (spell.min_dmg + spell.max_dmg) / 2.0;
        } else {
            dmg = self.rng.gen_range(spell.min_dmg..=spell.max_dmg);
        }

        if spell.is_fixed_dmg {
            return dmg;
        }

        if spell.coeff > 0.0 {
            dmg+= self.get_spell_power(unit_id, spell, target_id) * spell.coeff;
        }

        dmg*= self.spell_buff_dmg_multiplier(unit_id, spell);

        dmg
    }

    fn get_spell_power(&mut self, unit_id: i32, spell: &spell::Spell, target_id: i32) -> f64 {
        self.unit(unit_id).spell_power(spell.school)
    }

    fn spell_buff_dmg_multiplier(&mut self, unit_id: i32, spell: &spell::Spell) -> f64 {
        self.unit(unit_id).buff_spell_dmg_multiplier(spell)
    }

    fn spell_debuff_dmg_multiplier(&mut self, unit_id: i32, spell: &spell::Spell, target_id: i32) -> f64 {
        let mut dmg = 1.0;

        let auras = &self.targets.get_mut(&target_id).expect("TARGET_NOT_FOUND").auras;

        if auras.has_any(aura::FIRE_VULNERABILITY) {
            dmg*= 1.0 + 0.03 * auras.stacks(aura::FIRE_VULNERABILITY, 0) as f64;
        }

        dmg
    }

    fn spell_crit_dmg_multiplier(&mut self, unit_id: i32, spell: &spell::Spell, target_id: i32) -> f64 {
        let mut base = 1.5;
        let mut multi = 1.0;

        base*= self.unit(unit_id).spell_crit_dmg_base_multiplier(spell);

        if spell.is_proc {
            return base;
        }

        multi*= self.unit(unit_id).spell_crit_dmg_multiplier(spell);

        (base - 1.0) * multi + 1.0
    }

    /**
     * Source for resistance based mitigation
     * May not match exactly, but it is the best estimate we got
     * https://royalgiraffe.github.io/legacy-sim/
     */
    fn spell_dmg_resist(&mut self, unit_id: i32, instance: &spell::SpellInstance) -> f64 {
        if instance.spell.is_binary {
            return 0.0;
        }

        let mut resist_score: f64 = self.spell_resist_score(unit_id, &instance.spell, true);

        // Dots only use 10% of the resistance
        // But only if the dot has no initial damage (like fireball, pyroblast)
        if instance.spell.is_dot && (instance.spell.id != spell::FIREBALL_DOT || instance.spell.id != spell::PYROBLAST_DOT) {
            resist_score*= 0.1;
        }

        let cap: f64 = (self.unit(unit_id).level() as f64) * 5.0;
        let ratio: f64 = resist_score / cap;
        let i = (ratio * 3.0).floor() as usize;
        let fraction = ratio * 3.0 - i as f64;
        let mut percentages: [f64; 4] = [0.0; 4];
        let mut segments: [[f64; 4]; 4] = [[0.0; 4]; 4];
        segments[0] = [100.0, 0.0, 0.0, 0.0];
        segments[1] = [24.0, 55.0, 18.0, 3.0];
        segments[2] = [0.0, 22.0, 56.0, 22.0];
        segments[3] = [0.0, 4.0, 16.0, 80.0];


        if i >= 3 {
            percentages.copy_from_slice(&segments[3]);
        } else {
            for (j, percentage) in percentages.iter_mut().enumerate() {
                *percentage = (segments[i][j] * (1.0 - fraction) + segments[i+1][j] * fraction).round();
            }

            if ratio < 2.0/3.0 - 0.000001 {
                percentages[0] = percentages[0].max(1.0);
            }
        }

        let mut roll: f64 = self.rng.gen_range(0..=99) as f64;
        let mut resistance_multiplier = 0.0;

        for (n, percentage) in percentages.iter_mut().enumerate() {
            if roll < *percentage {
                resistance_multiplier = (n as f64) * 0.25;
                break;
            }

            roll-= *percentage;
        }

        if resistance_multiplier > 0.0 {
            (instance.dmg * resistance_multiplier).round()
        } else {
            0.0
        }
    }

    fn spell_resist_score(&mut self, unit_id: i32, spell: &spell::Spell, level_based: bool) -> f64 {
        let mut resist_score: f64 = (self.config.target_resistance as f64) - self.unit(unit_id).spell_penetration(spell.school);
        let unit_level = self.unit(unit_id).level();

        if self.config.curse_of_elements && (spell.school == common::School::Fire || spell.school == common::School::Frost) {
            if unit_level >= 56 {
                resist_score-= 75.0;
            } else if unit_level >= 44 {
                resist_score-= 60.0;
            } else if unit_level >= 32 {
                resist_score-= 45.0;
            }
        }

        if self.config.curse_of_elements && (spell.school == common::School::Arcane || spell.school == common::School::Shadow) {
            if unit_level >= 56 {
                resist_score-= 75.0;
            } else if unit_level >= 44 {
                resist_score-= 60.0;
            }
        }

        resist_score = resist_score.max(0.0);

        if level_based && self.config.target_level > unit_level {
            let diff: f64 = (self.config.target_level - unit_level) as f64;

            // Not sure what to do here, the same source disagree's with itself
            // The guide says the level-based resistance is based on attacker level
            // The calculator (by the same author) uses a fixed value of 8.0
            // Let's go with the written guide for now
            resist_score+= 2.0/15.0 * (unit_level as f64) * diff;
            // resist_score+= 8.0 * diff;
        }

        resist_score
    }

    pub fn log_push(&mut self, log: log::LogEntry) {
        if self.log_enabled {
            self.log.push(log);
        }
    }

    pub fn log(&mut self, log_type: log::LogType, text: String, unit_id: i32) {
        self.log_push(log::LogEntry {
            log_type,
            text,
            unit_name: self.unit(unit_id).name(),
            t: self.t,
            mana: self.unit(unit_id).current_mana(),
            mana_percent: self.unit(unit_id).mana_percent(),
            dmg: 0.0,
            resist: 0.0,
            spell_result: spell::SpellResult::None,
        });
    }

    pub fn log_spell_impact(&mut self, unit_id: i32, instance: &spell::SpellInstance, target_id: i32) {
        self.log_push(log::LogEntry {
            log_type: log::LogType::SpellImpact,
            text: format!("s[{}] -> t[{}]", instance.spell.name, self.target(target_id).name),
            unit_name: self.unit(unit_id).name(),
            t: self.t,
            mana: self.unit(unit_id).current_mana(),
            mana_percent: self.unit(unit_id).mana_percent(),
            dmg: instance.dmg,
            resist: instance.resist,
            spell_result: instance.result,
        });
    }

}